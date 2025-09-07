// src/advisors/menu_optimizer/algorithm/genetic.rs - Genetic Algorithm for Menu Optimization

use crate::core::{FitnessError, Result};
use crate::models::{
    optimization::*,
    food::{Recipe, NutritionFacts, MealType, Food},
};
use rand::{Rng, SeedableRng};
use rand::seq::SliceRandom;
use rand_distr::{Distribution, Normal};
use rayon::prelude::*;
use std::collections::HashMap;
use std::time::Instant;

pub struct GeneticAlgorithm {
    pub config: AlgorithmConfig,
    pub recipes: Vec<Recipe>,
    pub foods: HashMap<String, Food>,
    rng: rand::rngs::StdRng,
}

impl GeneticAlgorithm {
    pub fn new(
        config: AlgorithmConfig,
        recipes: Vec<Recipe>,
        foods: HashMap<String, Food>,
        seed: Option<u64>,
    ) -> Self {
        let rng = match seed {
            Some(s) => rand::rngs::StdRng::seed_from_u64(s),
            None => rand::rngs::StdRng::from_entropy(),
        };

        Self {
            config,
            recipes,
            foods,
            rng,
        }
    }

    pub fn optimize(&mut self, request: &OptimizationRequest) -> Result<OptimizationSolution> {
        let start_time = Instant::now();
        
        // Validate the optimization request
        request.validate()
            .map_err(|e| FitnessError::optimization(format!("Invalid request: {}", e)))?;

        // Initialize population
        let mut population = self.create_initial_population(request)?;
        
        // Evaluate initial population
        self.evaluate_population(&mut population, request)?;
        
        let mut best_fitness_history = Vec::new();
        let mut convergence_generation = None;
        let mut generations_run = 0;

        // Evolution loop
        for generation in 0..self.config.max_generations {
            generations_run = generation + 1;

            // Check runtime limit
            if start_time.elapsed().as_secs() > self.config.max_runtime_seconds {
                break;
            }

            // Track best fitness
            let best_fitness = population.iter()
                .map(|ind| ind.get_fitness())
                .fold(f64::NEG_INFINITY, f64::max);
            best_fitness_history.push(best_fitness);

            // Check convergence
            if best_fitness_history.len() >= 50 {
                let recent_improvement = best_fitness_history.iter().rev().take(50)
                    .fold(0.0, |acc, &f| (f - best_fitness_history[best_fitness_history.len() - 51]).max(acc));
                
                if recent_improvement < self.config.convergence_threshold {
                    convergence_generation = Some(generation);
                    break;
                }
            }

            // Selection
            let parents = self.selection(&population);

            // Crossover and Mutation
            let mut offspring = self.create_offspring(&parents, request)?;

            // Evaluate offspring
            self.evaluate_population(&mut offspring, request)?;

            // Survivor selection (elitism + tournament)
            population = self.survivor_selection(population, offspring);

            // Age population
            for individual in &mut population {
                individual.age += 1;
            }
        }

        // Create solution from best individual
        let best_individual = population.into_iter()
            .max_by(|a, b| a.get_fitness().partial_cmp(&b.get_fitness()).unwrap())
            .ok_or_else(|| FitnessError::optimization("No valid solution found"))?;

        self.create_solution(best_individual, AlgorithmMetadata {
            algorithm_used: AlgorithmType::GeneticAlgorithm,
            generations_run,
            final_population_size: self.config.population_size,
            convergence_generation,
            execution_time_ms: start_time.elapsed().as_millis() as f64,
            evaluations_performed: generations_run * self.config.population_size,
            best_fitness_history,
            diversity_score: 0.75, // TODO: Calculate actual diversity
        })
    }

    fn create_initial_population(&mut self, request: &OptimizationRequest) -> Result<Vec<Individual>> {
        let mut population = Vec::with_capacity(self.config.population_size);

        for _ in 0..self.config.population_size {
            let individual = self.create_random_individual(request)?;
            population.push(individual);
        }

        Ok(population)
    }

