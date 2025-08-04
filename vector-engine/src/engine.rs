use faiss::{index_factory, Index, MetricType};

pub struct VectorEngine {
    index: faiss::IndexImpl,
    dimension: usize,
}

impl VectorEngine {
    /// Create a new vector search engine with specified dimension
    pub fn new(dimension: usize) -> Self {
        let index = index_factory(dimension as u32, "Flat", MetricType::L2)
            .expect("Failed to initialize FAISS index");

        Self {
            index,
            dimension,
        }
    }

    /// Add vectors to the FAISS index
    pub fn add_vectors(&mut self, vectors: &[Vec<f32>]) {
        let flattened: Vec<f32> = vectors.iter().flatten().cloned().collect();
        self.index
            .add(&flattened)
            .expect("Failed to add vectors to FAISS index");
    }

    /// Search for nearest neighbors
    pub fn search(&self, query: &[f32], k: usize) -> Vec<(usize, f32)> {
        let results = self
            .index
            .search(query, k)
            .expect("Failed to search FAISS index");

        results.labels
            .iter()
            .zip(results.distances.iter())
            .map(|(&id, &score)| (id as usize, score))
            .collect()
    }
}
