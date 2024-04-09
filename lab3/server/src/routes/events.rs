use super::auth::LoggedUser;
use crate::broadcast::Broadcaster;
use actix_web::{web, Responder};

async fn event_stream(broadcaster: web::Data<Broadcaster>, user: LoggedUser) -> impl Responder {
    broadcaster.new_client(user.email).await
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("").route(web::get().to(event_stream)));
}
