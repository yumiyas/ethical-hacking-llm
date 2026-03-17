
**src/security/rate_limiter.rs:**
```rust
//! Rate Limiting
//! Prevent abuse with rate limiting

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use tracing::warn;

struct RateLimitEntry {
    count: u32,
    reset_at: Instant,
}

pub struct RateLimiter {
    requests: Arc<Mutex<HashMap<String, RateLimitEntry>>>,
    max_requests: u32,
    window_secs: u64,
    whitelist: Vec<String>,
}

impl RateLimiter {
    pub fn new(max_requests: u32, window_secs: u64) -> Self {
        Self {
            requests: Arc::new(Mutex::new(HashMap::new())),
            max_requests,
            window_secs,
            whitelist: Vec::new(),
        }
    }

    pub async fn check_rate_limit(&self, key: &str) -> bool {
        // Check whitelist
        if self.whitelist.contains(&key.to_string()) {
            return true;
        }

        let mut requests = self.requests.lock().await;
        let now = Instant::now();

        if let Some(entry) = requests.get_mut(key) {
            if now < entry.reset_at {
                if entry.count >= self.max_requests {
                    warn!("Rate limit exceeded for key: {}", key);
                    return false;
                }
                entry.count += 1;
            } else {
                // Reset window
                entry.count = 1;
                entry.reset_at = now + Duration::from_secs(self.window_secs);
            }
        } else {
            // First request
            requests.insert(key.to_string(), RateLimitEntry {
                count: 1,
                reset_at: now + Duration::from_secs(self.window_secs),
            });
        }

        true
    }

    pub async fn get_remaining(&self, key: &str) -> u32 {
        if self.whitelist.contains(&key.to_string()) {
            return u32::MAX;
        }

        let requests = self.requests.lock().await;
        
        if let Some(entry) = requests.get(key) {
            if Instant::now() < entry.reset_at {
                return self.max_requests.saturating_sub(entry.count);
            }
        }
        
        self.max_requests
    }

    pub async fn reset(&self, key: &str) {
        let mut requests = self.requests.lock().await;
        requests.remove(key);
    }

    pub fn add_to_whitelist(&mut self, key: String) {
        self.whitelist.push(key);
    }

    pub async fn get_stats(&self) -> serde_json::Value {
        let requests = self.requests.lock().await;
        let total_requests: u32 = requests.values().map(|e| e.count).sum();
        
        serde_json::json!({
            "total_tracked_keys": requests.len(),
            "total_requests": total_requests,
            "max_requests_per_window": self.max_requests,
            "window_seconds": self.window_secs,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rate_limiter() {
        let limiter = RateLimiter::new(3, 60);
        
        // First 3 requests should succeed
        assert!(limiter.check_rate_limit("test").await);
        assert!(limiter.check_rate_limit("test").await);
        assert!(limiter.check_rate_limit("test").await);
        
        // 4th request should fail
        assert!(!limiter.check_rate_limit("test").await);
        
        // Check remaining
        assert_eq!(limiter.get_remaining("test").await, 0);
    }
}
