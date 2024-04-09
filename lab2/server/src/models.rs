pub mod user;
pub mod message;

pub type Pool = sqlx::Pool<sqlx::Sqlite>;