    fn create_random_individual(&mut self, request: &OptimizationRequest) -> Result<Individual> {
        let mut genome = Vec::new();

        for day in 0..request.time_horizon_days {
            // Add breakfast
            for _ in 0..request.constraints.meal_count_per_day.breakfast {
                genome.push(self.create_random_meal_gene(day, MealType::Breakfast)?);
            }

            // Add lunch
            for _ in 0..request.constraints.meal_count_per_day.lunch {
                genome.push(self.create_random_meal_gene(day, MealType::Lunch)?);
            }

            // Add dinner
            for _ in 0..request.constraints.meal_count_per_day.dinner {
                genome.push(self.create_random_meal_gene(day, MealType::Dinner)?);
            }

            // Add snacks
            for _ in 0..request.constraints.meal_count_per_day.snacks {
                genome.push(self.create_random_meal_gene(day, MealType::Snack)?);
            }
        }

        Ok(Individual::new(genome))
    }

    fn create_random_meal_gene(&mut self, day: u32, meal_type: MealType) -> Result<MealGene> {
        // Filter recipes by meal type and user preferences
        let suitable_recipes: Vec<_> = self.recipes.iter()
            .filter(|recipe| recipe.meal_type == meal_type)
            .collect();

        if suitable_recipes.is_empty() {
            return Err(FitnessError::optimization(
                format!("No suitable recipes found for meal type: {:?}", meal_type)
            ));
        }

        let recipe = suitable_recipes.choose(&mut self.rng)
            .ok_or_else(|| FitnessError::optimization("Failed to select random recipe"))?;

        // Random portion size between 0.5 and 2.0
        let portion_size = self.rng.gen_range(0.5..=2.0);

        Ok(MealGene {
            day,
            meal_type,
            recipe_id: recipe.id.clone(),
            portion_size,
        })
    }

    fn evaluate_population(&self, population: &mut [Individual], request: &OptimizationRequest) -> Result<()> {
        if self.config.parallel_evaluation {
            population.par_iter_mut()
                .try_for_each(|individual| self.evaluate_individual(individual, request))?;
        } else {
            for individual in population {
                self.evaluate_individual(individual, request)?;
            }
        }

        Ok(())
    }

    fn evaluate_individual(&self, individual: &mut Individual, request: &OptimizationRequest) -> Result<()> {
        // Calculate total nutrition for the meal plan
        let total_nutrition = self.calculate_total_nutrition(&individual.genome)?;

        // Evaluate objectives
        let mut objective_scores = HashMap::new();
        
        for objective in &request.objectives {
            let score = match objective {
                OptimizationObjective::MaximizeNutrition => self.evaluate_nutrition_quality(&total_nutrition),
                OptimizationObjective::MinimizeCost => self.evaluate_cost(&individual.genome)?,
                OptimizationObjective::MaximizeTasteScore => self.evaluate_taste_score(&individual.genome, &request.preferences)?,
                OptimizationObjective::MaximizeVariety => self.evaluate_variety(&individual.genome),
                OptimizationObjective::MinimizePreparationTime => self.evaluate_preparation_time(&individual.genome)?,
                OptimizationObjective::MaximizeSeasonality => self.evaluate_seasonality(&individual.genome),
                OptimizationObjective::BalanceMacros => self.evaluate_macro_balance(&total_nutrition, &request.constraints),
                OptimizationObjective::MinimizeFoodWaste => 0.8, // Placeholder
            };
            
            objective_scores.insert(format!("{:?}", objective), score);
        }

        // Check constraints and calculate violations
        let constraint_violations = self.check_constraints(&total_nutrition, request);

        // Calculate fitness (weighted sum of objectives with penalties)
        let base_fitness: f64 = objective_scores.values().sum::<f64>() / objective_scores.len() as f64;
        let constraint_penalty = individual.get_total_constraint_violation() * 0.1;
        let fitness = (base_fitness - constraint_penalty).max(0.0);

        individual.fitness = Some(fitness);
        individual.objective_scores = objective_scores;
        individual.constraint_violations = constraint_violations;

        Ok(())
    }

