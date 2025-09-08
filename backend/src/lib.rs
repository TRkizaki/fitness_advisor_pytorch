// src/lib.rs - Backend library exports

pub mod database;
pub mod ml_client;
pub mod config;
pub mod core;
pub mod models;
pub mod advisors;
pub mod sample_data;
pub mod rag;
// pub mod api;
// pub mod ai_analytics;
// pub mod websocket;

// Re-export commonly used types
pub use models::*;
pub use rag::*;