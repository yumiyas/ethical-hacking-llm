//! In-Memory Cache Implementation

use super::{CacheTrait, CacheStats};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::debug;

struct CacheEntry {
    value: String,
    expires_at: Instant,
    size: usize,
}

pub struct MemoryCache {
    cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
    max_size: usize,
    hits: std::sync::atomic::AtomicU64,
    misses: std::sync::atomic::AtomicU64,
    current_size: Arc<std::sync::atomic::AtomicUsize>,
}

impl MemoryCache {
    pub fn new(max_size: usize) -> Self {
        let cache = Arc::new(RwLock::new(HashMap::new()));
        let current_size = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        
        // Spawn cleanup task
        let cache_clone = cache.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(60));
            loop {
                interval.tick().await;
                Self::cleanup_expired(&cache_clone).await;
            }
        });

        Self {
            cache,
            max_size,
            hits: std::sync::atomic::AtomicU64::new(0),
            misses: std::sync::atomic::AtomicU64::new(0),
            current_size,
        }
    }

    async fn cleanup_expired(cache: &Arc<RwLock<HashMap<String, CacheEntry>>>) {
        let mut cache = cache.write().await;
        let now = Instant::now();
        cache.retain(|_, entry| now < entry.expires_at);
        debug!("Cache cleanup completed, size: {}", cache.len());
    }

    fn estimate_size(value: &str) -> usize {
        value.len() + std::mem::size_of::<CacheEntry>()
    }
}

#[async_trait]
impl CacheTrait for MemoryCache {
    async fn get(&self, key: &str) -> Option<String> {
        let cache = self.cache.read().await;
        
        if let Some(entry) = cache.get(key) {
            if Instant::now() < entry.expires_at {
                self.hits.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                return Some(entry.value.clone());
            }
        }
        
        self.misses.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        None
    }

    async fn set(&self, key: String, value: String, ttl: Duration) {
        let mut cache = self.cache.write().await;
        
        // Check if we need to evict
        let entry_size = Self::estimate_size(&value);
        
        if cache.len() >= self.max_size {
            // Simple LRU eviction - remove oldest entries
            let mut entries: Vec<_> = cache.iter().collect();
            entries.sort_by_key(|(_, entry)| entry.expires_at);
            
            let to_remove = entries.len() - self.max_size + 1;
            for (key, _) in entries.iter().take(to_remove) {
                if let Some(removed) = cache.remove(*key) {
                    self.current_size.fetch_sub(removed.size, std::sync::atomic::Ordering::Relaxed);
                }
            }
        }

        let entry = CacheEntry {
            value: value.clone(),
            expires_at: Instant::now() + ttl,
            size: entry_size,
        };

        self.current_size.fetch_add(entry_size, std::sync::atomic::Ordering::Relaxed);
        cache.insert(key, entry);
    }

    async fn delete(&self, key: &str) {
        let mut cache = self.cache.write().await;
        if let Some(removed) = cache.remove(key) {
            self.current_size.fetch_sub(removed.size, std::sync::atomic::Ordering::Relaxed);
        }
    }

    async fn exists(&self, key: &str) -> bool {
        let cache = self.cache.read().await;
        cache.contains_key(key)
    }

    async fn clear(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
        self.current_size.store(0, std::sync::atomic::Ordering::Relaxed);
    }

    async fn stats(&self) -> CacheStats {
        let cache = self.cache.read().await;
        CacheStats {
            hits: self.hits.load(std::sync::atomic::Ordering::Relaxed),
            misses: self.misses.load(std::sync::atomic::Ordering::Relaxed),
            size: cache.len(),
            memory_usage: self.current_size.load(std::sync::atomic::Ordering::Relaxed) as u64,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_memory_cache() {
        let cache = MemoryCache::new(10);
        
        cache.set("key1".to_string(), "value1".to_string(), Duration::from_secs(60)).await;
        
        let value = cache.get("key1").await;
        assert_eq!(value, Some("value1".to_string()));
        
        let stats = cache.stats().await;
        assert_eq!(stats.size, 1);
    }

    #[tokio::test]
    async fn test_cache_expiry() {
        let cache = MemoryCache::new(10);
        
        cache.set("key1".to_string(), "value1".to_string(), Duration::from_millis(10)).await;
        
        tokio::time::sleep(Duration::from_millis(20)).await;
        
        let value = cache.get("key1").await;
        assert_eq!(value, None);
    }
}
