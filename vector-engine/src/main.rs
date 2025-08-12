use actix_web::{post, web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

mod engine;
use engine::VectorEngine;

#[derive(Deserialize)]
struct SearchReq {
    vector: Vec<f32>,
    #[serde(default = "default_topk")]
    top_k: usize,
}
fn default_topk() -> usize { 5 }

#[derive(Serialize)]
struct SearchRes {
    id: usize,
    score: f32,
}

#[post("/search")]
async fn search(
    data: web::Data<Mutex<VectorEngine>>,
    req: web::Json<SearchReq>,
) -> HttpResponse {
    let engine = data.lock().unwrap();
    let results = engine.search(&req.vector, req.top_k);
    let body: Vec<SearchRes> = results
        .into_iter()
        .map(|(id, score)| SearchRes { id, score })
        .collect();
    HttpResponse::Ok().json(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Demo data
    let mut engine = VectorEngine::new(4);
    engine.add_vector(1, vec![0.1, 0.2, 0.3, 0.4]);
    engine.add_vector(2, vec![0.4, 0.3, 0.2, 0.1]);
    engine.add_vector(3, vec![0.0, 1.0, 0.0, 0.0]);

    let shared = web::Data::new(Mutex::new(engine));

    HttpServer::new(move || App::new().app_data(shared.clone()).service(search))
        .bind(("0.0.0.0", 9000))?
        .run()
        .await
}
