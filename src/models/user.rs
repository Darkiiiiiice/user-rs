use serde::Deserialize;
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize)]
pub(crate) struct User {
    pub id: i64,
    pub username: String,
    pub nickname: String,
}