    fn calculate_total_nutrition(&self, genome: &[MealGene]) -> Result<NutritionFacts> {
        let mut total = NutritionFacts::new();

        for gene in genome {
            let recipe = self.recipes.iter()
                .find(|r| r.id == gene.recipe_id)
                .ok_or_else(|| FitnessError::optimization(format!("Recipe not found: {}", gene.recipe_id)))?;

            let mut meal_nutrition = recipe.nutrition_per_serving.clone();
            
            // Scale by portion size
            meal_nutrition.calories *= gene.portion_size;
            meal_nutrition.protein_g *= gene.portion_size;
            meal_nutrition.carbs_g *= gene.portion_size;
            meal_nutrition.fat_g *= gene.portion_size;
            meal_nutrition.fiber_g *= gene.portion_size;
            meal_nutrition.sugar_g *= gene.portion_size;
            meal_nutrition.sodium_mg *= gene.portion_size;
            meal_nutrition.potassium_mg *= gene.portion_size;
            meal_nutrition.calcium_mg *= gene.portion_size;
            meal_nutrition.iron_mg *= gene.portion_size;
            meal_nutrition.vitamin_c_mg *= gene.portion_size;
            meal_nutrition.vitamin_d_iu *= gene.portion_size;
            meal_nutrition.vitamin_b12_mcg *= gene.portion_size;
            meal_nutrition.folate_mcg *= gene.portion_size;
            meal_nutrition.omega3_g *= gene.portion_size;
            meal_nutrition.omega6_g *= gene.portion_size;

            total.add(&meal_nutrition);
        }

        Ok(total)
    }

    fn evaluate_nutrition_quality(&self, nutrition: &NutritionFacts) -> f64 {
        nutrition.calculate_nutrition_score()
    }

    fn evaluate_cost(&self, genome: &[MealGene]) -> Result<f64> {
        let total_cost: f64 = genome.iter()
            .map(|gene| {
                self.recipes.iter()
                    .find(|r| r.id == gene.recipe_id)
                    .and_then(|r| r.cost_per_serving)
                    .unwrap_or(5.0) * gene.portion_size // Default cost if not available
            })
            .sum();

        // Convert cost to score (lower cost = higher score)
        Ok(1.0 / (1.0 + total_cost / 100.0))
    }

    fn evaluate_taste_score(&self, genome: &[MealGene], preferences: &UserPreferences) -> Result<f64> {
        let mut total_score = 0.0;
        let mut count = 0;

        for gene in genome {
            if let Some(recipe) = self.recipes.iter().find(|r| r.id == gene.recipe_id) {
                // Calculate taste compatibility based on user preferences
                let mut taste_score = 0.5; // Base score

                // Check cuisine preferences
                if let Some(ref cuisine) = recipe.cuisine_type {
                    if preferences.cuisine_preferences.contains(cuisine) {
                        taste_score += 0.3;
                    }
                }

                // Check liked/disliked foods
                let recipe_food_ids: Vec<String> = recipe.ingredients.iter()
                    .map(|i| i.food_id.clone())
                    .collect();

                let disliked_count = recipe_food_ids.iter()
                    .filter(|id| preferences.disliked_foods.contains(id))
                    .count();

                let preferred_count = recipe_food_ids.iter()
                    .filter(|id| preferences.preferred_foods.contains(id))
                    .count();

                taste_score += (preferred_count as f64 * 0.1) - (disliked_count as f64 * 0.2);

                total_score += taste_score.max(0.0).min(1.0);
                count += 1;
            }
        }

        Ok(if count > 0 { total_score / count as f64 } else { 0.5 })
    }

    fn evaluate_variety(&self, genome: &[MealGene]) -> f64 {
        let unique_recipes: std::collections::HashSet<_> = genome.iter()
            .map(|gene| &gene.recipe_id)
            .collect();

        let total_meals = genome.len();
        if total_meals == 0 {
            return 0.0;
        }

        unique_recipes.len() as f64 / total_meals as f64
    }

    fn evaluate_preparation_time(&self, genome: &[MealGene]) -> Result<f64> {
        let total_time: u32 = genome.iter()
            .map(|gene| {
                self.recipes.iter()
                    .find(|r| r.id == gene.recipe_id)
                    .map(|r| r.prep_time_minutes + r.cook_time_minutes)
                    .unwrap_or(30) // Default time if not available
            })
            .sum();

        // Convert time to score (less time = higher score)
        Ok(1.0 / (1.0 + total_time as f64 / 1000.0))
    }

