use actix_cors::Cors;
use actix_web::{post, get, web, App, HttpServer, HttpResponse, Responder};
use awc::Client;
use serde::{Deserialize, Serialize};

#[get("/")]
async fn health() -> impl Responder {
    "Backend service running"
}

#[derive(Serialize, Deserialize)]
struct SearchReq {
    vector: Vec<f32>
}

#[derive(Serialize, Deserialize)]
struct SearchRes {
    id: usize,
    score: f32
}

#[post("/search")]
async fn search(req: web::Json<SearchReq>) -> HttpResponse {
    let client = Client::default();
    let response = client
        .post("http://vector-engine:9000/search")
        .send_json(&*req)
        .await;

    match response {
        Ok(mut res) => match res.body().await {
            Ok(bytes) => HttpResponse::Ok()
                .content_type("application/json")
                .body(bytes),
            Err(_) => HttpResponse::InternalServerError().finish(),
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .service(health)
            .service(search)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
