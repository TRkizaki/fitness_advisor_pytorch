use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::HashMap;
use uuid::Uuid;

use crate::mcp::types::{CallToolResult, ToolResponseContent, UserProfile, WorkoutPlan, Exercise, DifficultyLevel};
use crate::rag::{KnowledgeBase, SearchQuery}; // Import from existing RAG system

#[derive(Debug, Deserialize)]
pub struct WorkoutPlanRequest {
    pub user_profile: UserProfile,
    pub workout_preferences: Option<WorkoutPreferences>,
}

#[derive(Debug, Deserialize)]
pub struct WorkoutPreferences {
    pub duration_minutes: Option<u32>,
    pub days_per_week: Option<u32>,
    pub difficulty_level: Option<String>,
    pub target_muscle_groups: Option<Vec<String>>,
    pub equipment_available: Option<Vec<String>>,
    pub workout_type: Option<String>, // "strength", "cardio", "mixed", "flexibility"
}

#[derive(Debug, Deserialize)]
pub struct SeasonalOptimizationRequest {
    pub location: Option<String>,
    pub season: Option<String>,
    pub indoor_preference: Option<bool>,
    pub user_profile: UserProfile,
}

#[derive(Debug, Deserialize)]
pub struct ProgressTrackingRequest {
    pub user_id: Uuid,
    pub metrics: Vec<ProgressMetric>,
    pub time_range_days: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProgressMetric {
    pub name: String,
    pub value: f32,
    pub unit: String,
    pub date: chrono::DateTime<chrono::Utc>,
    pub notes: Option<String>,
}

pub struct FitnessTools {
    // Integration with existing RAG system
    knowledge_base: Option<KnowledgeBase>,
}

impl FitnessTools {
    pub fn new() -> Self {
        Self {
            knowledge_base: None,
        }
    }

    pub fn with_knowledge_base(mut self, kb: KnowledgeBase) -> Self {
        self.knowledge_base = Some(kb);
        self
    }

    // Create personalized workout plan
    pub async fn create_workout_plan(&self, args: Option<Value>) -> Result<CallToolResult> {
        let request: WorkoutPlanRequest = match args {
            Some(args) => serde_json::from_value(args)?,
            None => return Ok(CallToolResult {
                content: vec![ToolResponseContent::Text {
                    text: "Error: No workout plan parameters provided".to_string(),
                }],
                is_error: Some(true),
            }),
        };

        let workout_plan = self.generate_workout_plan(&request).await?;
        
        let response_text = format!(
            "# Personalized Workout Plan: {}\n\n## Overview\n{}\n\n**Duration:** {} minutes\n**Difficulty:** {:?}\n**Target Muscle Groups:** {}\n\n## Exercises\n{}\n\n## Notes\n- Rest {} seconds between exercises\n- Stay hydrated throughout the workout\n- Focus on proper form over speed\n- Progress gradually by increasing intensity or duration",
            workout_plan.name,
            workout_plan.description,
            workout_plan.duration_minutes,
            workout_plan.difficulty_level,
            workout_plan.target_muscle_groups.join(", "),
            workout_plan.exercises.iter()
                .enumerate()
                .map(|(i, ex)| format!(
                    "{}. **{}**\n   - Sets: {} | Reps: {} | Rest: {}s\n   - {}\n   - Target: {}",
                    i + 1,
                    ex.name,
                    ex.sets,
                    ex.reps,
                    ex.rest_seconds,
                    ex.instructions,
                    ex.muscle_groups.join(", ")
                ))
                .collect::<Vec<_>>()
                .join("\n\n")
        );

        Ok(CallToolResult {
            content: vec![ToolResponseContent::Text {
                text: response_text,
            }],
            is_error: None,
        })
    }

