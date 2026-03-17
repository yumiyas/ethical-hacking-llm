//! API Routes Definition

use axum::{
    Router,
    routing::{get, post},
};
use crate::api::handlers;

/// Query-related routes
pub fn query_routes() -> Router {
    Router::new()
        .route("/query", post(handlers::handle_query))
        .route("/query/stream", post(handlers::handle_stream_query))
}

/// Health check routes
pub fn health_routes() -> Router {
    Router::new()
        .route("/health", get(handlers::health_check))
        .route("/ready", get(handlers::readiness_check))
        .route("/live", get(handlers::liveness_check))
}

/// Metrics routes
pub fn metrics_routes() -> Router {
    Router::new()
        .route("/metrics", get(handlers::get_metrics))
        .route("/stats", get(handlers::get_stats))
}

/// Model management routes
pub fn model_routes() -> Router {
    Router::new()
        .route("/models", get(handlers::list_models))
        .route("/models/:name", get(handlers::get_model_info))
        .route("/models/:name/load", post(handlers::load_model))
        .route("/models/:name/unload", post(handlers::unload_model))
}

/// Documentation routes
pub fn docs_routes() -> Router {
    Router::new()
        .route("/docs", get(handlers::get_documentation))
        .route("/openapi.json", get(handlers::get_openapi_spec))
}
