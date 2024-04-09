use crate::app_state::AppState;
use crate::errors::ServiceError;
use crate::models::user::{SlimUser, User};
use crate::util::{hash_password, verify_password};
use actix_identity::Identity;
use actix_web::dev::Payload;
use actix_web::{web, FromRequest, HttpMessage, HttpRequest, HttpResponse};
use log::{debug, error};
use serde::Deserialize;
use sqlx::SqlitePool;
use std::future::{ready, Ready};

#[derive(Deserialize)]
struct RegisterForm {
    email: String,
    password: String,
    name: String,
}

async fn register(
    req: HttpRequest,
    form: web::Json<RegisterForm>,
    data: web::Data<AppState>,
) -> anyhow::Result<HttpResponse, actix_web::Error> {
    let password_hash = hash_password(&form.password);

    let user = User {
        id: None,
        email: form.email.clone(),
        password: password_hash,
        name: form.name.clone(),
    };

    sqlx::query!(
        "INSERT INTO users (email, password, name) VALUES ($1, $2, $3)",
        user.email,
        user.password,
        user.name
    )
    .execute(&data.db)
    .await
    .unwrap();

    let user_string = serde_json::to_string(&SlimUser::from(user)).unwrap();
    Identity::login(&req.extensions(), user_string)?;

    Ok(HttpResponse::NoContent().finish())
}

pub type LoggedUser = SlimUser;

impl FromRequest for LoggedUser {
    type Error = actix_web::Error;
    type Future = Ready<Result<LoggedUser, actix_web::Error>>;

    fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
        match Identity::from_request(req, pl).into_inner() {
            Ok(identity) => {
                if let Ok(user_json) = identity.id() {
                    if let Ok(user) = serde_json::from_str(&user_json) {
                        return ready(Ok(user));
                    }
                }
            }
            Err(err) => {
                error!("{}", err);
                return ready(Err(ServiceError::Unauthorized.into()));
            }
        }
        ready(Err(ServiceError::Unauthorized.into()))
    }
}

pub async fn logout(id: Identity) -> HttpResponse {
    id.logout();
    HttpResponse::NoContent().finish()
}

#[derive(Deserialize)]
struct LoginForm {
    email: String,
    password: String,
}

async fn login(
    req: HttpRequest,
    form: web::Json<LoginForm>,
    data: web::Data<AppState>,
) -> anyhow::Result<HttpResponse, actix_web::Error> {
    let user = get_user(form.into_inner(), &data.db).await?;

    let user_string = serde_json::to_string(&user).unwrap();
    Identity::login(&req.extensions(), user_string)?;

    Ok(HttpResponse::NoContent().finish())
}

async fn get_me(id: LoggedUser) -> HttpResponse {
    HttpResponse::Ok().json(id)
}

async fn get_user(form: LoginForm, pool: &SqlitePool) -> Result<SlimUser, ServiceError> {
    let user_result = sqlx::query_as!(User, "SELECT * FROM users WHERE email=?", form.email)
        .fetch_one(pool)
        .await;

    match user_result {
        Ok(user) => {
            debug!("User found: {:?}", user);
            if verify_password(&form.password, &user.password) {
                return Ok(user.into());
            }
            return Err(ServiceError::Unauthorized);
        }
        Err(_) => Err(ServiceError::InternalServerError),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/login").route(web::post().to(login)))
        .service(web::resource("/register").route(web::post().to(register)))
        .service(web::resource("/me").route(web::get().to(get_me)))
        .service(web::resource("/logout").route(web::post().to(logout)));
}
