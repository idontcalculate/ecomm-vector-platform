use actix_web::{post, web, App, HttpServer, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Debug, Clone)]
struct VectorEngine {
    dim: usize,
    vectors: Vec<(usize, Vec<f32>)>, // (id, vector)
}

impl VectorEngine {
    fn new(dim: usize) -> Self {
        VectorEngine {
            dim,
            vectors: vec![],
        }
    }
    fn add_vector(&mut self, id: usize, vector: Vec<f32>) {
        if vector.len() == self.dim {
            self.vectors.push((id, vector));
        }
    }
    fn search(&self, query: Vec<f32>, top_k: usize) -> Vec<(usize, f32)> {
        let mut scored = self.vectors
            .iter()
            .map(|(id, v)| (*id, cosine_similarity(&query, v)))
            .collect::<Vec<_>>();
        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        scored.into_iter().take(top_k).collect()
    }
}

fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot = a.iter().zip(b).map(|(x, y)| x * y).sum::<f32>();
    let norm_a = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm_a > 0.0 && norm_b > 0.0 {
        dot / (norm_a * norm_b)
    } else {
        0.0
    }
}

#[derive(Deserialize)]
struct SearchReq {
    vector: Vec<f32>
}

#[derive(Serialize)]
struct SearchRes {
    id: usize,
    score: f32
}

#[post("/search")]
async fn search(
    data: web::Data<Mutex<VectorEngine>>,
    req: web::Json<SearchReq>
) -> HttpResponse {
    let engine = data.lock().unwrap();
    let results = engine.search(req.vector.clone(), 5);
    HttpResponse::Ok().json(
        results
            .into_iter()
            .map(|(id, score)| SearchRes { id, score })
            .collect::<Vec<_>>()
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut engine = VectorEngine::new(4);
    // Example vectors
    engine.add_vector(1, vec![0.1, 0.2, 0.3, 0.4]);
    engine.add_vector(2, vec![0.4, 0.3, 0.2, 0.1]);
    engine.add_vector(3, vec![0.0, 1.0, 0.0, 0.0]);
    let shared = web::Data::new(Mutex::new(engine));
    HttpServer::new(move || {
        App::new()
            .app_data(shared.clone())
            .service(search)
    })
    .bind(("0.0.0.0", 9000))?
    .run()
    .await
}
