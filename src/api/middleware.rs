//! API Middleware
//! CORS, compression, logging, and other middleware

use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use tower_http::{
    cors::{CorsLayer, Any},
    compression::CompressionLayer,
    trace::TraceLayer,
    limit::RequestBodyLimitLayer,
};
use std::time::Instant;
use tracing::info;

/// CORS middleware configuration
pub fn cors_middleware() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_credentials(true)
        .max_age(std::time::Duration::from_secs(3600))
}

/// Compression middleware
pub fn compression_middleware() -> CompressionLayer {
    CompressionLayer::new()
        .gzip(true)
        .br(true)
        .deflate(true)
        .quality(tower_http::compression::CompressionLevel::Default)
}

/// Tracing middleware
pub fn trace_middleware() -> TraceLayer {
    TraceLayer::new_for_http()
}

/// Request body size limiter (10MB max)
pub fn request_limit_middleware() -> RequestBodyLimitLayer {
    RequestBodyLimitLayer::new(10 * 1024 * 1024) // 10MB
}

/// Request logger middleware
pub async fn request_logger<B>(
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let start = Instant::now();
    let method = req.method().clone();
    let uri = req.uri().clone();
    let headers = req.headers().clone();
    
    // Extract client IP
    let client_ip = headers
        .get("x-forwarded-for")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("unknown")
        .to_string();

    let response = next.run(req).await;

    let latency = start.elapsed();
    let status = response.status();

    info!(
        target: "http",
        method = %method,
        path = %uri,
        status = %status,
        latency_ms = latency.as_millis(),
        client_ip = %client_ip,
        "HTTP request processed"
    );

    Ok(response)
}

/// Authentication middleware
pub async fn auth_middleware<B>(
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    // Check for API key in headers
    let auth_header = req.headers()
        .get("authorization")
        .and_then(|h| h.to_str().ok());

    match auth_header {
        Some(token) if token.starts_with("Bearer ") => {
            // Validate token
            let token = &token[7..];
            if token == std::env::var("API_KEY").unwrap_or_default() {
                Ok(next.run(req).await)
            } else {
                Err(StatusCode::UNAUTHORIZED)
            }
        }
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}

/// Rate limiting middleware
pub async fn rate_limit_middleware<B>(
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    // TODO: Implement rate limiting with Redis
    Ok(next.run(req).await)
}

/// Error handling middleware
pub async fn error_handler_middleware<B>(
    req: Request<B>,
    next: Next<B>,
) -> Response {
    let response = next.run(req).await;
    
    if response.status().is_server_error() {
        tracing::error!("Server error: {:?}", response);
    }
    
    response
}
