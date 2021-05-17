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
                .service(routes::user_routes::get_user_info)
                .service(routes::user_routes::set_username)
                .service(routes::user_routes::set_password)
                .service(routes::poll_routes::create_poll)
                .service(routes::poll_routes::get_poll_info)
                .service(routes::poll_routes::set_poll_title)
                .service(routes::poll_routes::set_poll_description)
                .service(routes::poll_routes::delete_poll)
                .service(routes::poll_option_routes::create_poll_option)
                .service(routes::poll_option_routes::get_poll_option_info)
                .service(routes::poll_option_routes::set_poll_option_value)
                .service(routes::poll_option_routes::get_poll_option_poll)
                .service(routes::poll_option_routes::delete_poll_option)
                .service(routes::poll_vote_routes::poll_vote)
                .service(routes::poll_vote_routes::poll_unvote)
                .service(routes::poll_vote_routes::get_poll_vote_poll)
                .default_service(web::route().to(not_found))
        })
        .bind(("0.0.0.0", port))?
        .run();

    println!("App running on port {}", port);

    server.await
}
