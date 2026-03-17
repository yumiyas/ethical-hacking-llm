//! Security Module
//! Input validation, rate limiting, audit logging

pub mod validator;
pub mod rate_limiter;
pub mod audit;

use anyhow::Result;
use once_cell::sync::Lazy;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

static SECURITY_MANAGER: Lazy<Arc<RwLock<SecurityManager>>> = Lazy::new(|| {
    Arc::new(RwLock::new(SecurityManager::new()))
});

#[derive(Debug, thiserror::Error)]
pub enum SecurityError {
    #[error("Input too long (max {max} characters)")]
    TooLong { max: usize },
    
    #[error("Blocked pattern detected")]
    BlockedPattern,
    
    #[error("Invalid characters in input")]
    InvalidChars,
    
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
    
    #[error("Unauthorized access")]
    Unauthorized,
    
    #[error("Invalid API key")]
    InvalidApiKey,
}

pub struct SecurityManager {
    rate_limiter: Option<Arc<rate_limiter::RateLimiter>>,
    auditor: Option<Arc<audit::AuditLogger>>,
    validator: validator::InputValidator,
}

impl SecurityManager {
    fn new() -> Self {
        Self {
            rate_limiter: None,
            auditor: None,
            validator: validator::InputValidator::new(),
        }
    }

    pub async fn init_rate_limiter(&mut self, max_requests: u32, window_secs: u64) {
        self.rate_limiter = Some(Arc::new(rate_limiter::RateLimiter::new(max_requests, window_secs)));
    }

    pub async fn init_audit(&mut self, log_file: &str) -> Result<()> {
        self.auditor = Some(Arc::new(audit::AuditLogger::new(log_file)?));
        Ok(())
    }
}

pub async fn init_security(config: &crate::config::AppConfig) -> Result<()> {
    info!("Initializing security components...");

    let mut manager = SECURITY_MANAGER.write().await;
    
    // Initialize rate limiter
    manager.init_rate_limiter(config.security.rate_limit_per_minute, 60).await;
    
    // Initialize audit logger
    manager.init_audit("logs/audit.log").await?;
    
    info!("✅ Security components initialized");
    Ok(())
}

pub async fn get_security_manager() -> Arc<RwLock<SecurityManager>> {
    SECURITY_MANAGER.clone()
}
