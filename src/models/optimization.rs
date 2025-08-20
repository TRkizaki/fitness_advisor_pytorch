// src/models/optimization.rs - Optimization algorithm types and constraints

use serde::{Deserialize, Serialize};
use crate::models::food::{Allergen, DietaryFlag, NutritionFacts, MealType};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationRequest {
    pub user_id: String,
    pub constraints: NutritionConstraints,
    pub preferences: UserPreferences,
    pub objectives: Vec<OptimizationObjective>,
    pub time_horizon_days: u32,
    pub algorithm_config: AlgorithmConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutritionConstraints {
    pub daily_calories: CalorieRange,
    pub macros: MacroConstraints,
    pub micronutrients: MicronutrientConstraints,
    pub meal_count_per_day: MealCountConstraints,
    pub budget_per_day: Option<f64>,
    pub preparation_time_max_minutes: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalorieRange {
    pub min: f64,
    pub max: f64,
    pub target: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacroConstraints {
    pub protein_g: Range,
    pub carbs_g: Range,
    pub fat_g: Range,
    pub fiber_g: Range,
    pub sugar_g_max: Option<f64>,
    pub sodium_mg_max: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MicronutrientConstraints {
    pub vitamin_c_mg: Range,
    pub calcium_mg: Range,
    pub iron_mg: Range,
    pub vitamin_d_iu: Range,
    pub vitamin_b12_mcg: Range,
    pub folate_mcg: Range,
    pub omega3_g: Range,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Range {
    pub min: f64,
    pub max: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MealCountConstraints {
    pub breakfast: u32,
    pub lunch: u32,
    pub dinner: u32,
    pub snacks: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub dietary_restrictions: Vec<DietaryFlag>,
    pub allergens_to_avoid: Vec<Allergen>,
    pub cuisine_preferences: Vec<String>,
    pub disliked_foods: Vec<String>, // Food IDs
    pub preferred_foods: Vec<String>, // Food IDs
    pub taste_preferences: TastePreferences,
    pub cooking_skill_level: CookingSkillLevel,
    pub equipment_available: Vec<CookingEquipment>,
    pub meal_variety_importance: f64, // 0.0 to 1.0
    pub cost_importance: f64,         // 0.0 to 1.0
    pub health_importance: f64,       // 0.0 to 1.0
    pub convenience_importance: f64,  // 0.0 to 1.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TastePreferences {
    pub sweetness_preference: f64,  // -1.0 (avoid) to 1.0 (love)
    pub saltiness_preference: f64,
    pub sourness_preference: f64,
    pub bitterness_preference: f64,
    pub umami_preference: f64,
    pub spiciness_preference: f64,
    pub spice_tolerance: f64,       // 0.0 to 1.0
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CookingSkillLevel {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CookingEquipment {
    Stovetop,
    Oven,
    Microwave,
    SlowCooker,
    AirFryer,
    Blender,
    FoodProcessor,
    Grill,
    Steamer,
    PressureCooker,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationObjective {
    MinimizeCost,
    MaximizeNutrition,
    MaximizeTasteScore,
    MinimizePreparationTime,
    MaximizeVariety,
    MinimizeFoodWaste,
    MaximizeSeasonality,
    BalanceMacros,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlgorithmConfig {
    pub algorithm_type: AlgorithmType,
    pub population_size: usize,
    pub max_generations: usize,
    pub mutation_rate: f64,
    pub crossover_rate: f64,
    pub elitism_rate: f64,
    pub convergence_threshold: f64,
    pub max_runtime_seconds: u64,
    pub parallel_evaluation: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AlgorithmType {
    GeneticAlgorithm,
    ParticleSwarmOptimization,
    NSGA2,
    SimulatedAnnealing,
    Hybrid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationSolution {
    pub meal_plan_id: String,
    pub fitness_score: f64,
    pub objective_scores: HashMap<String, f64>,
    pub constraint_violations: Vec<ConstraintViolation>,
    pub nutrition_summary: NutritionFacts,
    pub total_cost: Option<f64>,
    pub variety_score: f64,
    pub taste_score: f64,
    pub convenience_score: f64,
    pub seasonality_score: f64,
    pub algorithm_metadata: AlgorithmMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstraintViolation {
    pub constraint_type: String,
    pub severity: ViolationSeverity,
    pub current_value: f64,
    pub required_value: f64,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ViolationSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlgorithmMetadata {
    pub algorithm_used: AlgorithmType,
    pub generations_run: usize,
    pub final_population_size: usize,
    pub convergence_generation: Option<usize>,
    pub execution_time_ms: f64,
    pub evaluations_performed: usize,
    pub best_fitness_history: Vec<f64>,
    pub diversity_score: f64,
}

#[derive(Debug)]
pub struct Individual {
    pub genome: Vec<MealGene>,
    pub fitness: Option<f64>,
    pub objective_scores: HashMap<String, f64>,
    pub constraint_violations: Vec<ConstraintViolation>,
    pub age: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MealGene {
    pub day: u32,
    pub meal_type: MealType,
    pub recipe_id: String,
    pub portion_size: f64,
}

impl Default for AlgorithmConfig {
    fn default() -> Self {
        Self {
            algorithm_type: AlgorithmType::GeneticAlgorithm,
            population_size: 100,
            max_generations: 500,
            mutation_rate: 0.1,
            crossover_rate: 0.8,
            elitism_rate: 0.1,
            convergence_threshold: 0.001,
            max_runtime_seconds: 300, // 5 minutes
            parallel_evaluation: true,
        }
    }
}

impl OptimizationRequest {
    pub fn validate(&self) -> Result<(), String> {
        // Validate calorie range
        if self.constraints.daily_calories.min >= self.constraints.daily_calories.max {
            return Err("Invalid calorie range: min must be less than max".to_string());
        }

        if self.constraints.daily_calories.target < self.constraints.daily_calories.min ||
           self.constraints.daily_calories.target > self.constraints.daily_calories.max {
            return Err("Target calories must be within min/max range".to_string());
        }

        // Validate time horizon
        if self.time_horizon_days == 0 || self.time_horizon_days > 30 {
            return Err("Time horizon must be between 1 and 30 days".to_string());
        }

        // Validate algorithm config
        if self.algorithm_config.population_size < 10 {
            return Err("Population size must be at least 10".to_string());
        }

        if self.algorithm_config.max_generations == 0 {
            return Err("Max generations must be greater than 0".to_string());
        }

        if self.algorithm_config.mutation_rate < 0.0 || self.algorithm_config.mutation_rate > 1.0 {
            return Err("Mutation rate must be between 0.0 and 1.0".to_string());
        }

        Ok(())
    }
}

impl Individual {
    pub fn new(genome: Vec<MealGene>) -> Self {
        Self {
            genome,
            fitness: None,
            objective_scores: HashMap::new(),
            constraint_violations: Vec::new(),
            age: 0,
        }
    }

    pub fn is_evaluated(&self) -> bool {
        self.fitness.is_some()
    }

    pub fn get_fitness(&self) -> f64 {
        self.fitness.unwrap_or(0.0)
    }

    pub fn get_total_constraint_violation(&self) -> f64 {
        self.constraint_violations.iter()
            .map(|v| match v.severity {
                ViolationSeverity::Low => 1.0,
                ViolationSeverity::Medium => 5.0,
                ViolationSeverity::High => 10.0,
                ViolationSeverity::Critical => 50.0,
            })
            .sum()
    }

    pub fn is_feasible(&self) -> bool {
        self.constraint_violations.iter()
            .all(|v| v.severity != ViolationSeverity::Critical)
    }
}

impl Range {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn contains(&self, value: f64) -> bool {
        value >= self.min && value <= self.max
    }

    pub fn violation_amount(&self, value: f64) -> f64 {
        if value < self.min {
            self.min - value
        } else if value > self.max {
            value - self.max
        } else {
            0.0
        }
    }
}