use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use dotenv::dotenv;
use std::env;


pub async fn create_pool() -> Result<PgPool, sqlx::Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in environment or .env file");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    Ok(pool)
}


pub async fn health_check(pool: &PgPool) -> bool {
    match sqlx::query("SELECT 1").execute(pool).await {
        Ok(_) => true,
        Err(_) => false,
    }
}

