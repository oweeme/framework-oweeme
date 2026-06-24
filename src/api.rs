use anyhow::Result;
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;

use crate::cache::ApiCache;

/// Cliente HTTP para consumir APIs externas (PHP, Flask, Django, Go, etc.)
/// con cache en memoria automático.
#[derive(Clone)]
pub struct ApiClient {
    client: Client,
    base_url: String,
    default_headers: HashMap<String, String>,
    cache: Option<ApiCache>,
}

impl ApiClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        ApiClient {
            client: Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .expect("Failed to create HTTP client"),
            base_url: base_url.into(),
            default_headers: HashMap::new(),
            cache: None,
        }
    }

    /// Activa cache con TTL en segundos (ej. 300 = 5 minutos).
    pub fn with_cache(mut self, ttl_secs: u64) -> Self {
        self.cache = Some(ApiCache::new(ttl_secs));
        self
    }

    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.default_headers.insert(key.into(), value.into());
        self
    }

    pub fn with_bearer(self, token: impl Into<String>) -> Self {
        self.with_header("Authorization", format!("Bearer {}", token.into()))
    }

    /// Expone el cache para usarlo desde el router (invalidación manual).
    pub fn cache(&self) -> Option<&ApiCache> {
        self.cache.as_ref()
    }

    /// GET con cache automático. Si hay hit válido, no toca la red.
    pub async fn get_json(&self, path: &str) -> Result<Value> {
        let cache_key = format!("GET:{}{}", self.base_url, path);

        if let Some(cache) = &self.cache {
            if let Some(cached) = cache.get(&cache_key) {
                tracing::debug!(path, "cache HIT");
                return Ok(cached);
            }
        }

        let url = format!("{}{}", self.base_url, path);
        let mut req = self.client.get(&url);
        for (k, v) in &self.default_headers {
            req = req.header(k, v);
        }
        let data = req.send().await?.error_for_status()?.json::<Value>().await?;

        if let Some(cache) = &self.cache {
            cache.set(&cache_key, data.clone());
            tracing::debug!(path, "cache MISS → guardado");
        }

        Ok(data)
    }

    /// GET con query params (sin cache, los params cambian demasiado).
    pub async fn get_with_params(&self, path: &str, params: &[(&str, &str)]) -> Result<Value> {
        let url = format!("{}{}", self.base_url, path);
        let mut req = self.client.get(&url).query(params);
        for (k, v) in &self.default_headers {
            req = req.header(k, v);
        }
        Ok(req.send().await?.error_for_status()?.json::<Value>().await?)
    }

    /// GET tipado (sin cache explícito, usa get_json internamente).
    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let val = self.get_json(path).await?;
        Ok(serde_json::from_value(val)?)
    }

    /// GET con TTL personalizado para esta petición específica.
    pub async fn get_json_ttl(&self, path: &str, ttl: Duration) -> Result<Value> {
        let cache_key = format!("GET:{}{}", self.base_url, path);

        if let Some(cache) = &self.cache {
            if let Some(cached) = cache.get(&cache_key) {
                return Ok(cached);
            }
        }

        let url = format!("{}{}", self.base_url, path);
        let mut req = self.client.get(&url);
        for (k, v) in &self.default_headers {
            req = req.header(k, v);
        }
        let data = req.send().await?.error_for_status()?.json::<Value>().await?;

        if let Some(cache) = &self.cache {
            cache.set_with_ttl(&cache_key, data.clone(), ttl);
        }

        Ok(data)
    }

    /// POST con body JSON (nunca cacheado).
    pub async fn post<T: DeserializeOwned>(&self, path: &str, body: &Value) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);
        let mut req = self.client.post(&url).json(body);
        for (k, v) in &self.default_headers {
            req = req.header(k, v);
        }
        Ok(req.send().await?.error_for_status()?.json::<T>().await?)
    }
}
