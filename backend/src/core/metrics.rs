// src/core/metrics.rs - System metrics and monitoring

use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub optimization_requests: u64,
    pub avg_optimization_time_ms: f64,
    pub successful_optimizations: u64,
    pub failed_optimizations: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub active_users: u64,
    pub total_meals_generated: u64,
    pub uptime_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationMetrics {
    pub algorithm_type: String,
    pub execution_time_ms: f64,
    pub iterations: u32,
    pub convergence_score: f64,
    pub constraint_violations: u32,
    pub solution_quality: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutritionMetrics {
    pub total_calories: f64,
    pub protein_g: f64,
    pub carbs_g: f64,
    pub fat_g: f64,
    pub fiber_g: f64,
    pub sugar_g: f64,
    pub sodium_mg: f64,
    pub macro_balance_score: f64,
    pub micronutrient_score: f64,
}

pub struct MetricsCollector {
    start_time: Instant,
    metrics: SystemMetrics,
    optimization_history: Vec<OptimizationMetrics>,
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            metrics: SystemMetrics {
                optimization_requests: 0,
                avg_optimization_time_ms: 0.0,
                successful_optimizations: 0,
                failed_optimizations: 0,
                cache_hits: 0,
                cache_misses: 0,
                active_users: 0,
                total_meals_generated: 0,
                uptime_seconds: 0,
            },
            optimization_history: Vec::new(),
        }
    }
    
    pub fn record_optimization_start(&mut self) {
        self.metrics.optimization_requests += 1;
    }
    
    pub fn record_optimization_success(&mut self, duration: Duration, opt_metrics: OptimizationMetrics) {
        self.metrics.successful_optimizations += 1;
        self.metrics.total_meals_generated += 1;
        
        // Update average optimization time
        let current_avg = self.metrics.avg_optimization_time_ms;
        let total_successful = self.metrics.successful_optimizations as f64;
        let new_time = duration.as_millis() as f64;
        
        self.metrics.avg_optimization_time_ms = 
            (current_avg * (total_successful - 1.0) + new_time) / total_successful;
        
        self.optimization_history.push(opt_metrics);
        
        // Keep only last 1000 optimization records
        if self.optimization_history.len() > 1000 {
            self.optimization_history.remove(0);
        }
    }
    
    pub fn record_optimization_failure(&mut self) {
        self.metrics.failed_optimizations += 1;
    }
    
    pub fn record_cache_hit(&mut self) {
        self.metrics.cache_hits += 1;
    }
    
    pub fn record_cache_miss(&mut self) {
        self.metrics.cache_misses += 1;
    }
    
    pub fn update_active_users(&mut self, count: u64) {
        self.metrics.active_users = count;
    }
    
    pub fn get_current_metrics(&self) -> SystemMetrics {
        let mut metrics = self.metrics.clone();
        metrics.uptime_seconds = self.start_time.elapsed().as_secs();
        metrics
    }
    
    pub fn get_optimization_stats(&self) -> HashMap<String, f64> {
        let mut stats = HashMap::new();
        
        if self.optimization_history.is_empty() {
            return stats;
        }
        
        // Calculate statistics from recent optimizations
        let recent: Vec<_> = self.optimization_history.iter().rev().take(100).collect();
        
        let avg_time: f64 = recent.iter().map(|o| o.execution_time_ms).sum::<f64>() / recent.len() as f64;
        let avg_iterations: f64 = recent.iter().map(|o| o.iterations as f64).sum::<f64>() / recent.len() as f64;
        let avg_convergence: f64 = recent.iter().map(|o| o.convergence_score).sum::<f64>() / recent.len() as f64;
        let avg_quality: f64 = recent.iter().map(|o| o.solution_quality).sum::<f64>() / recent.len() as f64;
        
        stats.insert("avg_execution_time_ms".to_string(), avg_time);
        stats.insert("avg_iterations".to_string(), avg_iterations);
        stats.insert("avg_convergence_score".to_string(), avg_convergence);
        stats.insert("avg_solution_quality".to_string(), avg_quality);
        stats.insert("total_optimizations".to_string(), self.optimization_history.len() as f64);
        
        stats
    }
    
    pub fn get_cache_hit_rate(&self) -> f64 {
        let total = self.metrics.cache_hits + self.metrics.cache_misses;
        if total == 0 {
            0.0
        } else {
            self.metrics.cache_hits as f64 / total as f64
        }
    }
    
    pub fn get_success_rate(&self) -> f64 {
        let total = self.metrics.successful_optimizations + self.metrics.failed_optimizations;
        if total == 0 {
            0.0
        } else {
            self.metrics.successful_optimizations as f64 / total as f64
        }
    }
}