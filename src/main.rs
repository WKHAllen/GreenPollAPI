use actix_web::{App, HttpServer, HttpResponse, Result, web, get};
use sqlx::postgres::PgPoolOptions;
use std::sync::{Mutex, Arc};

mod util;
mod dbinit;
mod routes;
mod services;

use util::AppData;

#[get("/")]
async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json("Hello, world!"))
}

async fn not_found() -> Result<HttpResponse> {
    Ok(HttpResponse::NotFound().json("404 not found"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Get port
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    // Get database URL
    let db_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must exist");

    // Create database pool
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&db_url[..])
        .await
        .expect("Failed to create database pool");

    // Initialize database
    dbinit::init_db(&pool)
        .await
        .expect("Failed to initialize database");

    // Application data
    let app_data = Arc::new(Mutex::new(AppData { pool }));

    // Create HTTP server
    let server = HttpServer::new(move || {
            App::new()
                .data(app_data.clone())
                .service(index)
                .service(routes::user_routes::get_user)
                .default_service(web::route().to(not_found))
        })
        .bind(("0.0.0.0", port))?
        .run();

    println!("App running on port {}", port);

    server.await
}
