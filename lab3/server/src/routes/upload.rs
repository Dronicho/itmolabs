use crate::{broadcast::Broadcaster, encode::encode_video, models::upload::UploadPayload};
use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{web, HttpResponse, Responder};
use log::info;
use uuid::Uuid;

use super::auth::LoggedUser;

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(rename = "file")]
    file: TempFile,
}

async fn upload_video(
    MultipartForm(form): MultipartForm<UploadForm>,
    broadcaster: web::Data<Broadcaster>,
    logged_user: LoggedUser,
) -> impl Responder {
    info!("{}", logged_user.email);

    let f = form.file;
    let file_name = f.file_name.unwrap();
    let input_file: String = format!("./tmp/{}", file_name);
    f.file.persist(input_file.clone()).unwrap();
    let output_name = Uuid::new_v4();
    let output_file = format!("./tmp_out/{}.webm", output_name);
    broadcaster
        .send_to(UploadPayload::started(), logged_user.email.clone())
        .await;
    let _encode_task = tokio::spawn(async move {
        match encode_video(&input_file.clone(), &output_file).await {
            Ok(_) => {
                broadcaster
                    .send_to(
                        UploadPayload::completed(format!(
                            "http://localhost:8080/api/download/{}.webm",
                            output_name
                        )),
                        logged_user.email.clone(),
                    )
                    .await
            }
            Err(_) => {
                broadcaster
                    .send_to(UploadPayload::error(), logged_user.email.clone())
                    .await
            }
        }
    });

    HttpResponse::Ok().body("Video upload and encoding started")
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("").route(web::post().to(upload_video)));
}
