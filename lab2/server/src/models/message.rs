use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Message {
    id: i64,
    message: String,
    from: String,
    to: String,
}