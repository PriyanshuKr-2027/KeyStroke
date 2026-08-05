use fnv::FnvHasher;
use lru::LruCache;
use parking_lot::Mutex;
use std::hash::Hasher;
use std::num::NonZeroUsize;
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::GrammarIssue;

pub fn compute_fnv_hash(text: &str) -> u64 {
    let mut hasher = FnvHasher::default();
    hasher.write(text.as_bytes());
    hasher.finish()
}

pub struct GrammarCache {
    cache: Arc<Mutex<LruCache<u64, (Instant, Vec<GrammarIssue>)>>>,
    ttl: Duration,
}

impl Default for GrammarCache {
    fn default() -> Self {
        Self::new(20, Duration::from_secs(300))
    }
}

impl GrammarCache {
    pub fn new(capacity: usize, ttl: Duration) -> Self {
        let cap = NonZeroUsize::new(capacity).unwrap_or(NonZeroUsize::new(20).unwrap());
        Self {
            cache: Arc::new(Mutex::new(LruCache::new(cap))),
            ttl,
        }
    }

    pub fn get(&self, text: &str) -> Option<Vec<GrammarIssue>> {
        let hash = compute_fnv_hash(text);
        let mut lock = self.cache.lock();

        if let Some((inserted_at, issues)) = lock.get(&hash) {
            if inserted_at.elapsed() < self.ttl {
                return Some(issues.clone());
            }
        }
        None
    }

    pub fn put(&self, text: &str, issues: Vec<GrammarIssue>) {
        let hash = compute_fnv_hash(text);
        let mut lock = self.cache.lock();
        lock.put(hash, (Instant::now(), issues));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_put_and_get() {
        let cache = GrammarCache::new(5, Duration::from_secs(10));
        assert!(cache.get("test text").is_none());

        cache.put("test text", vec![]);
        assert!(cache.get("test text").is_some());
    }
}
