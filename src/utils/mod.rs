//! Utility Module
//! Logging, metrics, error handling utilities

pub mod logger;
pub mod metrics;
pub mod errors;

use std::time::Instant;
use tracing::info;

/// Timer utility for measuring execution time
pub struct Timer {
    start: Instant,
    name: String,
}

impl Timer {
    pub fn start(name: &str) -> Self {
        Self {
            start: Instant::now(),
            name: name.to_string(),
        }
    }

    pub fn stop(&self) -> Duration {
        Duration {
            name: self.name.clone(),
            elapsed_ms: self.start.elapsed().as_millis() as u64,
        }
    }
}

pub struct Duration {
    name: String,
    elapsed_ms: u64,
}

impl Duration {
    pub fn log(&self) {
        info!("⏱️ Timer [{}]: {}ms", self.name, self.elapsed_ms);
    }

    pub fn as_ms(&self) -> u64 {
        self.elapsed_ms
    }
}

/// Retry utility for async operations
pub async fn retry<F, Fut, T, E>(mut f: F, max_retries: u32) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
{
    let mut last_error = None;
    
    for attempt in 0..max_retries {
        match f().await {
            Ok(value) => return Ok(value),
            Err(e) => {
                last_error = Some(e);
                if attempt < max_retries - 1 {
                    let delay = std::time::Duration::from_millis(100 * 2u64.pow(attempt));
                    tokio::time::sleep(delay).await;
                }
            }
        }
    }
    
    Err(last_error.unwrap())
}

/// Parse size string (e.g., "16GB", "512MB") to bytes
pub fn parse_size(size_str: &str) -> Option<u64> {
    let size_str = size_str.trim().to_uppercase();
    
    let (num_str, unit) = if size_str.ends_with("GB") {
        (&size_str[..size_str.len()-2], 1024u64.pow(3))
    } else if size_str.ends_with("MB") {
        (&size_str[..size_str.len()-2], 1024u64.pow(2))
    } else if size_str.ends_with("KB") {
        (&size_str[..size_str.len()-2], 1024u64)
    } else if size_str.ends_with("B") {
        (&size_str[..size_str.len()-1], 1u64)
    } else {
        return None;
    };

    let num: u64 = num_str.parse().ok()?;
    Some(num * unit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_size() {
        assert_eq!(parse_size("16GB"), Some(16 * 1024 * 1024 * 1024));
        assert_eq!(parse_size("512MB"), Some(512 * 1024 * 1024));
        assert_eq!(parse_size("1KB"), Some(1024));
        assert_eq!(parse_size("100B"), Some(100));
        assert_eq!(parse_size("invalid"), None);
    }
}
