use actix_web::web;
pub mod auth;
pub mod download;
pub mod events;
pub mod upload;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth").configure(auth::configure));
    cfg.service(web::scope("/upload").configure(upload::configure));
    cfg.service(web::scope("/download").configure(download::configure));
    cfg.service(web::scope("/events").configure(events::configure));
}
