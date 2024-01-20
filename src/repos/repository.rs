use std::sync::Arc;
use std::time::Duration;
use sqlx::postgres::PgConnectOptions;
use crate::models::user::User;
use crate::repos::ping::Ping;
use crate::repos::table::Table;

#[derive(Debug, Clone)]
pub struct Repository<'a> {
    pub users: Arc<Table<'a, User>>,
    pub ping: Arc<Table<'a, Ping>>,
}

impl<'a> Repository<'a> {
    pub async fn new() -> Repository<'a> {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .max_lifetime(Duration::new(15, 0))
            .idle_timeout(Duration::new(60, 0))
            .connect_with(PgConnectOptions::new()

            )
            .await.unwrap();

        Repository {
            users: Arc::new(Table::new(Arc::new(pool.clone()))),
            ping: Arc::new(Table::new(Arc::new(pool.clone()))),
        }
    }
}

