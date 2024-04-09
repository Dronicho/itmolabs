use actix_web::web;
pub mod auth;
pub mod message;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth").configure(auth::configure));
    cfg.service(web::scope("/messages").configure(message::configure));
}
