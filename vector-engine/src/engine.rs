use hnsw_rs::prelude::*;

/// Wrapper around an HNSW index for vector search.
pub struct VectorEngine {
    index: Hnsw<f32, DistCosine>,
    dim: usize,
}

impl VectorEngine {
    /// Create a new HNSW index with the given dimensionality.
    pub fn new(dim: usize) -> Self {
        // Parameters: max_nb_conn, ef_construction, max_elements, distance metric
        let index = Hnsw::new(16, dim, 200, 16, DistCosine {});

        Self { index, dim }
    }

    /// Add a vector to the index with an ID.
    pub fn add_vector(&mut self, id: usize, vector: Vec<f32>) {
        if vector.len() == self.dim {
            self.index.insert((&vector, id));
        }
    }

    /// Search for the top_k nearest neighbors.
    pub fn search(&self, query: Vec<f32>, top_k: usize) -> Vec<(usize, f32)> {
        if query.len() != self.dim {
            return vec![];
        }

        let ef_search = 16;
        let results = self.index.search(&query, top_k, ef_search);

        results
            .into_iter()
            .map(|n| (n.d_id, n.distance))
            .collect()
    }
}
