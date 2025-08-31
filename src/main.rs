// src/main.rs - Main application with database integration

mod database;
mod ml_client;
mod config;
mod core;
mod models;
mod advisors;
mod sample_data;
mod api;
mod ai_analytics;
mod websocket;

use std::sync::Arc;
use anyhow::Result;
use uuid::Uuid;
use tracing::{info, warn};

// Database imports
use database::DatabaseManager;
use ml_client::MLServiceClient;
use config::Config;
use advisors::{MenuOptimizer, menu_optimizer::DataLoader};
use models::*;
use ai_analytics::*;


pub struct FitnessAdvisor {
    db: Arc<DatabaseManager>,
}

impl FitnessAdvisor {
    pub async fn new(database_url: &str) -> Result<Self> {
        let db = Arc::new(DatabaseManager::new(database_url).await?);
        Ok(Self { db })
    }

    pub async fn register_user(&self, user: User) -> Result<()> {
        self.db.save_user(&user).await
    }

    pub async fn get_user(&self, user_id: &str) -> Result<Option<User>> {
        self.db.get_user(user_id).await
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>> {
        self.db.get_all_users().await
    }

    pub async fn recommend_workout(&self, user_id: &str) -> Result<Vec<ExerciseSet>> {
        let user = self.db.get_user(user_id).await?
            .ok_or_else(|| anyhow::anyhow!("User not found"))?;

        let mut recommendations = Vec::new();

        match user.fitness_level {
            FitnessLevel::Beginner => {
                recommendations.push(ExerciseSet {
                    exercise_id: "squat".to_string(),
                    sets: 2,
                    reps: 10,
                    weight_kg: None,
                    duration_seconds: None,
                    rest_seconds: 60,
                    completed: false,
                });
                
                recommendations.push(ExerciseSet {
                    exercise_id: "pushup".to_string(),
                    sets: 2,
                    reps: 8,
                    weight_kg: None,
                    duration_seconds: None,
                    rest_seconds: 60,
                    completed: false,
                });

                recommendations.push(ExerciseSet {
                    exercise_id: "plank".to_string(),
                    sets: 2,
                    reps: 1,
                    weight_kg: None,
                    duration_seconds: Some(30),
                    rest_seconds: 60,
                    completed: false,
                });
            },
            
            FitnessLevel::Intermediate => {
                recommendations.push(ExerciseSet {
                    exercise_id: "squat".to_string(),
                    sets: 3,
                    reps: 15,
                    weight_kg: None,
                    duration_seconds: None,
                    rest_seconds: 45,
                    completed: false,
                });
                
                recommendations.push(ExerciseSet {
                    exercise_id: "pushup".to_string(),
                    sets: 3,
                    reps: 12,
                    weight_kg: None,
                    duration_seconds: None,
                    rest_seconds: 45,
                    completed: false,
                });

                recommendations.push(ExerciseSet {
                    exercise_id: "burpee".to_string(),
                    sets: 2,
                    reps: 8,
                    weight_kg: None,
                    duration_seconds: None,
                    rest_seconds: 60,
                    completed: false,
                });

                recommendations.push(ExerciseSet {
                    exercise_id: "plank".to_string(),
                    sets: 3,
                    reps: 1,
                    weight_kg: None,
                    duration_seconds: Some(45),
                    rest_seconds: 45,
                    completed: false,
                });
            },
            
            _ => {
                recommendations.push(ExerciseSet {
                    exercise_id: "squat".to_string(),
                    sets: 4,
                    reps: 20,
                    weight_kg: None,
                    duration_seconds: None,
                    rest_seconds: 30,
                    completed: false,
                });

                recommendations.push(ExerciseSet {
                    exercise_id: "deadlift".to_string(),
                    sets: 4,
                    reps: 8,
                    weight_kg: Some(60.0),
                    duration_seconds: None,
                    rest_seconds: 90,
                    completed: false,
                });

                recommendations.push(ExerciseSet {
                    exercise_id: "burpee".to_string(),
                    sets: 3,
                    reps: 12,
                    weight_kg: None,
                    duration_seconds: None,
                    rest_seconds: 45,
                    completed: false,
                });

                recommendations.push(ExerciseSet {
                    exercise_id: "plank".to_string(),
                    sets: 3,
                    reps: 1,
                    weight_kg: None,
                    duration_seconds: Some(60),
                    rest_seconds: 30,
                    completed: false,
                });
            }
        }

        Ok(recommendations)
    }

    pub async fn analyze_progress(&self, user_id: &str) -> Result<ProgressAnalysis> {
        self.db.get_user_progress_analysis(user_id).await
    }

    pub async fn log_workout(&self, workout: WorkoutSession) -> Result<()> {
        self.db.save_workout(&workout).await
    }

    pub async fn get_exercise(&self, exercise_id: &str) -> Result<Option<Exercise>> {
        self.db.get_exercise(exercise_id).await
    }

    pub async fn get_all_exercises(&self) -> Result<Vec<Exercise>> {
        self.db.get_all_exercises().await
    }

    pub async fn get_user_workouts(&self, user_id: &str) -> Result<Vec<WorkoutSession>> {
        self.db.get_user_workouts(user_id).await
    }

    pub async fn database_health(&self) -> Result<database::DatabaseHealth> {
        self.db.health_check().await
    }
}

pub struct AppState {
    pub advisor: Arc<FitnessAdvisor>,
    pub ai_analyzer: Arc<AIMotionAnalyzer>,
    pub ml_client: Arc<MLServiceClient>,
    pub menu_optimizer: Arc<MenuOptimizer>,
    pub config: Arc<Config>,
}


pub async fn start_server(advisor: FitnessAdvisor, config: Config) -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let ml_client = MLServiceClient::with_config(
        config.ml_service.base_url.clone(),
        config.ml_service.timeout_seconds
    );
    
