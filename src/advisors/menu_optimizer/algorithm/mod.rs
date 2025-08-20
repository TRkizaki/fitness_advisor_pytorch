// src/advisors/menu_optimizer/algorithm/mod.rs - Algorithm module

pub mod genetic;
pub mod types;

pub use genetic::GeneticAlgorithm;
pub use types::{OptimizationAlgorithm, AlgorithmFactory, GeneticAlgorithmWrapper};