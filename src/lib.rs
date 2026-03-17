//! Ethical Hacking LLM Library
//! 
//! This library provides the core functionality for the Ethical Hacking LLM,
//! including model inference, knowledge base access, caching, and security.

pub mod api;
pub mod cache;
pub mod config;
pub mod hacking_knowledge;
pub mod model;
pub mod security;
pub mod utils;

// Re-export commonly used items
pub use config::AppConfig;
pub use model::ModelTrait;
pub use security::SecurityError;

use anyhow::Result;
use std::sync::Arc;

/// Initialize the library with default configuration
pub async fn init() -> Result<Arc<AppConfig>> {
    let config = Arc::new(AppConfig::from_env()?);
    
    if config.cache.enabled {
        cache::init_cache(&config).await?;
    }
    
    hacking_knowledge::init_knowledge_base().await?;
    security::init_security(&config).await?;
    
    Ok(config)
}

/// Version constant
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
