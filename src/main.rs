mod db;
mod models;
mod services;

use tokio;

#[tokio::main]
async fn main() {
    // Establish database connection pool˚†
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

    // Perform a health check
    if db::health_check(&pool).await {
        println!("Database health check passed.");
    } else {
        eprintln!("Database health check failed.");
        return;
    }

    // Example: Fetch all users (assuming the table exists)
    match services::UserService::get_all_users(&pool).await {
        Ok(users) => {
            println!("Found {} users.", users.len());
            for user in users {
                println!("- {} {} (ID: {})", user.first_name, user.last_name, user.public_id);
            }
        }
        Err(e) => {
            eprintln!("Failed to fetch users: {}", e);
        }
    }
}
