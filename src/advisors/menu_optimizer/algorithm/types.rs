// src/advisors/menu_optimizer/algorithm/types.rs - Algorithm-specific types

use crate::models::optimization::*;
use crate::core::Result;
use async_trait::async_trait;

/// Trait for optimization algorithms
#[async_trait]
pub trait OptimizationAlgorithm: Send + Sync {
    /// Execute the optimization algorithm
    async fn optimize(&mut self, request: &OptimizationRequest) -> Result<OptimizationSolution>;
    
    /// Get algorithm-specific configuration
    fn get_config(&self) -> &AlgorithmConfig;
    
    /// Get algorithm name for logging/metrics
    fn get_name(&self) -> &'static str;
    
    /// Check if algorithm can handle the given request
    fn can_handle(&self, request: &OptimizationRequest) -> bool;
    
    /// Get estimated runtime for the request
    fn estimate_runtime(&self, request: &OptimizationRequest) -> std::time::Duration;
}

/// Algorithm factory for creating optimization algorithms
pub struct AlgorithmFactory;

impl AlgorithmFactory {
    /// Create an algorithm instance based on type
    pub fn create_algorithm(
        algorithm_type: &AlgorithmType,
        config: AlgorithmConfig,
        recipes: Vec<crate::models::food::Recipe>,
        foods: std::collections::HashMap<String, crate::models::food::Food>,
    ) -> Result<Box<dyn OptimizationAlgorithm>> {
        match algorithm_type {
            AlgorithmType::GeneticAlgorithm => {
                let ga = crate::advisors::menu_optimizer::algorithm::genetic::GeneticAlgorithm::new(
                    config, recipes, foods, None
                );
                Ok(Box::new(GeneticAlgorithmWrapper::new(ga)))
            }
            AlgorithmType::ParticleSwarmOptimization => {
                // TODO: Implement PSO
                Err(crate::core::FitnessError::optimization("PSO not yet implemented"))
            }
            AlgorithmType::NSGA2 => {
                // TODO: Implement NSGA-II
                Err(crate::core::FitnessError::optimization("NSGA-II not yet implemented"))
            }
            AlgorithmType::SimulatedAnnealing => {
                // TODO: Implement SA
                Err(crate::core::FitnessError::optimization("Simulated Annealing not yet implemented"))
            }
            AlgorithmType::Hybrid => {
                // TODO: Implement hybrid approach
                Err(crate::core::FitnessError::optimization("Hybrid algorithm not yet implemented"))
            }
        }
    }
    
    /// Get recommended algorithm for request
    pub fn recommend_algorithm(request: &OptimizationRequest) -> AlgorithmType {
        // Algorithm selection logic based on request characteristics
        let objectives_count = request.objectives.len();
        let time_horizon = request.time_horizon_days;
        let complexity = objectives_count * time_horizon as usize;
        
        match complexity {
            0..=50 => AlgorithmType::GeneticAlgorithm,
            51..=200 => AlgorithmType::GeneticAlgorithm, // Could be PSO in future
            _ => AlgorithmType::GeneticAlgorithm, // Could be NSGA-II for very complex problems
        }
    }
}

/// Wrapper for genetic algorithm to implement the trait
pub struct GeneticAlgorithmWrapper {
    config: AlgorithmConfig,
    recipes: Vec<crate::models::food::Recipe>,
    foods: std::collections::HashMap<String, crate::models::food::Food>,
}

impl GeneticAlgorithmWrapper {
    pub fn new(algorithm: crate::advisors::menu_optimizer::algorithm::genetic::GeneticAlgorithm) -> Self {
        Self { 
            config: algorithm.config.clone(),
            recipes: algorithm.recipes.clone(),
            foods: algorithm.foods.clone(),
        }
    }
}

#[async_trait]
impl OptimizationAlgorithm for GeneticAlgorithmWrapper {
    async fn optimize(&mut self, request: &OptimizationRequest) -> Result<OptimizationSolution> {
        // Clone data for the blocking task
        let config = self.config.clone();
        let recipes = self.recipes.clone();
        let foods = self.foods.clone();
        let request = request.clone();
        
        // Run optimization in a blocking task to avoid blocking the async runtime
        let result = tokio::task::spawn_blocking(move || {
            let mut algorithm = crate::advisors::menu_optimizer::algorithm::genetic::GeneticAlgorithm::new(
                config, recipes, foods, None
            );
            algorithm.optimize(&request)
        }).await;
        
        match result {
            Ok(optimization_result) => optimization_result,
            Err(e) => Err(crate::core::FitnessError::optimization(format!("Task join error: {}", e))),
        }
    }
    
    fn get_config(&self) -> &AlgorithmConfig {
        &self.config
    }
    
    fn get_name(&self) -> &'static str {
        "Genetic Algorithm"
    }
    
    fn can_handle(&self, request: &OptimizationRequest) -> bool {
        // GA can handle most requests
        request.time_horizon_days <= 30 && 
        request.objectives.len() <= 8 &&
        !request.objectives.is_empty()
    }
    
    fn estimate_runtime(&self, request: &OptimizationRequest) -> std::time::Duration {
        let base_time = 30; // 30 seconds base
        let complexity_factor = request.objectives.len() * request.time_horizon_days as usize;
        let estimated_seconds = base_time + (complexity_factor / 10);
        
        std::time::Duration::from_secs(estimated_seconds.min(300) as u64) // Max 5 minutes
    }
}