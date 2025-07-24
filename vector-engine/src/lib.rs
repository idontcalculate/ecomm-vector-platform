use hnsw_rs::prelude::*;

pub struct VectorEngine {
    index: Hnsw<'static, f32, DistCosine>,
}

impl VectorEngine {
    pub fn new(dim: usize) -> Self {
        VectorEngine {
            index: Hnsw::new(16, dim, 32, 200, DistCosine {}),
        }
    }

    pub fn add_vector(&mut self, id: usize, vec: Vec<f32>) {
        self.index.insert((&vec[..], id));
    }

    pub fn search(&self, vec: Vec<f32>, k: usize) -> Vec<(usize, f32)> {
        self.index
            .search(&vec, k, 200)
            .into_iter()
            .map(|n| (n.d_id, n.distance))
            .collect()
    }
}