    info!("Checking ML service availability...");
    if ml_client.is_available().await {
        info!("ML service is available and ready");
    } else {
        warn!("ML service not available - starting without ML features");
        warn!("To enable ML features, start the Python ML service:");
        warn!("   python3 ml_service.py --host 127.0.0.1 --port 8001");
    }
    
    info!("Initializing menu optimizer with sample data...");
    let menu_optimizer = match DataLoader::load_sample_data().await {
        Ok(optimizer) => {
            let (food_count, recipe_count) = (
                optimizer.get_food_count().await,
                optimizer.get_recipe_count().await
            );
            info!("Menu optimizer loaded with {} foods and {} recipes", food_count, recipe_count);
            optimizer
        }
        Err(e) => {
            warn!("Failed to load sample data, using empty optimizer: {}", e);
            MenuOptimizer::new()
        }
    };
    
    let state = Arc::new(AppState {
        advisor: Arc::new(advisor),
        ai_analyzer: Arc::new(AIMotionAnalyzer::new()),
        ml_client: Arc::new(ml_client),
        menu_optimizer: Arc::new(menu_optimizer),
        config: Arc::new(config.clone()),
    });

    let app = api::create_router(state);

    let bind_address = config.get_server_address();
    let listener = tokio::net::TcpListener::bind(&bind_address).await?;
    info!("Fitness Advisor AI Server starting on http://{}", bind_address);
    info!("RTX 5070 Laptop GPU Ready for AI Processing");
    info!("SQLite Database Connected");
    info!("API Documentation:");
    info!("  POST   /api/users                          - Create user");
    info!("  GET    /api/users                          - Get all users");
    info!("  GET    /api/users/:id                      - Get specific user");
    info!("  GET    /api/users/:id/recommendations      - Get workout recommendations");
    info!("  GET    /api/users/:id/progress             - Get progress analysis");
    info!("  GET    /api/users/:id/workouts             - Get user workout history");
    info!("  GET    /api/exercises                      - Get all exercises");
    info!("  POST   /api/workouts                       - Log workout");
    info!("  POST   /api/ai/analyze-form                - AI form analysis (RTX 5070)");
    info!("  GET    /api/health                         - Health check");
    info!("  GET    /api/database/health                - Database health check");
    info!("  GET    /api/gpu-status                     - RTX 5070 status");

