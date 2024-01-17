use sqlx::{FromRow, Row};
use crate::repos::table::Table;

#[derive(Debug, FromRow)]
pub struct Ping {}

impl<'c> Table<'c, Ping> {
    pub async fn ping(&self) -> Result<String, sqlx::Error> {
        let result = sqlx::query("select version();")
            .fetch_one(&*self.pool).await;
        match result {
            Ok(row) => Ok(row.get("version")),
            Err(e) => Err(e),
        }
    }
}