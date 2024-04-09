use std::sync::Arc;

use actix_cors::Cors;
use actix_identity::IdentityMiddleware;
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{time::Duration, Key},
    web, App, HttpServer,
};
use anyhow::{Context, Ok};
use serde::Serialize;
use server::{app_state::AppState, broadcast::Broadcaster, routes, util::SECRET_KEY};
use sqlx::sqlite::SqlitePool;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug,actix_server=info,debug");
    pretty_env_logger::init();

    let database_url = dotenvy::var("DATABASE_URL")?;
    let db_pool: sqlx::Pool<sqlx::Sqlite> =
        SqlitePool::connect(database_url.as_str()).await.unwrap();

    info!("Starting migration");
    sqlx::migrate!()
        .run(&db_pool)
        .await
        .context("Error migrating")?;
    info!("Migration complete");

    let app_data = web::Data::new(AppState { db: db_pool });
    let broadcaster = Broadcaster::create();

    let _domain: String = std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_owned());

    let _ = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_method()
            .allow_any_origin()
            .supports_credentials();

        App::new()
            .wrap(IdentityMiddleware::default())
            .wrap(cors)
            .wrap(
                SessionMiddleware::builder(
                    CookieSessionStore::default(),
                    Key::from(SECRET_KEY.as_bytes()),
                )
                .session_lifecycle(PersistentSession::default().session_ttl(Duration::days(1)))
                .cookie_name("auth".to_owned())
                .cookie_domain(None)
                .cookie_path("/".to_owned())
                .build(),
            )
            .wrap(actix_web::middleware::Compress::default())
            .wrap(actix_web::middleware::NormalizePath::new(
                actix_web::middleware::TrailingSlash::Trim,
            ))
            .wrap(actix_web::middleware::Logger::default())
            .app_data(app_data.clone())
            .app_data(web::Data::from(Arc::clone(&broadcaster)))
            .service(web::scope("/api").configure(routes::configure))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await;

    Ok(())
}
