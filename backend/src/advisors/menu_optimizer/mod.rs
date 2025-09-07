// src/advisors/menu_optimizer/mod.rs - Menu optimization service

pub mod algorithm;
pub mod data_loader;

use crate::core::{FitnessError, Result, MetricsCollector, OptimizationMetrics};
use crate::models::{optimization::*, food::*};
use algorithm::{AlgorithmFactory, OptimizationAlgorithm};
pub use data_loader::DataLoader;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error};
use std::time::Instant;

/// Main menu optimization service
pub struct MenuOptimizer {
    recipes: Arc<RwLock<Vec<Recipe>>>,
    foods: Arc<RwLock<HashMap<String, Food>>>,
    metrics: Arc<RwLock<MetricsCollector>>,
    cache: Arc<RwLock<HashMap<String, OptimizationSolution>>>,
    default_config: AlgorithmConfig,
}

impl MenuOptimizer {
    /// Create a new menu optimizer
    pub fn new() -> Self {
        Self {
            recipes: Arc::new(RwLock::new(Vec::new())),
            foods: Arc::new(RwLock::new(HashMap::new())),
            metrics: Arc::new(RwLock::new(MetricsCollector::new())),
            cache: Arc::new(RwLock::new(HashMap::new())),
            default_config: AlgorithmConfig::default(),
        }
    }

    /// Create with initial data
    pub fn with_data(recipes: Vec<Recipe>, foods: HashMap<String, Food>) -> Self {
        Self {
            recipes: Arc::new(RwLock::new(recipes)),
            foods: Arc::new(RwLock::new(foods)),
            metrics: Arc::new(RwLock::new(MetricsCollector::new())),
            cache: Arc::new(RwLock::new(HashMap::new())),
            default_config: AlgorithmConfig::default(),
        }
    }

    /// Add recipes to the optimizer
    pub async fn add_recipes(&self, mut new_recipes: Vec<Recipe>) -> Result<()> {
        let mut recipes = self.recipes.write().await;
        recipes.append(&mut new_recipes);
        info!("Added {} recipes to menu optimizer", new_recipes.len());
        Ok(())
    }

    /// Add foods to the optimizer
    pub async fn add_foods(&self, new_foods: HashMap<String, Food>) -> Result<()> {
        let mut foods = self.foods.write().await;
        let added_count = new_foods.len();
        foods.extend(new_foods);
        info!("Added {} foods to menu optimizer", added_count);
        Ok(())
    }

    /// Get recipe count
    pub async fn get_recipe_count(&self) -> usize {
        self.recipes.read().await.len()
    }

    /// Get food count
    pub async fn get_food_count(&self) -> usize {
        self.foods.read().await.len()
    }

