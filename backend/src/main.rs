use actix_web::{get, App, HttpServer, Responder};

#[get("/")]
async fn health() -> impl Responder {
    "Backend service running"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(health))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