    fn evaluate_seasonality(&self, _genome: &[MealGene]) -> f64 {
        // Placeholder for seasonality evaluation
        // Would check current season and food availability
        0.7
    }

    fn evaluate_macro_balance(&self, nutrition: &NutritionFacts, constraints: &NutritionConstraints) -> f64 {
        let (protein_ratio, carbs_ratio, fat_ratio) = nutrition.get_macro_ratio();
        
        // Calculate ideal ratios based on constraints
        let protein_target = (constraints.macros.protein_g.min + constraints.macros.protein_g.max) / 2.0;
        let carbs_target = (constraints.macros.carbs_g.min + constraints.macros.carbs_g.max) / 2.0;
        let fat_target = (constraints.macros.fat_g.min + constraints.macros.fat_g.max) / 2.0;
        
        let total_target = protein_target + carbs_target + fat_target;
        let ideal_protein_ratio = protein_target / total_target;
        let ideal_carbs_ratio = carbs_target / total_target;
        let ideal_fat_ratio = fat_target / total_target;

        // Calculate deviation from ideal ratios
        let protein_deviation = (protein_ratio - ideal_protein_ratio).abs();
        let carbs_deviation = (carbs_ratio - ideal_carbs_ratio).abs();
        let fat_deviation = (fat_ratio - ideal_fat_ratio).abs();

        let total_deviation = protein_deviation + carbs_deviation + fat_deviation;
        
        // Convert to score (less deviation = higher score)
        1.0 - (total_deviation / 3.0).min(1.0)
    }

    fn check_constraints(&self, nutrition: &NutritionFacts, request: &OptimizationRequest) -> Vec<ConstraintViolation> {
        let mut violations = Vec::new();

        // Check calorie constraints
        if nutrition.calories < request.constraints.daily_calories.min {
            violations.push(ConstraintViolation {
                constraint_type: "daily_calories_min".to_string(),
                severity: ViolationSeverity::High,
                current_value: nutrition.calories,
                required_value: request.constraints.daily_calories.min,
                description: "Daily calories below minimum requirement".to_string(),
            });
        }

        if nutrition.calories > request.constraints.daily_calories.max {
            violations.push(ConstraintViolation {
                constraint_type: "daily_calories_max".to_string(),
                severity: ViolationSeverity::High,
                current_value: nutrition.calories,
                required_value: request.constraints.daily_calories.max,
                description: "Daily calories exceed maximum limit".to_string(),
            });
        }

        // Check macro constraints
        let macros = &request.constraints.macros;

        if nutrition.protein_g < macros.protein_g.min {
            violations.push(ConstraintViolation {
                constraint_type: "protein_min".to_string(),
                severity: ViolationSeverity::Medium,
                current_value: nutrition.protein_g,
                required_value: macros.protein_g.min,
                description: "Protein intake below minimum requirement".to_string(),
            });
        }

        if let Some(sodium_max) = macros.sodium_mg_max {
            if nutrition.sodium_mg > sodium_max {
                violations.push(ConstraintViolation {
                    constraint_type: "sodium_max".to_string(),
                    severity: ViolationSeverity::Medium,
                    current_value: nutrition.sodium_mg,
                    required_value: sodium_max,
                    description: "Sodium intake exceeds maximum limit".to_string(),
                });
            }
        }

        violations
    }

    fn selection(&mut self, population: &[Individual]) -> Vec<Individual> {
        let mut parents = Vec::new();
        let tournament_size = 5;

        for _ in 0..population.len() {
            // Tournament selection
            let mut tournament: Vec<_> = (0..tournament_size)
                .map(|_| &population[self.rng.gen_range(0..population.len())])
                .collect();

            tournament.sort_by(|a, b| b.get_fitness().partial_cmp(&a.get_fitness()).unwrap());
            parents.push(tournament[0].clone());
        }

        parents
    }

    fn create_offspring(&mut self, parents: &[Individual], request: &OptimizationRequest) -> Result<Vec<Individual>> {
        let mut offspring = Vec::new();

        for chunk in parents.chunks(2) {
            if chunk.len() == 2 {
                let (child1, child2) = self.crossover(&chunk[0], &chunk[1])?;
                
                let mut mutated_child1 = self.mutate(child1, request)?;
                let mut mutated_child2 = self.mutate(child2, request)?;

                offspring.push(mutated_child1);
                offspring.push(mutated_child2);
            }
        }

        Ok(offspring)
    }

