// src/core/mod.rs - Core system modules

pub mod errors;
pub mod metrics;

pub use errors::{FitnessError, Result};
pub use metrics::{MetricsCollector, SystemMetrics, OptimizationMetrics, NutritionMetrics};