    /// Optimize meal plan
    pub async fn optimize_meal_plan(&self, request: OptimizationRequest) -> Result<OptimizationSolution> {
        let start_time = Instant::now();
        
        // Update metrics
        {
            let mut metrics = self.metrics.write().await;
            metrics.record_optimization_start();
        }

        // Validate request
        request.validate()
            .map_err(|e| FitnessError::optimization(format!("Invalid optimization request: {}", e)))?;

        // Check cache first
        let cache_key = self.generate_cache_key(&request);
        if let Some(cached_solution) = self.check_cache(&cache_key).await? {
            info!("Returning cached optimization solution for user {}", request.user_id);
            {
                let mut metrics = self.metrics.write().await;
                metrics.record_cache_hit();
            }
            return Ok(cached_solution);
        }

        {
            let mut metrics = self.metrics.write().await;
            metrics.record_cache_miss();
        }

        // Get algorithm recommendation
        let algorithm_type = AlgorithmFactory::recommend_algorithm(&request);
        info!("Using algorithm: {:?} for optimization request from user {}", 
              algorithm_type, request.user_id);

        // Create algorithm instance
        let config = request.algorithm_config.clone();
        let recipes = self.recipes.read().await.clone();
        let foods = self.foods.read().await.clone();

        if recipes.is_empty() {
            return Err(FitnessError::optimization("No recipes available for optimization".to_string()));
        }

        if foods.is_empty() {
            return Err(FitnessError::optimization("No foods available for optimization".to_string()));
        }

        let mut algorithm = AlgorithmFactory::create_algorithm(&algorithm_type, config, recipes, foods)?;

        // Check if algorithm can handle the request
        if !algorithm.can_handle(&request) {
            return Err(FitnessError::optimization(
                format!("Algorithm {:?} cannot handle this optimization request", algorithm_type)
            ));
        }

        // Log estimated runtime
        let estimated_runtime = algorithm.estimate_runtime(&request);
        info!("Estimated optimization runtime: {:?} for user {}", 
              estimated_runtime, request.user_id);

        // Run optimization
        let solution = match algorithm.optimize(&request).await {
            Ok(solution) => {
                let duration = start_time.elapsed();
                info!("Optimization completed successfully for user {} in {:?}", 
                      request.user_id, duration);

                // Record success metrics
                {
                    let mut metrics = self.metrics.write().await;
                    let opt_metrics = OptimizationMetrics {
                        algorithm_type: algorithm.get_name().to_string(),
                        execution_time_ms: duration.as_millis() as f64,
                        iterations: solution.algorithm_metadata.generations_run as u32,
                        convergence_score: solution.fitness_score,
                        constraint_violations: solution.constraint_violations.len() as u32,
                        solution_quality: solution.fitness_score,
                    };
                    metrics.record_optimization_success(duration, opt_metrics);
                }

                // Cache the solution
                self.cache_solution(cache_key, solution.clone()).await?;

                solution
            }
            Err(e) => {
                error!("Optimization failed for user {}: {}", request.user_id, e);
                
                // Record failure metrics
                {
                    let mut metrics = self.metrics.write().await;
                    metrics.record_optimization_failure();
                }

                return Err(e);
            }
        };

        Ok(solution)
    }

    /// Generate personalized nutrition constraints for a user
    pub async fn generate_nutrition_constraints(
        &self,
        user: &crate::User,
        goals: &[crate::FitnessGoal],
    ) -> Result<NutritionConstraints> {
        // Calculate BMR using Mifflin-St Jeor equation
        let bmr = if user.name.contains("female") { // Simple heuristic - in real app this would be a user field
            655.1 + (9.563 * user.weight as f64) + (1.850 * user.height as f64) - (4.676 * user.age as f64)
        } else {
            88.362 + (13.397 * user.weight as f64) + (4.799 * user.height as f64) - (5.677 * user.age as f64)
        };

        // Activity multiplier based on fitness level
        let activity_multiplier = match user.fitness_level {
            crate::FitnessLevel::Beginner => 1.2,
            crate::FitnessLevel::Intermediate => 1.5,
            crate::FitnessLevel::Advanced => 1.7,
            crate::FitnessLevel::Elite => 1.9,
        };

        let tdee = bmr * activity_multiplier;

        // Adjust calories based on goals
        let target_calories = if goals.contains(&crate::FitnessGoal::WeightLoss) {
            tdee * 0.8 // 20% deficit
        } else if goals.contains(&crate::FitnessGoal::MuscleGain) {
            tdee * 1.1 // 10% surplus
        } else {
            tdee // Maintenance
        };

        // Calculate macro ranges
        let protein_g = if goals.contains(&crate::FitnessGoal::MuscleGain) {
            user.weight as f64 * 2.2 // 2.2g per kg for muscle gain
        } else {
            user.weight as f64 * 1.6 // 1.6g per kg for general health
        };

        let fat_calories = target_calories * 0.25; // 25% of calories from fat
        let fat_g = fat_calories / 9.0;

        let protein_calories = protein_g * 4.0;
        let remaining_calories = target_calories - protein_calories - fat_calories;
        let carbs_g = remaining_calories / 4.0;

        Ok(NutritionConstraints {
            daily_calories: CalorieRange {
                min: target_calories * 0.9,
                max: target_calories * 1.1,
                target: target_calories,
            },
            macros: MacroConstraints {
                protein_g: Range::new(protein_g * 0.8, protein_g * 1.2),
                carbs_g: Range::new(carbs_g * 0.7, carbs_g * 1.3),
                fat_g: Range::new(fat_g * 0.8, fat_g * 1.2),
                fiber_g: Range::new(25.0, 40.0),
                sugar_g_max: Some(50.0),
                sodium_mg_max: Some(2300.0),
            },
            micronutrients: MicronutrientConstraints {
                vitamin_c_mg: Range::new(65.0, 2000.0),
                calcium_mg: Range::new(1000.0, 2500.0),
                iron_mg: Range::new(8.0, 45.0),
                vitamin_d_iu: Range::new(600.0, 4000.0),
                vitamin_b12_mcg: Range::new(2.4, 100.0),
                folate_mcg: Range::new(400.0, 1000.0),
                omega3_g: Range::new(1.1, 3.0),
            },
            meal_count_per_day: MealCountConstraints {
                breakfast: 1,
                lunch: 1,
                dinner: 1,
                snacks: 2,
            },
            budget_per_day: None, // Can be set based on user preferences
            preparation_time_max_minutes: Some(120), // 2 hours max per day
        })
    }

