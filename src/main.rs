use actix_web::{App, HttpServer, HttpResponse, Result, get};
use serde::{Serialize, Deserialize};
use sqlx::postgres::PgPoolOptions;

#[derive(Serialize, Deserialize)]
struct Message {
    message: String,
}

#[get("/")]
async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(Message {
        message: String::from("Hello, world!")
    }))
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
        .unwrap();

    // Create database pool
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&db_url[..])
        .await
        .unwrap();

    // Initialize database
    sqlx::query_file!("sql/init.sql")
        .fetch_all(&pool)
        .await
        .unwrap();

    // Create HTTP server
    let server = HttpServer::new(|| App::new().service(index))
        .bind(("0.0.0.0", port))?
        .run();

    println!("App running on port {}", port);

    server.await
}
