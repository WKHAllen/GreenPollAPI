use actix_web::{App, HttpServer, HttpResponse, Result, web, get};
use sqlx::postgres::PgPoolOptions;
use serde::{Serialize, Deserialize};
use std::sync::{Mutex, Arc};

mod dbinit;
mod services;

#[derive(Serialize, Deserialize)]
struct ErrorJSON {
    error: String,
}

#[derive(Serialize, Deserialize)]
struct UserQuery {
    user_id: i32,
}

#[derive(Serialize, Deserialize)]
struct UserJSON {
    id: i32,
    email: String,
    verified: bool,
    join_time: i64,
}

struct AppData {
    pool: sqlx::Pool<sqlx::Postgres>,
}

#[get("/")]
async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json("Hello, world!"))
}

#[get("/get_user")]
async fn get_user(app_data: web::Data<Arc<Mutex<AppData>>>, query: web::Query<UserQuery>) -> Result<HttpResponse> {
    let data = app_data.lock().unwrap();

    match services::user_service::get_user(&data.pool, query.user_id).await {
        Ok(user) => Ok(HttpResponse::Ok().json(UserJSON {
            id: user.id,
            email: user.email,
            verified: user.verified,
            join_time: user.join_time.timestamp()
        })),
        Err(e) => Ok(HttpResponse::Ok().json(ErrorJSON {
            error: format!("{}", e)
        })),
    }
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
                .service(get_user)
        })
        .bind(("0.0.0.0", port))?
        .run();

    println!("App running on port {}", port);

    server.await
}
