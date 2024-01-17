use crate::models::user::User;
use crate::repos::table::Table;

impl<'c> Table<'c, User> {
    pub async fn add_user(&self, user: &User) -> Result<u64, sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO users (`id`, `name`, `email`)
            VALUES(?, ?, ?)"#,
        )
            .bind(&user.id)
            .bind(&user.username)
            .bind(&user.nickname)
            .execute(&*self.pool)
            .await
            .map(|x| x.rows_affected())
    }
}