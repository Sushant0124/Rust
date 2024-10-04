use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use dotenv::dotenv; // For loading environment variables
use crate::auth::{signup, login, verify_email}; // Import your handlers

mod auth;
mod models;
mod db;
mod email;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load environment variables from .env

    // Set up the PostgreSQL connection pool
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await.expect("Failed to connect to the database");

    // Start the Actix Web server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // Share database connection pool
            .service(signup)
            .service(login)
            .service(verify_email)
    })
    .bind(("127.0.0.1", 8080))? // Bind to localhost:8080
    .run()
    .await
}
