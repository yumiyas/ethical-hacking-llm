//! Logging Configuration
//! Structured logging with tracing

use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer};
use std::str::FromStr;

/// Initialize logging system
pub fn init_logging() {
    // Parse log level from environment or default to INFO
    let log_level = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    
    // Console layer with pretty formatting
    let console_layer = fmt::Layer::new()
        .with_target(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_file(true)
        .with_line_number(true)
        .with_level(true)
        .with_ansi(true)
        .pretty();

    // File logging (JSON format for machine parsing)
    let file_appender = RollingFileAppender::new(
        Rotation::DAILY,
        "logs",
        "ethical-hacking-llm.log",
    );

    let file_layer = fmt::Layer::new()
        .with_writer(file_appender)
        .with_ansi(false)
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .json();

    // Environment filter
    let env_filter = EnvFilter::from_str(&log_level)
        .unwrap_or_else(|_| EnvFilter::from_str("info").unwrap());

    // Combine layers
    tracing_subscriber::registry()
        .with(env_filter)
        .with(console_layer)
        .with(file_layer)
        .init();

    tracing::info!("🚀 Logging initialized with level: {}", log_level);
}

/// Log HTTP request information
pub fn log_request(method: &str, path: &str, status: u16, duration_ms: u64) {
    tracing::info!(
        target: "http",
        method = method,
        path = path,
        status = status,
        duration_ms = duration_ms,
        "HTTP request completed"
    );
}

/// Log model inference information
pub fn log_inference(model: &str, tokens: usize, duration_ms: u64) {
    tracing::info!(
        target: "inference",
        model = model,
        tokens = tokens,
        duration_ms = duration_ms,
        "Model inference completed"
    );
}

/// Log security event
pub fn log_security(event: &str, ip: &str, details: &str) {
    tracing::warn!(
        target: "security",
        event = event,
        ip = ip,
        details = details,
        "Security event detected"
    );
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        tracing::error!($($arg)*);
    };
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        tracing::info!($($arg)*);
    };
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        tracing::debug!($($arg)*);
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        tracing::warn!($($arg)*);
    };
}
