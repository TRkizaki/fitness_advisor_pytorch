use std::sync::Arc;
use serde::Deserialize;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::{info, warn};

use crate::{
    AppState, ApiResponse, FitnessGoal,
    models::optimization,
};

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub user: crate::User,
}

#[derive(Deserialize)]
pub struct LogWorkoutRequest {
    pub workout: crate::WorkoutSession,
}

#[derive(Deserialize)]
pub struct AnalyzeFormRequest {
    pub video_base64: String,
}

#[derive(Deserialize)]
pub struct AnalyzeFrameRequest {
    pub frame_base64: String,
}

#[derive(Deserialize)]
pub struct AnalyzeVideoRequest {
    pub video_base64: String,
}

#[derive(Debug, Deserialize)]
pub struct MLBatchRequest {
    pub video_path: String,
}

#[derive(Debug, Deserialize)]
pub struct OptimizeMealPlanRequest {
    pub user_id: String,
    pub goals: Vec<FitnessGoal>,
    pub time_horizon_days: u32,
    pub preferences: Option<optimization::UserPreferences>,
    pub objectives: Option<Vec<optimization::OptimizationObjective>>,
}

pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateUserRequest>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    match state.advisor.register_user(request.user.clone()).await {
        Ok(_) => {
            info!("User {} registered successfully", request.user.id);
            Ok(Json(ApiResponse::success(format!(
                "User {} registered successfully", 
                request.user.id
            ))))
        }
        Err(e) => {
            warn!("Failed to register user: {}", e);
            Ok(Json(ApiResponse::error(format!("Registration failed: {}", e))))
        }
    }
}