    async fn generate_workout_plan(&self, request: &WorkoutPlanRequest) -> Result<WorkoutPlan> {
        let preferences = request.workout_preferences.as_ref();
        let duration = preferences.and_then(|p| p.duration_minutes).unwrap_or(45);
        let difficulty = self.determine_difficulty_level(&request.user_profile, preferences);
        
        // Determine workout type based on goals
        let workout_type = preferences
            .and_then(|p| p.workout_type.as_ref())
            .map(|s| s.as_str())
            .unwrap_or_else(|| self.determine_workout_type_from_goals(&request.user_profile.fitness_goals));

        let exercises = match workout_type {
            "strength" => self.generate_strength_exercises(&request.user_profile, preferences),
            "cardio" => self.generate_cardio_exercises(&request.user_profile, preferences),
            "mixed" => self.generate_mixed_exercises(&request.user_profile, preferences),
            "flexibility" => self.generate_flexibility_exercises(&request.user_profile, preferences),
            _ => self.generate_mixed_exercises(&request.user_profile, preferences),
        };

        let target_muscle_groups = preferences
            .and_then(|p| p.target_muscle_groups.as_ref())
            .cloned()
            .unwrap_or_else(|| vec!["full_body".to_string()]);

        let equipment_needed = preferences
            .and_then(|p| p.equipment_available.as_ref())
            .cloned()
            .unwrap_or_else(|| vec!["bodyweight".to_string()]);

        Ok(WorkoutPlan {
            id: Uuid::new_v4(),
            name: format!("{} {} Workout", 
                self.capitalize_first(&difficulty.to_string().to_lowercase()),
                self.capitalize_first(workout_type)
            ),
            description: format!(
                "A personalized {} workout designed for {} fitness level, targeting {} goals.",
                workout_type,
                difficulty.to_string().to_lowercase(),
                request.user_profile.fitness_goals.iter()
                    .map(|g| format!("{:?}", g).to_lowercase().replace('_', " "))
                    .collect::<Vec<_>>()
                    .join(" and ")
            ),
            exercises,
            duration_minutes: duration,
            difficulty_level: difficulty,
            target_muscle_groups,
            equipment_needed,
        })
    }

    fn determine_difficulty_level(&self, user_profile: &UserProfile, preferences: Option<&WorkoutPreferences>) -> DifficultyLevel {
        if let Some(prefs) = preferences {
            if let Some(difficulty) = &prefs.difficulty_level {
                return match difficulty.to_lowercase().as_str() {
                    "beginner" => DifficultyLevel::Beginner,
                    "intermediate" => DifficultyLevel::Intermediate,
                    "advanced" => DifficultyLevel::Advanced,
                    "expert" => DifficultyLevel::Expert,
                    _ => DifficultyLevel::Beginner,
                };
            }
        }

        // Determine from user profile
        match user_profile.activity_level {
            crate::mcp::types::ActivityLevel::Sedentary => DifficultyLevel::Beginner,
            crate::mcp::types::ActivityLevel::LightlyActive => DifficultyLevel::Beginner,
            crate::mcp::types::ActivityLevel::ModeratelyActive => DifficultyLevel::Intermediate,
            crate::mcp::types::ActivityLevel::VeryActive => DifficultyLevel::Advanced,
            crate::mcp::types::ActivityLevel::SuperActive => DifficultyLevel::Expert,
        }
    }

    fn determine_workout_type_from_goals(&self, goals: &[crate::mcp::types::FitnessGoal]) -> &str {
        use crate::mcp::types::FitnessGoal::*;
        
        if goals.contains(&StrengthGain) || goals.contains(&MuscleGain) {
            "strength"
        } else if goals.contains(&Endurance) || goals.contains(&WeightLoss) {
            "cardio"
        } else if goals.contains(&GeneralFitness) || goals.contains(&BodyRecomposition) {
            "mixed"
        } else {
            "mixed"
        }
    }

