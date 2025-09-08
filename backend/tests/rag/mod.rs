// RAG system test modules

pub mod test_document_processor;
pub mod test_embeddings;
pub mod test_integration;
pub mod test_api;
pub mod sample_data;

// Re-export for convenience
pub use sample_data::FitnessSampleData;