// src/lib.rs - Leptos frontend library

#[cfg(feature = "ssr")]
pub mod database;
#[cfg(feature = "ssr")]
pub mod ml_client;
#[cfg(feature = "ssr")]
pub mod config;
#[cfg(feature = "ssr")]
pub mod core;
#[cfg(feature = "ssr")]
pub mod models;
#[cfg(feature = "ssr")]
pub mod advisors;
#[cfg(feature = "ssr")]
pub mod sample_data;
#[cfg(feature = "ssr")]
pub mod api;
#[cfg(feature = "ssr")]
pub mod ai_analytics;
#[cfg(feature = "ssr")]
pub mod websocket;

pub mod frontend;

pub use frontend::*;