//! Redis Cache Implementation

use super::{CacheTrait, CacheStats};
use anyhow::{Result, Context};
use async_trait::async_trait;
use redis::{AsyncCommands, Client, RedisError};
use std::time::Duration;
use tracing::{debug, warn};

pub struct RedisCache {
    client: Client,
    pool: deadpool::managed::Pool<RedisManager>,
    hits: std::sync::atomic::AtomicU64,
    misses: std::sync::atomic::AtomicU64,
}

struct RedisManager;

impl deadpool::managed::Manager for RedisManager {
    type Type = redis::aio::ConnectionManager;
    type Error = RedisError;

    async fn create(&self) -> Result<Self::Type, Self::Error> {
        // This will be set after client is created
        unimplemented!()
    }

    async fn recycle(&self, _: &mut Self::Type) -> deadpool::managed::RecycleResult<Self::Error> {
        Ok(())
    }
}

impl RedisCache {
    pub async fn new(redis_url: &str) -> Result<Self> {
        let client = Client::open(redis_url)
            .context("Failed to create Redis client")?;

        // Test connection
        let mut conn = client.get_async_connection().await
            .context("Failed to connect to Redis")?;
        
        redis::cmd("PING").query_async(&mut conn).await
            .context("Redis ping failed")?;

        Ok(Self {
            client,
            pool: deadpool::managed::Pool::new(RedisManager, 16),
            hits: std::sync::atomic::AtomicU64::new(0),
            misses: std::sync::atomic::AtomicU64::new(0),
        })
    }

    async fn get_connection(&self) -> Result<impl AsyncCommands> {
        Ok(self.client.get_async_connection().await?)
    }
}

#[async_trait]
impl CacheTrait for RedisCache {
    async fn get(&self, key: &str) -> Option<String> {
        match self.get_connection().await {
            Ok(mut conn) => {
                match conn.get(key).await {
                    Ok(value) => {
                        self.hits.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        value
                    }
                    Err(e) => {
                        warn!("Redis get error: {}", e);
                        self.misses.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        None
                    }
                }
            }
            Err(e) => {
                warn!("Redis connection error: {}", e);
                self.misses.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                None
            }
        }
    }

    async fn set(&self, key: String, value: String, ttl: Duration) {
        if let Ok(mut conn) = self.get_connection().await {
            let ttl_secs = ttl.as_secs() as usize;
            let _: Result<(), _> = conn.set_ex(key, value, ttl_secs).await;
        }
    }

    async fn delete(&self, key: &str) {
        if let Ok(mut conn) = self.get_connection().await {
            let _: Result<(), _> = conn.del(key).await;
        }
    }

    async fn exists(&self, key: &str) -> bool {
        if let Ok(mut conn) = self.get_connection().await {
            conn.exists(key).await.unwrap_or(false)
        } else {
            false
        }
    }

    async fn clear(&self) {
        if let Ok(mut conn) = self.get_connection().await {
            let _: Result<(), _> = redis::cmd("FLUSHDB").query_async(&mut conn).await;
        }
    }

    async fn stats(&self) -> CacheStats {
        let hits = self.hits.load(std::sync::atomic::Ordering::Relaxed);
        let misses = self.misses.load(std::sync::atomic::Ordering::Relaxed);
        
        let size = if let Ok(mut conn) = self.get_connection().await {
            conn.dbsize().await.unwrap_or(0)
        } else {
            0
        };

        CacheStats {
            hits,
            misses,
            size,
            memory_usage: 0, // Redis memory usage not easily available
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_redis_cache() {
        // This test requires Redis to be running
        let cache = RedisCache::new("redis://localhost:6379").await;
        assert!(cache.is_ok());
    }
}