    fn generate_strength_exercises(&self, _profile: &UserProfile, preferences: Option<&WorkoutPreferences>) -> Vec<Exercise> {
        let has_equipment = preferences
            .and_then(|p| p.equipment_available.as_ref())
            .map(|eq| !eq.is_empty() && !eq.iter().all(|e| e == "bodyweight"))
            .unwrap_or(false);

        if has_equipment {
            vec![
                Exercise {
                    name: "Barbell Squats".to_string(),
                    sets: 3,
                    reps: 8,
                    duration_seconds: None,
                    rest_seconds: 90,
                    instructions: "Stand with feet shoulder-width apart, lower by pushing hips back and bending knees, drive through heels to return to start.".to_string(),
                    muscle_groups: vec!["quadriceps".to_string(), "glutes".to_string(), "core".to_string()],
                },
                Exercise {
                    name: "Bench Press".to_string(),
                    sets: 3,
                    reps: 8,
                    duration_seconds: None,
                    rest_seconds: 90,
                    instructions: "Lie on bench, grip bar slightly wider than shoulders, lower to chest, press up explosively.".to_string(),
                    muscle_groups: vec!["chest".to_string(), "triceps".to_string(), "shoulders".to_string()],
                },
                Exercise {
                    name: "Deadlifts".to_string(),
                    sets: 3,
                    reps: 5,
                    duration_seconds: None,
                    rest_seconds: 120,
                    instructions: "Stand with feet hip-width apart, grip bar, lift by driving through heels and extending hips.".to_string(),
                    muscle_groups: vec!["hamstrings".to_string(), "glutes".to_string(), "back".to_string(), "core".to_string()],
                },
                Exercise {
                    name: "Bent-Over Rows".to_string(),
                    sets: 3,
                    reps: 10,
                    duration_seconds: None,
                    rest_seconds: 75,
                    instructions: "Hinge at hips, pull bar to lower chest, squeeze shoulder blades together.".to_string(),
                    muscle_groups: vec!["back".to_string(), "biceps".to_string(), "rear_delts".to_string()],
                },
            ]
        } else {
            vec![
                Exercise {
                    name: "Push-ups".to_string(),
                    sets: 3,
                    reps: 12,
                    duration_seconds: None,
                    rest_seconds: 60,
                    instructions: "Start in plank position, lower chest to ground, push back up maintaining straight line.".to_string(),
                    muscle_groups: vec!["chest".to_string(), "triceps".to_string(), "core".to_string()],
                },
                Exercise {
                    name: "Bodyweight Squats".to_string(),
                    sets: 3,
                    reps: 15,
                    duration_seconds: None,
                    rest_seconds: 60,
                    instructions: "Stand with feet shoulder-width apart, lower by sitting back, drive through heels to stand.".to_string(),
                    muscle_groups: vec!["quadriceps".to_string(), "glutes".to_string()],
                },
                Exercise {
                    name: "Pike Push-ups".to_string(),
                    sets: 3,
                    reps: 8,
                    duration_seconds: None,
                    rest_seconds: 60,
                    instructions: "Start in downward dog position, lower head toward hands, press back up.".to_string(),
                    muscle_groups: vec!["shoulders".to_string(), "triceps".to_string()],
                },
                Exercise {
                    name: "Single-leg Glute Bridges".to_string(),
                    sets: 3,
                    reps: 10,
                    duration_seconds: None,
                    rest_seconds: 45,
                    instructions: "Lie on back, one foot on ground, lift hips by squeezing glutes, lower slowly.".to_string(),
                    muscle_groups: vec!["glutes".to_string(), "hamstrings".to_string(), "core".to_string()],
                },
            ]
        }
    }

