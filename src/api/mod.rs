
## 4. API Module

**src/api/mod.rs:**
```rust
//! API Module
//! HTTP server and routing for the LLM API

pub mod routes;
pub mod handlers;
pub mod middleware;

use axum::Router;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{info, error};
use crate::config::AppConfig;
use crate::utils::metrics;

/// Start the HTTP server
pub async fn start_server(config: Arc<AppConfig>) -> anyhow::Result<()> {
    let addr = SocketAddr::new(
        config.server.host.parse()?,
        config.server.port,
    );

    // Build router with all routes
    let app = Router::new()
        .merge(routes::query_routes())
        .merge(routes::health_routes())
        .merge(routes::metrics_routes())
        .merge(routes::model_routes())
        .layer(middleware::cors_middleware())
        .layer(middleware::compression_middleware())
        .layer(middleware::trace_middleware())
        .layer(axum::middleware::from_fn(middleware::request_logger))
        .with_state(config);

    info!("🚀 Server listening on http://{}", addr);
    info!("📚 API documentation available at http://{}/docs", addr);
    
    // Start server with graceful shutdown
    let listener = TcpListener::bind(addr).await?;
    
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

/// Graceful shutdown signal handler
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("🛑 Shutting down gracefully (Ctrl+C)...");
        },
        _ = terminate => {
            info!("🛑 Shutting down gracefully (SIGTERM)...");
        },
    }

    // Flush metrics
    metrics::flush_metrics();
}
