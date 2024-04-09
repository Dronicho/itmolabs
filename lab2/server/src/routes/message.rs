use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

use crate::{app_state::AppState, models::{message::Message, user::User}};

#[derive(Deserialize)]
struct MessageForm {
    from: String,
    to: String,
    message: String,
}

async fn send_message(form: web::Json<MessageForm>, data: web::Data<AppState>) -> impl Responder {
    let sender = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = ?")
        .bind(&form.from)
        .fetch_optional(&data.db)
        .await
        .unwrap();

    if let Some(sender) = sender {
        sqlx::query("INSERT INTO messages (\"from\", \"to\", message) VALUES (?, ?, ?)")
            .bind(sender.email)
            .bind(&form.to)
            .bind(&form.message)
            .execute(&data.db)
            .await
            .unwrap();

        HttpResponse::Ok().body("Message sent successfully")
    } else {
        HttpResponse::NotFound().body("Recipient not found")
    }
}

async fn get_messages(data: web::Data<AppState>) -> impl Responder {
    let rows = sqlx::query_as::<_, Message>("SELECT * FROM messages")
        .fetch_all(&data.db)
        .await
        .unwrap();

    HttpResponse::Ok().json(rows)
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(web::post().to(send_message))
            .route(web::get().to(get_messages)),
    );
}