    fn generate_cardio_exercises(&self, _profile: &UserProfile, _preferences: Option<&WorkoutPreferences>) -> Vec<Exercise> {
        vec![
            Exercise {
                name: "High Knees".to_string(),
                sets: 3,
                reps: 1,
                duration_seconds: Some(30),
                rest_seconds: 30,
                instructions: "Run in place, bringing knees up to hip level, pump arms rhythmically.".to_string(),
                muscle_groups: vec!["legs".to_string(), "core".to_string(), "cardiovascular".to_string()],
            },
            Exercise {
                name: "Burpees".to_string(),
                sets: 3,
                reps: 8,
                duration_seconds: None,
                rest_seconds: 60,
                instructions: "Squat down, jump back to plank, push-up, jump feet to squat, jump up with arms overhead.".to_string(),
                muscle_groups: vec!["full_body".to_string(), "cardiovascular".to_string()],
            },
            Exercise {
                name: "Mountain Climbers".to_string(),
                sets: 3,
                reps: 1,
                duration_seconds: Some(30),
                rest_seconds: 30,
                instructions: "Start in plank position, alternate bringing knees toward chest rapidly.".to_string(),
                muscle_groups: vec!["core".to_string(), "shoulders".to_string(), "cardiovascular".to_string()],
            },
            Exercise {
                name: "Jumping Jacks".to_string(),
                sets: 3,
                reps: 20,
                duration_seconds: None,
                rest_seconds: 30,
                instructions: "Stand with feet together, jump while spreading legs and raising arms overhead.".to_string(),
                muscle_groups: vec!["legs".to_string(), "arms".to_string(), "cardiovascular".to_string()],
            },
        ]
    }

    fn generate_mixed_exercises(&self, profile: &UserProfile, preferences: Option<&WorkoutPreferences>) -> Vec<Exercise> {
        let mut exercises = Vec::new();
        
        // Mix of strength and cardio
        let strength_exercises = self.generate_strength_exercises(profile, preferences);
        let cardio_exercises = self.generate_cardio_exercises(profile, preferences);
        
        // Take alternating exercises
        for i in 0..3 {
            if i < strength_exercises.len() {
                exercises.push(strength_exercises[i].clone());
            }
            if i < cardio_exercises.len() {
                exercises.push(cardio_exercises[i].clone());
            }
        }

        exercises
    }

    fn generate_flexibility_exercises(&self, _profile: &UserProfile, _preferences: Option<&WorkoutPreferences>) -> Vec<Exercise> {
        vec![
            Exercise {
                name: "Cat-Cow Stretch".to_string(),
                sets: 2,
                reps: 10,
                duration_seconds: None,
                rest_seconds: 30,
                instructions: "On hands and knees, alternate arching and rounding your back slowly.".to_string(),
                muscle_groups: vec!["spine".to_string(), "core".to_string()],
            },
            Exercise {
                name: "Downward Dog".to_string(),
                sets: 3,
                reps: 1,
                duration_seconds: Some(30),
                rest_seconds: 15,
                instructions: "From plank, lift hips up and back, straighten legs, press palms down.".to_string(),
                muscle_groups: vec!["hamstrings".to_string(), "calves".to_string(), "shoulders".to_string()],
            },
            Exercise {
                name: "Pigeon Pose".to_string(),
                sets: 2,
                reps: 1,
                duration_seconds: Some(45),
                rest_seconds: 30,
                instructions: "From downward dog, bring one knee forward, extend back leg, hold stretch.".to_string(),
                muscle_groups: vec!["hip_flexors".to_string(), "glutes".to_string()],
            },
            Exercise {
                name: "Seated Spinal Twist".to_string(),
                sets: 2,
                reps: 5,
                duration_seconds: Some(15),
                rest_seconds: 30,
                instructions: "Sit cross-legged, place hand behind you, twist gently and hold.".to_string(),
                muscle_groups: vec!["spine".to_string(), "obliques".to_string()],
            },
        ]
    }

    // Seasonal optimization for workouts
    pub async fn optimize_for_season(&self, args: Option<Value>) -> Result<CallToolResult> {
        let request: SeasonalOptimizationRequest = match args {
            Some(args) => serde_json::from_value(args)?,
            None => return Ok(CallToolResult {
                content: vec![ToolResponseContent::Text {
                    text: "Error: No seasonal optimization parameters provided".to_string(),
                }],
                is_error: Some(true),
            }),
        };

        let recommendations = self.generate_seasonal_recommendations(&request).await?;

        Ok(CallToolResult {
            content: vec![ToolResponseContent::Text {
                text: recommendations,
            }],
            is_error: None,
        })
    }

