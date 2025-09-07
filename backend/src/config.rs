// src/config.rs - Configuration management for fitness advisor

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub ml_service: MLServiceConfig,
    pub logging: LoggingConfig,
    pub ai_analysis: AIAnalysisConfig,
    pub fitness: FitnessConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub cors_origins: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub connection_timeout_seconds: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MLServiceConfig {
    pub base_url: String,
    pub timeout_seconds: u64,
    pub retry_attempts: u32,
    pub health_check_interval_seconds: u64,
    pub endpoints: MLEndpoints,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MLEndpoints {
    pub health: String,
    pub analyze_frame: String,
    pub analyze_video: String,
    pub analyze_batch: String,
    pub models_status: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
    pub file_enabled: bool,
    pub file_path: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AIAnalysisConfig {
    pub realtime_max_latency_ms: u32,
    pub realtime_confidence_threshold: f64,
    pub batch_sample_rate: u32,
    pub batch_timeout_minutes: u32,
    pub form_thresholds: FormThresholds,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FormThresholds {
    pub squat: SquatThresholds,
    pub pushup: PushupThresholds,
    pub plank: PlankThresholds,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SquatThresholds {
    pub knee_angle_min: f64,
    pub knee_angle_max: f64,
    pub back_straightness: f64,
    pub hip_knee_alignment: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PushupThresholds {
    pub arm_angle_min: f64,
    pub arm_angle_max: f64,
    pub body_straightness: f64,
    pub shoulder_stability: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PlankThresholds {
    pub body_straightness: f64,
    pub hip_position: f64,
    pub shoulder_alignment: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FitnessConfig {
    pub default_workout_duration_minutes: u32,
    pub default_workouts_per_week: u32,
    pub bmr_multipliers: HashMap<String, f64>,
    pub macro_ratios: MacroRatios,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MacroRatios {
    pub muscle_gain: MacroRatio,
    pub weight_loss: MacroRatio,
    pub maintenance: MacroRatio,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MacroRatio {
    pub protein: f64,
    pub fat: f64,
    pub carbs: f64,
}

impl Config {
    /// Load configuration from TOML file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        
        if !path.exists() {
            return Err(anyhow!("Configuration file not found: {}", path.display()));
        }

        let content = std::fs::read_to_string(path)
            .map_err(|e| anyhow!("Failed to read config file {}: {}", path.display(), e))?;

        let config: Config = toml::from_str(&content)
            .map_err(|e| anyhow!("Failed to parse config file {}: {}", path.display(), e))?;

        Ok(config)
    }

    /// Load configuration with environment variable overrides
    pub fn load_with_env() -> Result<Self> {
        // Try to load from default location
        let config_path = std::env::var("FITNESS_CONFIG_PATH")
            .unwrap_or_else(|_| "config/default.toml".to_string());

        let mut config = Self::load_from_file(&config_path)?;

        // Override with environment variables
        config.apply_env_overrides();

        Ok(config)
    }

    /// Apply environment variable overrides
    fn apply_env_overrides(&mut self) {
        // Server overrides
        if let Ok(host) = std::env::var("FITNESS_SERVER_HOST") {
            self.server.host = host;
        }
        if let Ok(port) = std::env::var("FITNESS_SERVER_PORT") {
            if let Ok(port) = port.parse() {
                self.server.port = port;
            }
        }

        // Database overrides
        if let Ok(db_url) = std::env::var("FITNESS_DATABASE_URL") {
            self.database.url = db_url;
        }

        // ML service overrides
        if let Ok(ml_url) = std::env::var("FITNESS_ML_SERVICE_URL") {
            self.ml_service.base_url = ml_url;
        }
        if let Ok(timeout) = std::env::var("FITNESS_ML_TIMEOUT_SECONDS") {
            if let Ok(timeout) = timeout.parse() {
                self.ml_service.timeout_seconds = timeout;
            }
        }

        // Logging overrides
        if let Ok(log_level) = std::env::var("FITNESS_LOG_LEVEL") {
            self.logging.level = log_level;
        }
    }

    /// Get database URL with fallback
    pub fn get_database_url(&self) -> &str {
        &self.database.url
    }

    /// Get ML service base URL
    pub fn get_ml_service_url(&self) -> &str {
        &self.ml_service.base_url
    }

    /// Get server bind address
    pub fn get_server_address(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        // Validate server config
        if self.server.port == 0 {
            return Err(anyhow!("Invalid server port: {}", self.server.port));
        }

        // Validate ML service URL
        if self.ml_service.base_url.is_empty() {
            return Err(anyhow!("ML service base URL is empty"));
        }

        // Validate macro ratios sum to 1.0
        let muscle_gain_sum = self.fitness.macro_ratios.muscle_gain.protein +
                              self.fitness.macro_ratios.muscle_gain.fat +
                              self.fitness.macro_ratios.muscle_gain.carbs;
        
        if (muscle_gain_sum - 1.0).abs() > 0.01 {
            return Err(anyhow!("Muscle gain macro ratios do not sum to 1.0: {}", muscle_gain_sum));
        }

        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 3000,
                cors_origins: vec!["http://localhost:3000".to_string()],
            },
            database: DatabaseConfig {
                url: "sqlite:./fitness_advisor.db".to_string(),
                max_connections: 10,
                connection_timeout_seconds: 30,
            },
            ml_service: MLServiceConfig {
                base_url: "http://127.0.0.1:8001".to_string(),
                timeout_seconds: 30,
                retry_attempts: 3,
                health_check_interval_seconds: 60,
                endpoints: MLEndpoints {
                    health: "/health".to_string(),
                    analyze_frame: "/analyze/frame".to_string(),
                    analyze_video: "/analyze/video".to_string(),
                    analyze_batch: "/analyze/batch".to_string(),
                    models_status: "/models/status".to_string(),
                },
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                format: "json".to_string(),
                file_enabled: true,
                file_path: "./logs/fitness_advisor.log".to_string(),
            },
            ai_analysis: AIAnalysisConfig {
                realtime_max_latency_ms: 50,
                realtime_confidence_threshold: 0.7,
                batch_sample_rate: 30,
                batch_timeout_minutes: 5,
                form_thresholds: FormThresholds {
                    squat: SquatThresholds {
                        knee_angle_min: 70.0,
                        knee_angle_max: 120.0,
                        back_straightness: 0.15,
                        hip_knee_alignment: 0.1,
                    },
                    pushup: PushupThresholds {
                        arm_angle_min: 45.0,
                        arm_angle_max: 90.0,
                        body_straightness: 0.1,
                        shoulder_stability: 0.05,
                    },
                    plank: PlankThresholds {
                        body_straightness: 0.08,
                        hip_position: 0.1,
                        shoulder_alignment: 0.05,
                    },
                },
            },
            fitness: FitnessConfig {
                default_workout_duration_minutes: 45,
                default_workouts_per_week: 3,
                bmr_multipliers: {
                    let mut map = HashMap::new();
                    map.insert("sedentary".to_string(), 1.2);
                    map.insert("lightly_active".to_string(), 1.375);
                    map.insert("moderately_active".to_string(), 1.55);
                    map.insert("very_active".to_string(), 1.725);
                    map.insert("extra_active".to_string(), 1.9);
                    map
                },
                macro_ratios: MacroRatios {
                    muscle_gain: MacroRatio {
                        protein: 0.30,
                        fat: 0.25,
                        carbs: 0.45,
                    },
                    weight_loss: MacroRatio {
                        protein: 0.40,
                        fat: 0.30,
                        carbs: 0.30,
                    },
                    maintenance: MacroRatio {
                        protein: 0.30,
                        fat: 0.30,
                        carbs: 0.40,
                    },
                },
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.server.port, 3000);
        assert_eq!(config.database.url, "sqlite:./fitness_advisor.db");
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let serialized = toml::to_string(&config).unwrap();
        let deserialized: Config = toml::from_str(&serialized).unwrap();
        
        assert_eq!(config.server.port, deserialized.server.port);
        assert_eq!(config.ml_service.base_url, deserialized.ml_service.base_url);
    }

    #[test]
    fn test_config_from_file() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("test_config.toml");
        
        let config_content = r#"
[server]
host = "127.0.0.1"
port = 8080
cors_origins = ["http://test.com"]

[database]
url = "sqlite::memory:"
max_connections = 5
connection_timeout_seconds = 10

[ml_service]
base_url = "http://test-ml-service:8001"
timeout_seconds = 60
retry_attempts = 5
health_check_interval_seconds = 30

[ml_service.endpoints]
health = "/health"
analyze_frame = "/analyze/frame"
analyze_video = "/analyze/video"
analyze_batch = "/analyze/batch"
models_status = "/models/status"

[logging]
level = "debug"
format = "text"
file_enabled = false
file_path = "/tmp/test.log"

[ai_analysis]
realtime_max_latency_ms = 30
realtime_confidence_threshold = 0.8
batch_sample_rate = 15
batch_timeout_minutes = 10

[ai_analysis.form_thresholds.squat]
knee_angle_min = 60.0
knee_angle_max = 130.0
back_straightness = 0.2
hip_knee_alignment = 0.15

[ai_analysis.form_thresholds.pushup]
arm_angle_min = 40.0
arm_angle_max = 95.0
body_straightness = 0.12
shoulder_stability = 0.06

[ai_analysis.form_thresholds.plank]
body_straightness = 0.09
hip_position = 0.12
shoulder_alignment = 0.06

[fitness]
default_workout_duration_minutes = 60
default_workouts_per_week = 4

[fitness.bmr_multipliers]
sedentary = 1.2
lightly_active = 1.375
moderately_active = 1.55
very_active = 1.725
extra_active = 1.9

[fitness.macro_ratios.muscle_gain]
protein = 0.35
fat = 0.25
carbs = 0.40

[fitness.macro_ratios.weight_loss]
protein = 0.45
fat = 0.30
carbs = 0.25

[fitness.macro_ratios.maintenance]
protein = 0.30
fat = 0.30
carbs = 0.40
        "#;
        
        fs::write(&config_path, config_content).unwrap();
        
        let config = Config::load_from_file(&config_path).unwrap();
        
        assert_eq!(config.server.host, "127.0.0.1");
        assert_eq!(config.server.port, 8080);
        assert_eq!(config.database.url, "sqlite::memory:");
        assert_eq!(config.ml_service.base_url, "http://test-ml-service:8001");
        assert_eq!(config.logging.level, "debug");
        assert_eq!(config.ai_analysis.realtime_max_latency_ms, 30);
        assert_eq!(config.fitness.default_workout_duration_minutes, 60);
    }
}