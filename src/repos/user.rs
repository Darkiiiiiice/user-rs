use crate::models::user::User;
use crate::repos::table::Table;


impl<'c> Table<'c, User> {
    pub async fn create_common_user(&self, user: User) -> Result<i64, sqlx::Error> {
        let result = sqlx::query_file!(
            "sql_scripts/users/insert_user.sql",
            &user.username,
            &user.email,
            &user.phone,
            &user.created_by,
            &user.updated_by,
            user.created_at.naive_utc(),
           user.updated_at.naive_utc(),
        )
            .fetch_one(&*self.pool)
            .await?;

        Ok(result.id)
    }
}