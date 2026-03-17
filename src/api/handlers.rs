//! API Request Handlers

use axum::{
    extract::{Path, State},
    response::{IntoResponse, Json, Response, sse::{Event, Sse}},
    http::StatusCode,
    Json as JsonExtractor,
};
use axum::response::sse::KeepAlive;
use futures::stream::Stream;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::sync::Arc;
use std::time::Instant;
use tokio_stream::StreamExt;
use tracing::{info, warn, error};
use crate::config::AppConfig;
use crate::model::ModelTrait;
use crate::security::validator::InputValidator;
use crate::utils::metrics;

#[derive(Debug, Deserialize)]
pub struct QueryRequest {
    pub query: String,
    #[serde(default = "default_max_tokens")]
    pub max_tokens: usize,
    #[serde(default = "default_temperature")]
    pub temperature: f32,
    #[serde(default)]
    pub stream: bool,
    #[serde(default)]
    pub model: Option<String>,
}

fn default_max_tokens() -> usize { 100 }
fn default_temperature() -> f32 { 0.7 }

#[derive(Debug, Serialize)]
pub struct QueryResponse {
    pub response: String,
    pub processing_time_ms: u64,
    pub model_used: String,
    pub cached: bool,
    pub tokens_used: usize,
    pub timestamp: String,
}

#[derive(Debug, Serialize)]
pub struct ModelInfo {
    pub name: String,
    pub model_type: String,
    pub loaded: bool,
    pub stats: ModelStats,
}

#[derive(Debug, Serialize)]
pub struct ModelStats {
    pub total_queries: u64,
    pub avg_latency_ms: f64,
    pub memory_usage_mb: u64,
}

/// Handle regular query requests
pub async fn handle_query(
    State(config): State<Arc<AppConfig>>,
    JsonExtractor(req): JsonExtractor<QueryRequest>,
) -> impl IntoResponse {
    let start = Instant::now();
    metrics::record_request_start();

    // Validate input
    if let Err(e) = InputValidator::validate(&req.query) {
        metrics::record_request_end(start.elapsed().as_secs_f64() * 1000.0, 400);
        return (
            StatusCode::BAD_REQUEST,
            Json(QueryResponse {
                response: format!("Invalid input: {}", e),
                processing_time_ms: start.elapsed().as_millis() as u64,
                model_used: "none".to_string(),
                cached: false,
                tokens_used: 0,
                timestamp: chrono::Utc::now().to_rfc3339(),
            }),
        );
    }

    // TODO: Implement actual model inference
    // For now, return mock response
    let response = format!("Response to: {}", req.query);
    let elapsed = start.elapsed().as_millis() as u64;

    metrics::record_request_end(elapsed as f64, 200);
    metrics::record_query(req.query.len() as u64, elapsed);

    info!("Query processed in {}ms", elapsed);

    (
        StatusCode::OK,
        Json(QueryResponse {
            response,
            processing_time_ms: elapsed,
            model_used: "quantized".to_string(),
            cached: false,
            tokens_used: req.max_tokens,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }),
    )
}

/// Handle streaming query requests
pub async fn handle_stream_query(
    State(config): State<Arc<AppConfig>>,
    JsonExtractor(req): JsonExtractor<QueryRequest>,
) -> Response {
    let stream = tokio_stream::iter(vec![
        Ok::<_, Infallible>(Event::default().data("Hello")),
        Ok(Event::default().data("World")),
    ]);

    Sse::new(stream)
        .keep_alive(KeepAlive::default())
        .into_response()
}

/// Health check endpoint
pub async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "version": env!("CARGO_PKG_VERSION"),
        "timestamp": chrono::Utc::now().to_rfc3339(),
    }))
}

/// Readiness check endpoint
pub async fn readiness_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "ready",
        "models_loaded": true,
        "cache_available": true,
    }))
}

/// Liveness check endpoint
pub async fn liveness_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "alive",
        "uptime_seconds": 0, // TODO: Track uptime
    }))
}

/// List available models
pub async fn list_models() -> Json<Vec<ModelInfo>> {
    Json(vec![
        ModelInfo {
            name: "phi-2-q4".to_string(),
            model_type: "quantized".to_string(),
            loaded: true,
            stats: ModelStats {
                total_queries: 1234,
                avg_latency_ms: 45.6,
                memory_usage_mb: 612,
            },
        },
        ModelInfo {
            name: "tinyllama".to_string(),
            model_type: "local".to_string(),
            loaded: false,
            stats: ModelStats {
                total_queries: 0,
                avg_latency_ms: 0.0,
                memory_usage_mb: 0,
            },
        },
    ])
}

/// Get model information
pub async fn get_model_info(Path(name): Path<String>) -> impl IntoResponse {
    Json(serde_json::json!({
        "name": name,
        "type": "quantized",
        "size_mb": 612,
        "quantization": "4-bit",
        "context_length": 2048,
    }))
}

/// Load a model
pub async fn load_model(Path(name): Path<String>) -> impl IntoResponse {
    info!("Loading model: {}", name);
    StatusCode::OK
}

/// Unload a model
pub async fn unload_model(Path(name): Path<String>) -> impl IntoResponse {
    info!("Unloading model: {}", name);
    StatusCode::OK
}

/// Get metrics
pub async fn get_metrics() -> String {
    metrics::gather_metrics()
}

/// Get statistics
pub async fn get_stats() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "total_queries": metrics::get_total_queries(),
        "avg_latency_ms": metrics::get_avg_latency(),
        "cache_hits": metrics::get_cache_hits(),
        "cache_misses": metrics::get_cache_misses(),
        "active_requests": metrics::get_active_requests(),
    }))
}

/// Get API documentation
pub async fn get_documentation() -> impl IntoResponse {
    (StatusCode::OK, "API Documentation")
}

/// Get OpenAPI specification
pub async fn get_openapi_spec() -> impl IntoResponse {
    (StatusCode::OK, Json(serde_json::json!({
        "openapi": "3.0.0",
        "info": {
            "title": "Ethical Hacking LLM API",
            "version": env!("CARGO_PKG_VERSION"),
        },
    })))
}
