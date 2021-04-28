use actix_web::{HttpResponse, Result, get};
use serde::{Serialize, Deserialize};

mod db;

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
    use actix_web::{App, HttpServer};

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    let server = HttpServer::new(|| App::new().service(index))
        .bind(("0.0.0.0", port))?
        .run();

    println!("App running on port {}", port);

    server.await
}
