use actix_files::NamedFile;
use actix_web::web;
use std::path::PathBuf;

async fn download(path_str: web::Path<String>) -> actix_web::Result<NamedFile> {
    let path: PathBuf = format!("./tmp_out/{}", path_str).parse().unwrap();
    let file = NamedFile::open(path)?;
    let response = file.set_content_disposition(actix_web::http::header::ContentDisposition {
        disposition: actix_web::http::header::DispositionType::Attachment,
        parameters: vec![actix_web::http::header::DispositionParam::Filename(
            path_str.to_string(),
        )],
    });
    Ok(response)
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("{path}").route(web::get().to(download)));
}
