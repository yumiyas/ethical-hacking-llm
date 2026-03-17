//! Ethical Hacking LLM - Main Entry Point
//! High-performance Language Model for Ethical Hacking

mod api;
mod cache;
mod config;
mod hacking_knowledge;
mod model;
mod security;
mod utils;

use anyhow::Result;
use config::AppConfig;
use std::sync::Arc;
use tracing::{error, info};
use utils::{logger, metrics};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging and telemetry
    logger::init_logging();
    metrics::init_metrics();

    info!("🚀 Starting Ethical Hacking LLM Server");
    info!("Version: {}", env!("CARGO_PKG_VERSION"));

    // Load configuration
    let config = match AppConfig::from_env() {
        Ok(cfg) => {
            info!("✅ Configuration loaded successfully");
            info!("   Server: {}:{}", cfg.server.host, cfg.server.port);
            info!("   Model: {} ({})", cfg.model.path, cfg.model.model_type);
            info!("   Cache: {}", if cfg.cache.enabled { "enabled" } else { "disabled" });
            Arc::new(cfg)
        }
        Err(e) => {
            error!("❌ Failed to load configuration: {}", e);
            std::process::exit(1);
        }
    };

    // Initialize components
    if let Err(e) = initialize_components(&config).await {
        error!("❌ Failed to initialize components: {}", e);
        std::process::exit(1);
    }

    // Start server
    info!("📡 Starting API server...");
    if let Err(e) = api::start_server(config).await {
        error!("❌ Server error: {}", e);
        std::process::exit(1);
    }

    Ok(())
}

async fn initialize_components(config: &AppConfig) -> Result<()> {
    info!("🔄 Initializing components...");

    // Initialize cache
    if config.cache.enabled {
        cache::init_cache(config).await?;
        info!("✅ Cache initialized");
    }

    // Initialize knowledge base
    hacking_knowledge::init_knowledge_base().await?;
    info!("✅ Knowledge base initialized");

    // Initialize security components
    security::init_security(config).await?;
    info!("✅ Security components initialized");

    Ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!env!("CARGO_PKG_VERSION").is_empty());
    }
}
