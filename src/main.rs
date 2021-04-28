use actix_web::{HttpResponse, Result, get};
use serde::{Serialize, Deserialize};

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

    let server = HttpServer::new(|| App::new().service(index))
        .bind("localhost:3000")?
        .run();

    println!("Server running at http://localhost:3000/");

    server.await
}
