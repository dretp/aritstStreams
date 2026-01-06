use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;
use tokio;

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let _pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");
    let result = add(2, 3).await;

    testing_async_function().await;
    println!("Result: {}", result);
    
}

async fn testing_async_function() {
    // Simulate some async work
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    println!("Async function completed");
}



async fn add(a: i32, b: i32) -> i32 {
    a + b
}
