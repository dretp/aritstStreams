

use sqlx::PgPool;
use crate::models::User;


pub struct UserService;

impl UserService {

    pub async fn get_all_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
        let users = sqlx::query_as::<_, User>("SELECT * FROM \"user\" ORDER BY first_name")
            .fetch_all(pool)
            .await?;
        Ok(users)
    }
}