pub async fn get_all_users(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<Vec<crate::User>>>, StatusCode> {
    match state.advisor.get_all_users().await {
        Ok(users) => {
            info!("Retrieved {} users", users.len());
            Ok(Json(ApiResponse::success(users)))
        }
        Err(e) => {
            warn!("Failed to get users: {}", e);
            Ok(Json(ApiResponse::error(format!("Failed to get users: {}", e))))
        }
    }
}

pub async fn get_user(
    Path(user_id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<crate::User>>, StatusCode> {
    match state.advisor.get_user(&user_id).await {
        Ok(Some(user)) => {
            info!("Retrieved user {}", user_id);
            Ok(Json(ApiResponse::success(user)))
        }
        Ok(None) => {
            warn!("User {} not found", user_id);
            Ok(Json(ApiResponse::error("User not found".to_string())))
        }
        Err(e) => {
            warn!("Failed to get user {}: {}", user_id, e);
            Ok(Json(ApiResponse::error(format!("Failed to get user: {}", e))))
        }
    }
}

pub async fn get_workout_recommendation(
    Path(user_id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<Vec<crate::ExerciseSet>>>, StatusCode> {
    match state.advisor.recommend_workout(&user_id).await {
        Ok(recommendations) => {
            info!("Generated workout recommendation for user {}", user_id);
            Ok(Json(ApiResponse::success(recommendations)))
        }
        Err(e) => {
            warn!("Failed to generate recommendation for user {}: {}", user_id, e);
            Ok(Json(ApiResponse::error(format!("Recommendation failed: {}", e))))
        }
    }
}

pub async fn log_workout(
    State(state): State<Arc<AppState>>,
    Json(request): Json<LogWorkoutRequest>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    match state.advisor.log_workout(request.workout.clone()).await {
        Ok(_) => {
            info!("Workout logged for user {}", request.workout.user_id);
            Ok(Json(ApiResponse::success("Workout logged successfully".to_string())))
        }
        Err(e) => {
            warn!("Failed to log workout: {}", e);
            Ok(Json(ApiResponse::error(format!("Logging failed: {}", e))))
        }
    }
}

pub async fn get_progress_analysis(
    Path(user_id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<crate::ProgressAnalysis>>, StatusCode> {
    match state.advisor.analyze_progress(&user_id).await {
        Ok(analysis) => {
            info!("Generated progress analysis for user {}", user_id);
            Ok(Json(ApiResponse::success(analysis)))
        }
        Err(e) => {
            warn!("Failed to analyze progress for user {}: {}", user_id, e);
            Ok(Json(ApiResponse::error(format!("Analysis failed: {}", e))))
        }
    }
}

pub async fn get_user_workouts(
    Path(user_id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<Vec<crate::WorkoutSession>>>, StatusCode> {
    match state.advisor.get_user_workouts(&user_id).await {
        Ok(workouts) => {
            info!("Retrieved {} workouts for user {}", workouts.len(), user_id);
            Ok(Json(ApiResponse::success(workouts)))
        }
        Err(e) => {
            warn!("Failed to get workouts for user {}: {}", user_id, e);
            Ok(Json(ApiResponse::error(format!("Failed to get workouts: {}", e))))
        }
    }
}

pub async fn get_exercises(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<Vec<crate::Exercise>>>, StatusCode> {
    match state.advisor.get_all_exercises().await {
        Ok(exercises) => {
            info!("Retrieved {} exercises", exercises.len());
            Ok(Json(ApiResponse::success(exercises)))
        }
        Err(e) => {
            warn!("Failed to get exercises: {}", e);
            Ok(Json(ApiResponse::error(format!("Failed to get exercises: {}", e))))
        }
    }
}

pub async fn analyze_form(
    State(state): State<Arc<AppState>>,
    Json(request): Json<AnalyzeFormRequest>,
) -> Result<Json<ApiResponse<crate::FormAnalysis>>, StatusCode> {
    info!("🎥 Starting form analysis with RTX 5070...");
    
    let video_data = match base64::prelude::Engine::decode(&base64::prelude::BASE64_STANDARD, &request.video_base64) {
        Ok(data) => data,
        Err(e) => {
            warn!("Failed to decode video data: {}", e);
            return Ok(Json(ApiResponse::error("Invalid video data".to_string())));
        }
    };

    match state.ai_analyzer.analyze_form(&video_data).await {
        Ok(analysis) => {
            info!("✅ Form analysis completed using RTX 5070");
            Ok(Json(ApiResponse::success(analysis)))
        }
        Err(e) => {
            warn!("❌ Form analysis failed: {}", e);
            Ok(Json(ApiResponse::error(format!("Analysis failed: {}", e))))
        }
    }
}

pub async fn health_check() -> Json<ApiResponse<String>> {
    Json(ApiResponse::success("Fitness Advisor AI is healthy! 💪".to_string()))
}

pub async fn database_health(
    State(state): State<Arc<AppState>>,
) -> Json<ApiResponse<crate::database::DatabaseHealth>> {
    match state.advisor.database_health().await {
        Ok(health) => Json(ApiResponse::success(health)),
        Err(e) => {
            warn!("Database health check failed: {}", e);
            Json(ApiResponse::error(format!("Database error: {}", e)))
        }
    }
}

pub async fn gpu_status() -> Json<ApiResponse<crate::GpuStatus>> {
    let status = crate::GpuStatus {
        gpu_available: true,
        gpu_name: "NVIDIA GeForce RTX 5070 Laptop GPU".to_string(),
        compute_capability: "12.0".to_string(),
        vram_total_mb: 7716,
        vram_used_mb: 72,
        cuda_version: "12.4".to_string(),
        ready_for_ai: true,
        features: vec![
            "Real-time pose estimation".to_string(),
            "Form analysis".to_string(),
            "Motion tracking".to_string(),
            "AI workout recommendations".to_string(),
            "Database-backed analytics".to_string(),
        ],
    };
    
    Json(ApiResponse::success(status))
}

pub async fn ml_analyze_frame(
    State(state): State<Arc<AppState>>,
    Json(request): Json<AnalyzeFrameRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    match state.ml_client.analyze_frame_realtime(request.frame_base64).await {
        Ok(response) => {
            if response.success {
                info!("ML frame analysis completed in {:.2}ms", response.processing_time_ms);
                Ok(Json(ApiResponse::success(response.result)))
            } else {
                warn!("ML frame analysis failed: {:?}", response.error);
                Ok(Json(ApiResponse::error(
                    response.error.unwrap_or("Analysis failed".to_string())
                )))
            }
        }
        Err(e) => {
            warn!("ML service request failed: {}", e);
            Ok(Json(ApiResponse::error(format!("ML service unavailable: {}", e))))
        }
    }
}

pub async fn ml_analyze_video(
    State(state): State<Arc<AppState>>,
    Json(request): Json<AnalyzeVideoRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    match state.ml_client.analyze_video(request.video_base64, "detailed").await {
        Ok(response) => {
            if response.success {
                info!("ML video analysis completed in {:.2}ms", response.processing_time_ms);
                Ok(Json(ApiResponse::success(response.result)))
            } else {
                warn!("ML video analysis failed: {:?}", response.error);
                Ok(Json(ApiResponse::error(
                    response.error.unwrap_or("Analysis failed".to_string())
                )))
            }
        }
        Err(e) => {
            warn!("ML service request failed: {}", e);
            Ok(Json(ApiResponse::error(format!("ML service unavailable: {}", e))))
        }
    }
}

pub async fn ml_analyze_batch(
    State(state): State<Arc<AppState>>,
    Json(request): Json<MLBatchRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    match state.ml_client.analyze_batch(request.video_path).await {
        Ok(response) => {
            if response.success {
                info!("ML batch analysis completed in {:.2}ms", response.processing_time_ms);
                Ok(Json(ApiResponse::success(response.result)))
            } else {
                warn!("ML batch analysis failed: {:?}", response.error);
                Ok(Json(ApiResponse::error(
                    response.error.unwrap_or("Analysis failed".to_string())
                )))
            }
        }
        Err(e) => {
            warn!("ML service request failed: {}", e);
            Ok(Json(ApiResponse::error(format!("ML service unavailable: {}", e))))
        }
    }
}

pub async fn ml_service_status(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    match state.ml_client.models_status().await {
        Ok(status) => {
            info!("ML service status retrieved successfully");
            Ok(Json(ApiResponse::success(serde_json::to_value(status).unwrap())))
        }
        Err(e) => {
            warn!("Failed to get ML service status: {}", e);
            Ok(Json(ApiResponse::error(format!("ML service unavailable: {}", e))))
        }
    }
}

pub async fn optimize_meal_plan(
    State(state): State<Arc<AppState>>,
    Json(request): Json<OptimizeMealPlanRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    let user_result = state.advisor.get_user(&request.user_id).await;
    let user = match user_result {
        Ok(Some(user)) => user,
        Ok(None) => {
            warn!("User not found for meal plan optimization: {}", request.user_id);
            return Ok(Json(ApiResponse::error("User not found".to_string())));
        }
        Err(e) => {
            warn!("Failed to get user {}: {}", request.user_id, e);
            return Ok(Json(ApiResponse::error(format!("Database error: {}", e))));
        }
    };

    let constraints = match state.menu_optimizer.generate_nutrition_constraints(&user, &request.goals).await {
        Ok(constraints) => constraints,
        Err(e) => {
            warn!("Failed to generate nutrition constraints for user {}: {}", request.user_id, e);
            return Ok(Json(ApiResponse::error(format!("Constraint generation failed: {}", e))));
        }
    };

    let preferences = request.preferences.unwrap_or_else(|| optimization::UserPreferences {
        dietary_restrictions: vec![],
        allergens_to_avoid: vec![],
        cuisine_preferences: vec!["American".to_string(), "Italian".to_string()],
        disliked_foods: vec![],
        preferred_foods: vec![],
        taste_preferences: optimization::TastePreferences {
            sweetness_preference: 0.0,
            saltiness_preference: 0.0,
            sourness_preference: 0.0,
            bitterness_preference: 0.0,
            umami_preference: 0.0,
            spiciness_preference: 0.0,
            spice_tolerance: 0.5,
        },
        cooking_skill_level: optimization::CookingSkillLevel::Intermediate,
        equipment_available: vec![
            optimization::CookingEquipment::Stovetop,
            optimization::CookingEquipment::Oven,
            optimization::CookingEquipment::Microwave,
        ],
        meal_variety_importance: 0.7,
        cost_importance: 0.5,
        health_importance: 0.8,
        convenience_importance: 0.6,
    });

    let objectives = request.objectives.unwrap_or_else(|| vec![
        optimization::OptimizationObjective::MaximizeNutrition,
        optimization::OptimizationObjective::MaximizeTasteScore,
        optimization::OptimizationObjective::BalanceMacros,
        optimization::OptimizationObjective::MaximizeVariety,
    ]);

    let opt_request = optimization::OptimizationRequest {
        user_id: request.user_id.clone(),
        constraints,
        preferences,
        objectives,
        time_horizon_days: request.time_horizon_days,
        algorithm_config: optimization::AlgorithmConfig::default(),
    };

    match state.menu_optimizer.optimize_meal_plan(opt_request).await {
        Ok(solution) => {
            info!("Menu optimization completed for user {}", request.user_id);
            Ok(Json(ApiResponse::success(serde_json::to_value(solution).unwrap())))
        }
        Err(e) => {
            warn!("Menu optimization failed for user {}: {}", request.user_id, e);
            Ok(Json(ApiResponse::error(format!("Optimization failed: {}", e))))
        }
    }
}

pub async fn menu_optimizer_status(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    let metrics = state.menu_optimizer.get_metrics().await;
    let (cache_size, hit_rate) = state.menu_optimizer.get_cache_stats().await;
    
    let recipe_count = state.menu_optimizer.get_recipe_count().await;
    let food_count = state.menu_optimizer.get_food_count().await;

    let status = serde_json::json!({
        "service": "Menu Optimizer",
        "status": "healthy",
        "metrics": metrics,
        "cache": {
            "size": cache_size,
            "hit_rate": hit_rate
        },
        "data": {
            "recipes": recipe_count,
            "foods": food_count
        }
    });

    Ok(Json(ApiResponse::success(status)))
}

pub async fn get_menu_recommendations(
    Path(user_id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<Vec<String>>>, StatusCode> {
    match state.menu_optimizer.get_optimization_recommendations(&user_id).await {
        Ok(recommendations) => {
            info!("Retrieved menu recommendations for user {}", user_id);
            Ok(Json(ApiResponse::success(recommendations)))
        }
        Err(e) => {
            warn!("Failed to get menu recommendations for user {}: {}", user_id, e);
            Ok(Json(ApiResponse::error(format!("Failed to get recommendations: {}", e))))
        }
    }
}

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/users", post(create_user))
        .route("/api/users", get(get_all_users))
        .route("/api/users/:user_id", get(get_user))
        .route("/api/users/:user_id/recommendations", get(get_workout_recommendation))
        .route("/api/users/:user_id/progress", get(get_progress_analysis))
        .route("/api/users/:user_id/workouts", get(get_user_workouts))
        
        .route("/api/exercises", get(get_exercises))
        
        .route("/api/workouts", post(log_workout))
        
        .route("/api/ai/analyze-form", post(analyze_form))
        .route("/api/ai/realtime", get(crate::websocket::websocket_handler))
        
        .route("/api/ml/analyze-frame", post(ml_analyze_frame))
        .route("/api/ml/analyze-video", post(ml_analyze_video))
        .route("/api/ml/analyze-batch", post(ml_analyze_batch))
        .route("/api/ml/status", get(ml_service_status))
        
        .route("/api/menu/optimize", post(optimize_meal_plan))
        .route("/api/menu/status", get(menu_optimizer_status))
        .route("/api/menu/recommendations/:user_id", get(get_menu_recommendations))
        
        .route("/api/health", get(health_check))
        .route("/api/database/health", get(database_health))
        .route("/api/gpu-status", get(gpu_status))
        
        .with_state(state)
        
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .into_inner()
        )
}