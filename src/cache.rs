use dashmap::DashMap;
use serde_json::Value;
use std::sync::Arc;
use std::time::{Duration, Instant};

#[derive(Clone)]
struct CacheEntry {
    value: Value,
    inserted_at: Instant,
    ttl: Duration,
}

impl CacheEntry {
    fn is_expired(&self) -> bool {
        self.inserted_at.elapsed() > self.ttl
    }
}

/// Cache en memoria thread-safe para respuestas de API.
///
/// Usa DashMap (concurrent HashMap) con TTL por entrada.
/// Se limpia automáticamente en cada `get` (lazy eviction).
#[derive(Clone)]
pub struct ApiCache {
    inner: Arc<DashMap<String, CacheEntry>>,
    default_ttl: Duration,
}

impl ApiCache {
    pub fn new(default_ttl_secs: u64) -> Self {
        ApiCache {
            inner: Arc::new(DashMap::new()),
            default_ttl: Duration::from_secs(default_ttl_secs),
        }
    }

    pub fn get(&self, key: &str) -> Option<Value> {
        if let Some(entry) = self.inner.get(key) {
            if !entry.is_expired() {
                return Some(entry.value.clone());
            }
        }
        // Lazy eviction: borra la entrada expirada
        self.inner.remove(key);
        None
    }

    pub fn set(&self, key: impl Into<String>, value: Value) {
        self.set_with_ttl(key, value, self.default_ttl);
    }

    pub fn set_with_ttl(&self, key: impl Into<String>, value: Value, ttl: Duration) {
        self.inner.insert(
            key.into(),
            CacheEntry { value, inserted_at: Instant::now(), ttl },
        );
    }

    pub fn invalidate(&self, key: &str) {
        self.inner.remove(key);
    }

    pub fn invalidate_prefix(&self, prefix: &str) {
        self.inner.retain(|k, _| !k.starts_with(prefix));
    }

    pub fn clear(&self) {
        self.inner.clear();
    }

    /// Número de entradas vivas (sin contar expiradas no purgadas aún).
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Purga manualmente todas las entradas expiradas.
    pub fn purge_expired(&self) {
        self.inner.retain(|_, v| !v.is_expired());
    }
}
