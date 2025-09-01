// Main Entry point for Backend App

// 
use actix_web::{App, HttpServer, middleware};
use dotenvy::dotenv;
use std::env;

// 
mod config;
mod database;
mod models;
mod handlers;
mod routes;

// 
use crate::config::Config;
use crate::database::db::create_pool;
use crate::routes::configure;

// 
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load .env variables
    dotenv().ok();

    // Initialize logging to show requests in terminal
    env_logger::init();

    // load App configuration
    let cfg = Config::from_env();

    // Create database connection pool
    let pool = create_pool(&cfg);

    // Build Server Address String
    let bind_addr = format!("{}:{}", cfg.host, cfg.port);

    // Start Actix-Web Server
    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .configure(configure)
    })
    .bind(bind_addr)?   // Bind to host:port
    .run()
    .await
}