    async fn generate_seasonal_recommendations(&self, request: &SeasonalOptimizationRequest) -> Result<String> {
        let season = request.season.as_deref().unwrap_or("current");
        let location = request.location.as_deref().unwrap_or("general");
        
        let recommendations = match season.to_lowercase().as_str() {
            "winter" => format!(
                "# Winter Fitness Optimization for {}\n\n## Exercise Recommendations\n- Focus on indoor strength training and flexibility work\n- High-intensity interval training (HIIT) for cardiovascular health\n- Vitamin D supplementation consideration due to limited sunlight\n- Warming up is crucial in cold weather\n\n## Seasonal Benefits\n- More time for consistent indoor routines\n- Perfect season for building strength and muscle mass\n- Focus on mobility work to counteract seasonal stiffness\n\n## Nutrition Adjustments\n- Increase healthy fats for energy and warmth\n- Focus on immune-supporting nutrients (vitamin C, zinc)\n- Maintain hydration despite reduced thirst sensation",
                location
            ),
            "spring" => format!(
                "# Spring Fitness Optimization for {}\n\n## Exercise Recommendations\n- Transition to more outdoor activities as weather improves\n- Gradual increase in activity levels after winter\n- Incorporate nature walks and hiking\n- Start preparing for summer activities\n\n## Seasonal Benefits\n- Increased daylight improves mood and motivation\n- Perfect time to establish new fitness habits\n- Natural detoxification and renewal energy\n\n## Nutrition Adjustments\n- Focus on fresh, seasonal vegetables\n- Lighter meals as metabolism adjusts\n- Increase hydration as activity levels rise",
                location
            ),
            "summer" => format!(
                "# Summer Fitness Optimization for {}\n\n## Exercise Recommendations\n- Early morning or evening workouts to avoid heat\n- Water-based activities (swimming, water aerobics)\n- Increased focus on hydration and electrolyte balance\n- Take advantage of longer daylight hours\n\n## Seasonal Benefits\n- Peak time for outdoor activities and sports\n- Natural vitamin D production from sun exposure\n- Social fitness activities and group sports\n\n## Nutrition Adjustments\n- Emphasize cooling foods and increased water intake\n- Fresh fruits and vegetables at their peak\n- Lighter, more frequent meals",
                location
            ),
            "autumn" | "fall" => format!(
                "# Autumn Fitness Optimization for {}\n\n## Exercise Recommendations\n- Take advantage of comfortable temperatures for outdoor activities\n- Begin transitioning to more indoor-friendly routines\n- Focus on building strength for the upcoming winter\n- Hiking and nature activities with beautiful scenery\n\n## Seasonal Benefits\n- Comfortable temperatures for most activities\n- Harvest season provides nutritious seasonal foods\n- Good time to establish indoor backup routines\n\n## Nutrition Adjustments\n- Incorporate seasonal produce (squashes, apples, root vegetables)\n- Gradually increase caloric intake for winter preparation\n- Focus on immune system support",
                location
            ),
            _ => format!(
                "# Year-Round Fitness Optimization for {}\n\n## General Recommendations\n- Adapt activities based on current weather and season\n- Maintain consistency regardless of seasonal changes\n- Listen to your body's seasonal energy fluctuations\n- Plan ahead for seasonal transitions\n\n## Universal Principles\n- Stay hydrated year-round\n- Adjust nutrition to seasonal availability\n- Maintain vitamin D levels through supplementation or sun exposure\n- Keep both indoor and outdoor activity options available",
                location
            ),
        };

        Ok(recommendations)
    }

    // Progress tracking and analysis
    pub async fn track_progress(&self, args: Option<Value>) -> Result<CallToolResult> {
        let request: ProgressTrackingRequest = match args {
            Some(args) => serde_json::from_value(args)?,
            None => return Ok(CallToolResult {
                content: vec![ToolResponseContent::Text {
                    text: "Error: No progress tracking parameters provided".to_string(),
                }],
                is_error: Some(true),
            }),
        };

        let analysis = self.analyze_progress(&request).await?;

        Ok(CallToolResult {
            content: vec![ToolResponseContent::Text {
                text: analysis,
            }],
            is_error: None,
        })
    }

