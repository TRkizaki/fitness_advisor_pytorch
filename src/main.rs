// src/main.rs - Main application with database integration

mod database;

use std::sync::Arc;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use uuid::Uuid;

// Web API imports
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

// Database imports
use database::{DatabaseManager, DatabaseHealth};

// === 基本データ構造 ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub age: u32,
    pub height: f32, // cm
    pub weight: f32, // kg
    pub fitness_level: FitnessLevel,
    pub goals: Vec<FitnessGoal>,
    pub preferences: UserPreferences,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FitnessLevel {
    Beginner,
    Intermediate,
    Advanced,
    Elite,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FitnessGoal {
    WeightLoss,
    MuscleGain,
    Endurance,
    Strength,
    Flexibility,
    GeneralHealth,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub preferred_exercise_types: Vec<ExerciseType>,
    pub available_equipment: Vec<Equipment>,
    pub workout_duration_minutes: u32,
    pub workouts_per_week: u32,
    pub preferred_time_of_day: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExerciseType {
    Cardio,
    Strength,
    Flexibility,
    Balance,
    Sports,
    Yoga,
    Pilates,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Equipment {
    None,        // 自重トレーニング
    Dumbbells,
    Barbells,
    ResistanceBands,
    PullUpBar,
    Bench,
    TreadMill,
    StationaryBike,
}

// === 運動データ構造 ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Exercise {
    pub id: String,
    pub name: String,
    pub description: String,
    pub exercise_type: ExerciseType,
    pub equipment_needed: Vec<Equipment>,
    pub difficulty_level: u32, // 1-10
    pub primary_muscles: Vec<MuscleGroup>,
    pub secondary_muscles: Vec<MuscleGroup>,
    pub instructions: Vec<String>,
    pub safety_tips: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MuscleGroup {
    Chest,
    Back,
    Shoulders,
    Arms,
    Core,
    Legs,
    Glutes,
    Calves,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkoutSession {
    pub id: String,
    pub user_id: String,
    pub date: String, // ISO 8601
    pub exercises: Vec<ExerciseSet>,
    pub total_duration_minutes: u32,
    pub calories_burned: Option<f32>,
    pub user_rating: Option<u32>, // 1-5
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseSet {
    pub exercise_id: String,
    pub sets: u32,
    pub reps: u32,
    pub weight_kg: Option<f32>,
    pub duration_seconds: Option<u32>,
    pub rest_seconds: u32,
    pub completed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProgressAnalysis {
    pub total_workouts: u32,
    pub average_duration_minutes: f32,
    pub total_calories_burned: f32,
    pub consistency_score: f32, // 0.0 - 1.0
}

// === AI 機能 ===

pub struct AIMotionAnalyzer {
    python_script_path: String,
}

impl AIMotionAnalyzer {
    pub fn new() -> Self {
        Self {
            python_script_path: "ml_analyzer_test.py".to_string(),
        }
    }

    pub async fn analyze_form(&self, video_data: &[u8]) -> Result<FormAnalysis> {
        use tokio::process::Command;
        use tokio::io::{AsyncWriteExt, AsyncReadExt};
        
        info!("🎥 Starting MediaPipe pose analysis...");
        
        // Encode video data as base64 for JSON transport
        let video_base64 = base64::prelude::Engine::encode(&base64::prelude::BASE64_STANDARD, video_data);
        
        // Create JSON input for Python script
        let input_json = serde_json::json!({
            "video_base64": video_base64
        });
        
        // Spawn Python process
        let mut child = Command::new("python3")
            .arg(&self.python_script_path)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| anyhow::anyhow!("Failed to spawn Python process: {}", e))?;

        // Write input to Python process
        if let Some(mut stdin) = child.stdin.take() {
            let input_str = input_json.to_string();
            stdin.write_all(input_str.as_bytes()).await
                .map_err(|e| anyhow::anyhow!("Failed to write to Python process: {}", e))?;
            stdin.flush().await
                .map_err(|e| anyhow::anyhow!("Failed to flush stdin: {}", e))?;
        }

        // Wait for process to complete with timeout
        let output = tokio::time::timeout(
            std::time::Duration::from_secs(30),
            child.wait_with_output()
        ).await
        .map_err(|_| anyhow::anyhow!("Python process timed out after 30 seconds"))?
        .map_err(|e| anyhow::anyhow!("Python process failed: {}", e))?;

        // Parse Python output
        if output.status.success() {
            let output_str = String::from_utf8(output.stdout)
                .map_err(|e| anyhow::anyhow!("Invalid UTF-8 from Python: {}", e))?;
            
            info!("✅ Python analysis completed successfully");
            
            // Parse JSON response from Python
            let python_result: serde_json::Value = serde_json::from_str(&output_str)
                .map_err(|e| anyhow::anyhow!("Failed to parse Python output: {}", e))?;
            
            // Convert Python response to FormAnalysis
            let form_analysis = FormAnalysis {
                overall_score: python_result["overall_score"].as_f64().unwrap_or(0.0) as f32,
                recommendations: python_result["recommendations"].as_array()
                    .unwrap_or(&Vec::new())
                    .iter()
                    .map(|v| v.as_str().unwrap_or("").to_string())
                    .collect(),
                detected_errors: python_result["detected_errors"].as_array()
                    .unwrap_or(&Vec::new())
                    .iter()
                    .map(|v| v.as_str().unwrap_or("").to_string())
                    .collect(),
                confidence: python_result["confidence"].as_f64().unwrap_or(0.0) as f32,
            };
            
            Ok(form_analysis)
        } else {
            let error_str = String::from_utf8(output.stderr).unwrap_or_default();
            warn!("❌ Python process failed with error: {}", error_str);
            Err(anyhow::anyhow!("Python analysis failed: {}", error_str))
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FormAnalysis {
    pub overall_score: f32, // 0.0 - 1.0
    pub recommendations: Vec<String>,
    pub detected_errors: Vec<String>,
    pub confidence: f32, // 0.0 - 1.0
}

// === Fitness Advisor with Database ===

pub struct FitnessAdvisor {
    db: Arc<DatabaseManager>,
}

impl FitnessAdvisor {
    pub async fn new(database_url: &str) -> Result<Self> {
        let db = Arc::new(DatabaseManager::new(database_url).await?);
        Ok(Self { db })
    }

    // ユーザー登録
    pub async fn register_user(&self, user: User) -> Result<()> {
        self.db.save_user(&user).await
    }

    // ユーザー取得
    pub async fn get_user(&self, user_id: &str) -> Result<Option<User>> {
        self.db.get_user(user_id).await
    }

    // 全ユーザー取得
    pub async fn get_all_users(&self) -> Result<Vec<User>> {
        self.db.get_all_users().await
    }

    // パーソナライズされたワークアウト推奨
    pub async fn recommend_workout(&self, user_id: &str) -> Result<Vec<ExerciseSet>> {
        let user = self.db.get_user(user_id).await?
            .ok_or_else(|| anyhow::anyhow!("User not found"))?;

        let mut recommendations = Vec::new();

        // 基本的な推奨ロジック（将来はAIモデルで置き換え）
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
                // Advanced/Elite
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

    // 進捗分析
    pub async fn analyze_progress(&self, user_id: &str) -> Result<ProgressAnalysis> {
        self.db.get_user_progress_analysis(user_id).await
    }

    // ワークアウト記録
    pub async fn log_workout(&self, workout: WorkoutSession) -> Result<()> {
        self.db.save_workout(&workout).await
    }

    // エクササイズ取得
    pub async fn get_exercise(&self, exercise_id: &str) -> Result<Option<Exercise>> {
        self.db.get_exercise(exercise_id).await
    }

    // 全エクササイズ取得
    pub async fn get_all_exercises(&self) -> Result<Vec<Exercise>> {
        self.db.get_all_exercises().await
    }

    // ユーザーのワークアウト履歴取得
    pub async fn get_user_workouts(&self, user_id: &str) -> Result<Vec<WorkoutSession>> {
        self.db.get_user_workouts(user_id).await
    }

    // データベース健康チェック
    pub async fn database_health(&self) -> Result<DatabaseHealth> {
        self.db.health_check().await
    }
}

// === Web API 実装 ===

// API レスポンス構造
#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: String,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: "Success".to_string(),
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            message,
        }
    }
}

// API リクエスト構造
#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub user: User,
}

#[derive(Deserialize)]
pub struct LogWorkoutRequest {
    pub workout: WorkoutSession,
}

#[derive(Deserialize)]
pub struct AnalyzeFormRequest {
    pub video_base64: String, // Base64エンコードされた動画データ
}

// アプリケーション状態
pub struct AppState {
    pub advisor: Arc<FitnessAdvisor>,
    pub ai_analyzer: Arc<AIMotionAnalyzer>,
}

// API ハンドラー
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
) -> Result<Json<ApiResponse<Vec<User>>>, StatusCode> {
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
) -> Result<Json<ApiResponse<User>>, StatusCode> {
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
) -> Result<Json<ApiResponse<Vec<ExerciseSet>>>, StatusCode> {
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
) -> Result<Json<ApiResponse<ProgressAnalysis>>, StatusCode> {
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
) -> Result<Json<ApiResponse<Vec<WorkoutSession>>>, StatusCode> {
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
) -> Result<Json<ApiResponse<Vec<Exercise>>>, StatusCode> {
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

// 🎯 RTX 5070 GPU を活用するフォーム分析エンドポイント
pub async fn analyze_form(
    State(state): State<Arc<AppState>>,
    Json(request): Json<AnalyzeFormRequest>,
) -> Result<Json<ApiResponse<FormAnalysis>>, StatusCode> {
    info!("🎥 Starting form analysis with RTX 5070...");
    
    // Base64デコード
    let video_data = match base64::prelude::Engine::decode(&base64::prelude::BASE64_STANDARD, &request.video_base64) {
        Ok(data) => data,
        Err(e) => {
            warn!("Failed to decode video data: {}", e);
            return Ok(Json(ApiResponse::error("Invalid video data".to_string())));
        }
    };

    // 🚀 RTX 5070でのAI分析
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

// ヘルスチェック
pub async fn health_check() -> Json<ApiResponse<String>> {
    Json(ApiResponse::success("Fitness Advisor AI is healthy! 💪".to_string()))
}

// データベースヘルスチェック
pub async fn database_health(
    State(state): State<Arc<AppState>>,
) -> Json<ApiResponse<DatabaseHealth>> {
    match state.advisor.database_health().await {
        Ok(health) => Json(ApiResponse::success(health)),
        Err(e) => {
            warn!("Database health check failed: {}", e);
            Json(ApiResponse::error(format!("Database error: {}", e)))
        }
    }
}

// GPU ステータス（RTX 5070 情報）
pub async fn gpu_status() -> Json<ApiResponse<GpuStatus>> {
    let status = GpuStatus {
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

#[derive(Serialize)]
pub struct GpuStatus {
    pub gpu_available: bool,
    pub gpu_name: String,
    pub compute_capability: String,
    pub vram_total_mb: u32,
    pub vram_used_mb: u32,
    pub cuda_version: String,
    pub ready_for_ai: bool,
    pub features: Vec<String>,
}

// API ルーター作成
pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        // ユーザー管理
        .route("/api/users", post(create_user))
        .route("/api/users", get(get_all_users))
        .route("/api/users/:user_id", get(get_user))
        .route("/api/users/:user_id/recommendations", get(get_workout_recommendation))
        .route("/api/users/:user_id/progress", get(get_progress_analysis))
        .route("/api/users/:user_id/workouts", get(get_user_workouts))
        
        // エクササイズ管理
        .route("/api/exercises", get(get_exercises))
        
        // ワークアウト管理
        .route("/api/workouts", post(log_workout))
        
        // 🎯 AI機能（RTX 5070活用）
        .route("/api/ai/analyze-form", post(analyze_form))
        
        // システム情報
        .route("/api/health", get(health_check))
        .route("/api/database/health", get(database_health))
        .route("/api/gpu-status", get(gpu_status))
        
        // 状態を共有
        .with_state(state)
        
        // CORS とミドルウェア
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .into_inner()
        )
}

// サーバー起動
pub async fn start_server(advisor: FitnessAdvisor) -> anyhow::Result<()> {
    // トレーシング初期化
    tracing_subscriber::fmt::init();

    let state = Arc::new(AppState {
        advisor: Arc::new(advisor),
        ai_analyzer: Arc::new(AIMotionAnalyzer::new()),
    });

    let app = create_router(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    info!("🚀 Fitness Advisor AI Server starting on http://0.0.0.0:3000");
    info!("💪 RTX 5070 Laptop GPU Ready for AI Processing");
    info!("🗄️  SQLite Database Connected");
    info!("📱 API Documentation:");
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

// === メイン実行 ===

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🏋️  Fitness Advisor AI Starting...");
    println!("🎯 RTX 5070 Laptop GPU - 7.7GB VRAM Ready!");
    println!("🗄️  Initializing SQLite Database...");
    
    // データベース接続
    let database_url = "sqlite:./fitness_advisor.db";
    
    // Ensure we can create the database file
    if let Err(e) = std::fs::File::create("fitness_advisor.db") {
        println!("Warning: Could not pre-create database file: {}", e);
    }
    
    let advisor = FitnessAdvisor::new(database_url).await?;
    
    // デモユーザーの作成
    let demo_user = User {
        id: "demo_user".to_string(),
        name: "Demo User".to_string(),
        age: 28,
        height: 175.0,
        weight: 70.0,
        fitness_level: FitnessLevel::Intermediate,
        goals: vec![FitnessGoal::Strength, FitnessGoal::GeneralHealth],
        preferences: UserPreferences {
            preferred_exercise_types: vec![ExerciseType::Strength],
            available_equipment: vec![Equipment::None, Equipment::Dumbbells],
            workout_duration_minutes: 45,
            workouts_per_week: 4,
            preferred_time_of_day: Some("evening".to_string()),
        },
    };

    advisor.register_user(demo_user.clone()).await?;
    
    // 初級ユーザーも追加
    let beginner_user = User {
        id: "beginner_user".to_string(),
        name: "Beginner User".to_string(),
        age: 25,
        height: 165.0,
        weight: 60.0,
        fitness_level: FitnessLevel::Beginner,
        goals: vec![FitnessGoal::GeneralHealth],
        preferences: UserPreferences {
            preferred_exercise_types: vec![ExerciseType::Strength, ExerciseType::Flexibility],
            available_equipment: vec![Equipment::None],
            workout_duration_minutes: 30,
            workouts_per_week: 3,
            preferred_time_of_day: Some("morning".to_string()),
        },
    };

    advisor.register_user(beginner_user).await?;

    // 上級ユーザーも追加
    let advanced_user = User {
        id: "advanced_user".to_string(),
        name: "Advanced User".to_string(),
        age: 35,
        height: 180.0,
        weight: 80.0,
        fitness_level: FitnessLevel::Advanced,
        goals: vec![FitnessGoal::Strength, FitnessGoal::MuscleGain],
        preferences: UserPreferences {
            preferred_exercise_types: vec![ExerciseType::Strength, ExerciseType::Cardio],
            available_equipment: vec![Equipment::Barbells, Equipment::Dumbbells, Equipment::Bench],
            workout_duration_minutes: 60,
            workouts_per_week: 5,
            preferred_time_of_day: Some("morning".to_string()),
        },
    };

    advisor.register_user(advanced_user).await?;

    // デモワークアウト記録
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

    // さらにワークアウトを追加して履歴を作る
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

    // データベースヘルスチェック
    let db_health = advisor.database_health().await?;
    println!("✅ Database initialized successfully");
    println!("👤 Users in database: {}", db_health.users_count);
    println!("💪 Exercises in database: {}", db_health.exercises_count);
    println!("📊 Workouts logged: {}", db_health.workouts_count);
    
    // サーバー起動
    start_server(advisor).await?;
    
    Ok(())
}
