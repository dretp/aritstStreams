mod db;
mod models;

mod video_test;
mod route;
mod controllers;

mod services;


use axum::Router;
use tokio;

#[tokio::main]
async fn main() {
    // Optional test pipeline
    video_test::start().await;

    // DB
    let pool = match db::create_pool().await {
        Ok(pool) => {
            println!("Database connection established successfully.");
            pool
        }
        Err(e) => {
            eprintln!("Failed to connect to database: {}", e);
            return;
        }
    };

    if !db::health_check(&pool).await {
        eprintln!("Database health check failed.");
        return;
    }

    println!("Database health check passed.");

    // Build router
    let app: Router = route::routes();

    // Start server
    let addr = "0.0.0.0:3000";
    println!("🚀 Server running on http://{}", addr);


}
