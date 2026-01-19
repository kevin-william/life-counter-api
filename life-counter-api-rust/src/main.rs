mod db;
mod dto;
mod handlers;
mod models;
mod services;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenv::dotenv;
use std::env;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

fn run_migrations(pool: &db::DbPool) {
    let mut conn = pool.get().expect("Failed to get db connection");
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");
    log::info!("Database migrations applied successfully");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env
    dotenv().ok();

    // Initialize logger
    env_logger::init();

    // Get configuration from environment
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let server_address = format!("{}:{}", host, port);

    log::info!("Starting Life Counter API...");
    log::info!("Database URL: {}", env::var("DATABASE_URL").unwrap_or_else(|_| "Not set".to_string()));

    // Create database connection pool
    let pool = db::establish_connection_pool();

    // Run migrations
    run_migrations(&pool);

    log::info!("Starting server at http://{}", server_address);

    // Start HTTP server
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::default())
            .wrap(cors)
            .configure(handlers::contadores::configure)
            .configure(handlers::utilizadores::configure)
            .configure(handlers::contador_utilizadores::configure)
    })
    .bind(&server_address)?
    .run()
    .await
}

