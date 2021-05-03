use actix_web::{App, HttpServer, HttpResponse, Result, web, get};
use sqlx::postgres::PgPoolOptions;
use serde::{Serialize, Deserialize};
use std::sync::{Mutex, Arc};

#[derive(Serialize, Deserialize)]
struct Message {
    message: String,
}

struct MainTable {
    id: i32,
    message: String,
}

struct AppData {
    pool: sqlx::Pool<sqlx::Postgres>,
}

#[get("/")]
async fn index(data: web::Data<Arc<Mutex<AppData>>>) -> Result<HttpResponse> {
    let data = data.lock().unwrap();

    let res = sqlx::query_file_as!(MainTable, "sql/getMessage.sql")
        .fetch_all(&data.pool)
        .await
        .unwrap();

    Ok(HttpResponse::Ok().json(Message {
        message: res[0].message.clone()
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
        .expect("DATABASE_URL must exist");

    // Create database pool
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&db_url[..])
        .await
        .expect("Failed to create database pool");

    // Initialize database
    sqlx::query_file!("sql/init.sql")
        .fetch_all(&pool)
        .await
        .expect("Failed to initialize the database");

    // Application data
    let app_data = Arc::new(Mutex::new(AppData { pool }));

    // Create HTTP server
    let server = HttpServer::new(move || {
            App::new()
                .data(app_data.clone())
                .service(index)
        })
        .bind(("0.0.0.0", port))?
        .run();

    println!("App running on port {}", port);

    server.await
}