    axum::serve(listener, app).await?;
    Ok(())
}


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Fitness Advisor AI Starting...");
    
    // Load configuration
    let config = Config::load_with_env().unwrap_or_else(|e| {
        println!("Warning: Failed to load config file, using defaults: {}", e);
        Config::default()
    });
    
    // Validate configuration
    if let Err(e) = config.validate() {
        return Err(anyhow::anyhow!("Invalid configuration: {}", e));
    }
    
    println!("Configuration loaded successfully");
    println!("RTX 5070 Laptop GPU - 7.7GB VRAM Ready!");
    println!("Initializing SQLite Database...");
    
    // データベース接続
    let database_url = &config.database.url;
    
    // Ensure we can create the database file
    if let Err(e) = std::fs::File::create("fitness_advisor.db") {
        println!("Warning: Could not pre-create database file: {}", e);
    }
    
    let advisor = FitnessAdvisor::new(database_url).await?;
    
    let demo_user = User {
        id: "demo_user".to_string(),
        name: "Demo User".to_string(),
        age: 28,
        height: 175.0,
        weight: 70.0,
        fitness_level: FitnessLevel::Intermediate,
        goals: vec![FitnessGoal::Strength, FitnessGoal::GeneralHealth],
        preferences: models::user::UserPreferences {
            preferred_exercise_types: vec![ExerciseType::Strength],
            available_equipment: vec![Equipment::None, Equipment::Dumbbells],
            workout_duration_minutes: 45,
            workouts_per_week: 4,
            preferred_time_of_day: Some("evening".to_string()),
        },
    };

    advisor.register_user(demo_user.clone()).await?;
    
    let beginner_user = User {
        id: "beginner_user".to_string(),
        name: "Beginner User".to_string(),
        age: 25,
        height: 165.0,
        weight: 60.0,
        fitness_level: FitnessLevel::Beginner,
        goals: vec![FitnessGoal::GeneralHealth],
        preferences: models::user::UserPreferences {
            preferred_exercise_types: vec![ExerciseType::Strength, ExerciseType::Flexibility],
            available_equipment: vec![Equipment::None],
            workout_duration_minutes: 30,
            workouts_per_week: 3,
            preferred_time_of_day: Some("morning".to_string()),
        },
    };

    advisor.register_user(beginner_user).await?;

    let advanced_user = User {
        id: "advanced_user".to_string(),
        name: "Advanced User".to_string(),
        age: 35,
        height: 180.0,
        weight: 80.0,
        fitness_level: FitnessLevel::Advanced,
        goals: vec![FitnessGoal::Strength, FitnessGoal::MuscleGain],
        preferences: models::user::UserPreferences {
            preferred_exercise_types: vec![ExerciseType::Strength, ExerciseType::Cardio],
            available_equipment: vec![Equipment::Barbells, Equipment::Dumbbells, Equipment::Bench],
            workout_duration_minutes: 60,
            workouts_per_week: 5,
            preferred_time_of_day: Some("morning".to_string()),
        },
    };

    advisor.register_user(advanced_user).await?;

    let demo_workout = WorkoutSession {
        id: Uuid::new_v4().to_string(),
        user_id: "demo_user".to_string(),
        date: "2025-08-13".to_string(),
        exercises: advisor.recommend_workout("demo_user").await?,
        total_duration_minutes: 35,
        calories_burned: Some(180.0),
        user_rating: Some(4),
        notes: Some("Great workout! Felt strong today.".to_string()),
    };

    advisor.log_workout(demo_workout).await?;

    let demo_workout2 = WorkoutSession {
        id: Uuid::new_v4().to_string(),
        user_id: "demo_user".to_string(),
        date: "2025-08-12".to_string(),
        exercises: advisor.recommend_workout("demo_user").await?,
        total_duration_minutes: 40,
        calories_burned: Some(200.0),
        user_rating: Some(5),
        notes: Some("Perfect form today!".to_string()),
    };

    advisor.log_workout(demo_workout2).await?;

    let db_health = advisor.database_health().await?;
    println!("Database initialized successfully");
    println!("Users in database: {}", db_health.users_count);
    println!("Exercises in database: {}", db_health.exercises_count);
    println!("Workouts logged: {}", db_health.workouts_count);
    
    start_server(advisor, config).await?;
    
    Ok(())
}
