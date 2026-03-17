//! Cache Module
//! Caching layer for LLM responses to improve performance

pub mod redis_cache;
pub mod memory_cache;

use anyhow::Result;
use async_trait::async_trait;
use once_cell::sync::OnceCell;
use std::time::Duration;
use tracing::info;

static GLOBAL_CACHE: OnceCell<Box<dyn CacheTrait + Send + Sync>> = OnceCell::new();

/// Cache trait for different cache implementations
#[async_trait]
pub trait CacheTrait: Send + Sync {
    /// Get value from cache
    async fn get(&self, key: &str) -> Option<String>;
    
    /// Set value in cache with TTL
    async fn set(&self, key: String, value: String, ttl: Duration);
    
    /// Delete value from cache
    async fn delete(&self, key: &str);
    
    /// Check if key exists
    async fn exists(&self, key: &str) -> bool;
    
    /// Clear all cache
    async fn clear(&self);
    
    /// Get cache statistics
    async fn stats(&self) -> CacheStats;
}

/// Cache statistics
#[derive(Debug, Clone, Default)]
pub struct CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub size: usize,
    pub memory_usage: u64,
}

/// Response cache for LLM responses
pub struct ResponseCache {
    inner: &'static (dyn CacheTrait + Send + Sync),
}

impl ResponseCache {
    /// Get global cache instance
    pub fn global() -> &'static Self {
        static INSTANCE: OnceCell<ResponseCache> = OnceCell::new();
        INSTANCE.get_or_init(|| {
            let inner = GLOBAL_CACHE.get().expect("Cache not initialized");
            ResponseCache { inner }
        })
    }

    /// Get cached response
    pub async fn get(&self, key: &str) -> Option<String> {
        self.inner.get(key).await
    }

    /// Insert response into cache
    pub async fn insert(&self, key: String, value: String) {
        self.inner.set(key, value, Duration::from_secs(3600)).await;
    }

    /// Insert with custom TTL
    pub async fn insert_with_ttl(&self, key: String, value: String, ttl: Duration) {
        self.inner.set(key, value, ttl).await;
    }

    /// Delete from cache
    pub async fn delete(&self, key: &str) {
        self.inner.delete(key).await;
    }

    /// Check if key exists
    pub async fn exists(&self, key: &str) -> bool {
        self.inner.exists(key).await
    }

    /// Clear cache
    pub async fn clear(&self) {
        self.inner.clear().await;
    }

    /// Get cache key for query
    pub fn cache_key(query: &str, max_tokens: usize, temperature: f32) -> String {
        format!("{}:{}:{}", query, max_tokens, temperature)
    }
}

/// Initialize global cache
pub async fn init_cache(config: &crate::config::AppConfig) -> Result<()> {
    info!("Initializing cache...");

    let cache: Box<dyn CacheTrait + Send + Sync> = if let Some(redis_url) = &config.cache.redis_url {
        info!("Using Redis cache at {}", redis_url);
        Box::new(redis_cache::RedisCache::new(redis_url).await?)
    } else {
        info!("Using in-memory cache with max size: {}", config.cache.max_size);
        Box::new(memory_cache::MemoryCache::new(config.cache.max_size))
    };

    GLOBAL_CACHE.set(cache)
        .map_err(|_| anyhow::anyhow!("Cache already initialized"))?;

    info!("✅ Cache initialized");
    Ok(())
}

/// Cache key generator
pub mod key {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    pub fn generate_key(query: &str, params: &[(&str, &str)]) -> String {
        let mut hasher = DefaultHasher::new();
        query.hash(&mut hasher);
        for (k, v) in params {
            k.hash(&mut hasher);
            v.hash(&mut hasher);
        }
        format!("{:x}", hasher.finish())
    }
}