    fn crossover(&mut self, parent1: &Individual, parent2: &Individual) -> Result<(Individual, Individual)> {
        if self.rng.gen::<f64>() > self.config.crossover_rate {
            return Ok((parent1.clone(), parent2.clone()));
        }

        let len = parent1.genome.len().min(parent2.genome.len());
        if len == 0 {
            return Ok((parent1.clone(), parent2.clone()));
        }

        let crossover_point = self.rng.gen_range(1..len);

        let mut child1_genome = parent1.genome[..crossover_point].to_vec();
        child1_genome.extend_from_slice(&parent2.genome[crossover_point..]);

        let mut child2_genome = parent2.genome[..crossover_point].to_vec();
        child2_genome.extend_from_slice(&parent1.genome[crossover_point..]);

        Ok((Individual::new(child1_genome), Individual::new(child2_genome)))
    }

    fn mutate(&mut self, mut individual: Individual, request: &OptimizationRequest) -> Result<Individual> {
        for gene in &mut individual.genome {
            if self.rng.gen::<f64>() < self.config.mutation_rate {
                // Mutate this gene
                match self.rng.gen_range(0..3) {
                    0 => {
                        // Change recipe
                        let new_gene = self.create_random_meal_gene(gene.day, gene.meal_type.clone())?;
                        gene.recipe_id = new_gene.recipe_id;
                    }
                    1 => {
                        // Adjust portion size
                        let normal = Normal::new(0.0, 0.1).unwrap();
                        let adjustment = normal.sample(&mut self.rng);
                        gene.portion_size = (gene.portion_size + adjustment).max(0.3).min(3.0);
                    }
                    _ => {
                        // Small chance to change meal type (within constraints)
                        // This is more complex and would need additional logic
                    }
                }
            }
        }

        // Reset fitness since the individual has changed
        individual.fitness = None;
        individual.objective_scores.clear();
        individual.constraint_violations.clear();

        Ok(individual)
    }

    fn survivor_selection(&self, mut population: Vec<Individual>, mut offspring: Vec<Individual>) -> Vec<Individual> {
        // Combine population and offspring
        population.append(&mut offspring);

        // Sort by fitness (descending)
        population.sort_by(|a, b| b.get_fitness().partial_cmp(&a.get_fitness()).unwrap());

        // Keep the best individuals (elitism)
        population.truncate(self.config.population_size);

        population
    }

    fn create_solution(&self, individual: Individual, metadata: AlgorithmMetadata) -> Result<OptimizationSolution> {
        let nutrition_summary = self.calculate_total_nutrition(&individual.genome)?;
        
        // Calculate additional scores
        let variety_score = self.evaluate_variety(&individual.genome);
        let taste_score = individual.objective_scores.get("MaximizeTasteScore").copied().unwrap_or(0.5);
        let convenience_score = individual.objective_scores.get("MinimizePreparationTime").copied().unwrap_or(0.5);
        let seasonality_score = self.evaluate_seasonality(&individual.genome);

        // Calculate total cost
        let total_cost = if let Ok(cost_score) = self.evaluate_cost(&individual.genome) {
            Some(100.0 * (1.0 - cost_score)) // Convert score back to cost estimate
        } else {
            None
        };

        Ok(OptimizationSolution {
            meal_plan_id: uuid::Uuid::new_v4().to_string(),
            fitness_score: individual.get_fitness(),
            objective_scores: individual.objective_scores,
            constraint_violations: individual.constraint_violations,
            nutrition_summary,
            total_cost,
            variety_score,
            taste_score,
            convenience_score,
            seasonality_score,
            algorithm_metadata: metadata,
        })
    }
}

impl Clone for Individual {
    fn clone(&self) -> Self {
        Self {
            genome: self.genome.clone(),
            fitness: self.fitness,
            objective_scores: self.objective_scores.clone(),
            constraint_violations: self.constraint_violations.clone(),
            age: self.age,
        }
    }
}