    /// Get system metrics
    pub async fn get_metrics(&self) -> crate::core::SystemMetrics {
        self.metrics.read().await.get_current_metrics()
    }

    /// Generate cache key for optimization request
    fn generate_cache_key(&self, request: &OptimizationRequest) -> String {
        use std::hash::{Hash, Hasher};
        use std::collections::hash_map::DefaultHasher;

        let mut hasher = DefaultHasher::new();
        
        // Hash key components of the request
        request.user_id.hash(&mut hasher);
        request.time_horizon_days.hash(&mut hasher);
        request.constraints.daily_calories.target.to_bits().hash(&mut hasher);
        request.objectives.len().hash(&mut hasher);
        
        format!("opt_{:x}", hasher.finish())
    }

    /// Check optimization cache
    async fn check_cache(&self, key: &str) -> Result<Option<OptimizationSolution>> {
        let cache = self.cache.read().await;
        Ok(cache.get(key).cloned())
    }

    /// Cache optimization solution
    async fn cache_solution(&self, key: String, solution: OptimizationSolution) -> Result<()> {
        let mut cache = self.cache.write().await;
        
        // Simple cache size management
        if cache.len() > 1000 {
            // Remove oldest entries (simplified - in production use LRU)
            let keys_to_remove: Vec<_> = cache.keys().take(100).cloned().collect();
            for key in keys_to_remove {
                cache.remove(&key);
            }
        }
        
        cache.insert(key, solution);
        Ok(())
    }

    /// Clear optimization cache
    pub async fn clear_cache(&self) -> Result<()> {
        let mut cache = self.cache.write().await;
        cache.clear();
        info!("Optimization cache cleared");
        Ok(())
    }

    /// Get cache statistics
    pub async fn get_cache_stats(&self) -> (usize, f64) {
        let cache_size = self.cache.read().await.len();
        let hit_rate = self.metrics.read().await.get_cache_hit_rate();
        (cache_size, hit_rate)
    }

    /// Validate meal plan solution
    pub async fn validate_solution(&self, solution: &OptimizationSolution) -> Result<bool> {
        // Check if all referenced recipes exist
        let recipes = self.recipes.read().await;
        let recipe_ids: std::collections::HashSet<_> = recipes.iter().map(|r| &r.id).collect();

        // This would need actual meal plan data from the solution
        // For now, just return true if we have recipes
        Ok(!recipes.is_empty())
    }

    /// Get optimization recommendations for user
    pub async fn get_optimization_recommendations(&self, user_id: &str) -> Result<Vec<String>> {
        let mut recommendations = Vec::new();

        let metrics = self.metrics.read().await;
        let stats = metrics.get_optimization_stats();

        if let Some(avg_time) = stats.get("avg_execution_time_ms") {
            if *avg_time > 60000.0 { // 1 minute
                recommendations.push("Consider reducing optimization complexity for faster results".to_string());
            }
        }

        if let Some(avg_quality) = stats.get("avg_solution_quality") {
            if *avg_quality < 0.6 {
                recommendations.push("Try adjusting your preferences or constraints for better meal plans".to_string());
            }
        }

        let success_rate = metrics.get_success_rate();
        if success_rate < 0.8 {
            recommendations.push("Some optimizations are failing - consider relaxing constraints".to_string());
        }

        if recommendations.is_empty() {
            recommendations.push("Optimization system is running well!".to_string());
        }

        Ok(recommendations)
    }
}

impl Default for MenuOptimizer {
    fn default() -> Self {
        Self::new()
    }
}