    async fn analyze_progress(&self, request: &ProgressTrackingRequest) -> Result<String> {
        if request.metrics.is_empty() {
            return Ok("No metrics provided for analysis.".to_string());
        }

        let mut analysis = format!("# Progress Analysis for User {}\n\n", request.user_id);
        
        // Group metrics by type
        let mut metric_groups: HashMap<String, Vec<&ProgressMetric>> = HashMap::new();
        for metric in &request.metrics {
            metric_groups.entry(metric.name.clone()).or_default().push(metric);
        }

        analysis.push_str("## Metric Summary\n\n");
        for (metric_name, metrics) in &metric_groups {
            if metrics.len() >= 2 {
                let first = metrics.first().unwrap();
                let last = metrics.last().unwrap();
                let change = last.value - first.value;
                let change_percent = if first.value != 0.0 {
                    (change / first.value) * 100.0
                } else {
                    0.0
                };

                analysis.push_str(&format!(
                    "**{}**: {} {} → {} {} (Change: {:.1} {} / {:.1}%)\n",
                    metric_name,
                    first.value,
                    first.unit,
                    last.value,
                    last.unit,
                    change,
                    first.unit,
                    change_percent
                ));
            } else if let Some(metric) = metrics.first() {
                analysis.push_str(&format!(
                    "**{}**: {} {}\n",
                    metric_name, metric.value, metric.unit
                ));
            }
        }

        analysis.push_str("\n## Recommendations\n\n");
        
        // Generate recommendations based on progress
        for (metric_name, metrics) in &metric_groups {
            if metrics.len() >= 2 {
                let trend = self.analyze_trend(metrics);
                analysis.push_str(&format!("- **{}**: {}\n", metric_name, trend));
            }
        }

        Ok(analysis)
    }

    fn analyze_trend(&self, metrics: &[&ProgressMetric]) -> String {
        if metrics.len() < 2 {
            return "Insufficient data for trend analysis".to_string();
        }

        let first = metrics.first().unwrap();
        let last = metrics.last().unwrap();
        let change = last.value - first.value;

        match first.name.to_lowercase().as_str() {
            name if name.contains("weight") && name.contains("body") => {
                if change < -0.5 {
                    "Great progress! Continue with current nutrition and exercise plan.".to_string()
                } else if change > 0.5 {
                    "Consider reviewing caloric intake and increasing cardio activities.".to_string()
                } else {
                    "Stable weight - good for maintenance or recomposition goals.".to_string()
                }
            }
            name if name.contains("strength") || name.contains("max") => {
                if change > 0.0 {
                    "Strength gains detected! Continue progressive overload approach.".to_string()
                } else {
                    "Consider deload week or form check with trainer.".to_string()
                }
            }
            name if name.contains("cardio") || name.contains("endurance") => {
                if change > 0.0 {
                    "Cardiovascular improvements! Keep up consistent training.".to_string()
                } else {
                    "Consider varying cardio modalities or checking recovery.".to_string()
                }
            }
            _ => {
                if change > 0.0 {
                    "Positive trend observed - maintain current approach.".to_string()
                } else if change < 0.0 {
                    "Declining trend - may need program adjustment.".to_string()
                } else {
                    "Stable metrics - consider if this aligns with your goals.".to_string()
                }
            }
        }
    }

    fn capitalize_first(&self, s: &str) -> String {
        let mut c = s.chars();
        match c.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
        }
    }

    // Integration with RAG system for evidence-based recommendations
    pub async fn get_evidence_based_recommendation(&self, query: &str) -> Result<String> {
        if let Some(kb) = &self.knowledge_base {
            let search_query = SearchQuery {
                query: query.to_string(),
                limit: Some(3),
                filters: None,
                threshold: Some(0.7),
            };

            // This would integrate with the existing RAG system
            // For now, return a placeholder
            Ok(format!("Evidence-based recommendation for '{}' would be retrieved from the knowledge base here.", query))
        } else {
            Ok("Knowledge base not available for evidence-based recommendations.".to_string())
        }
    }
}

impl Default for FitnessTools {
    fn default() -> Self {
        Self::new()
    }
}