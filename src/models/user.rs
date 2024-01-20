use chrono::{DateTime, Utc};
use serde::Deserialize;
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize)]
pub(crate) struct User {
    pub id: i64,
    pub username: String,
    pub status: bool,
    pub root: bool,
    pub email: String,
    pub phone: String,
    pub created_by: i64,
    pub updated_by: i64,
    pub deleted_by: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl User {
    pub(crate) fn new(username: &String, email: &String, phone: &String) -> Self {
        User {
            id: 0,
            username: username.clone(),
            email: email.clone(),
            phone: phone.clone(),
            status: false,
            root: false,
            created_by: 0,
            updated_by: 0,
            deleted_by: 0,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            deleted_at: None,
        }
    }
}