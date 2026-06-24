use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize)]
pub struct PwaManifest {
    pub name: String,
    pub short_name: String,
    pub description: String,
    pub start_url: String,
    pub display: String,
    pub background_color: String,
    pub theme_color: String,
    pub icons: Vec<PwaIcon>,
    pub categories: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct PwaIcon {
    pub src: String,
    pub sizes: String,
    pub r#type: String,
    pub purpose: Option<String>,
}

impl PwaManifest {
    pub fn new(
        name: impl Into<String>,
        short_name: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        PwaManifest {
            name: name.into(),
            short_name: short_name.into(),
            description: description.into(),
            start_url: "/".into(),
            display: "standalone".into(),
            background_color: "#ffffff".into(),
            theme_color: "#1a1a2e".into(),
            icons: vec![
                PwaIcon {
                    src: "/static/icons/icon-192.png".into(),
                    sizes: "192x192".into(),
                    r#type: "image/png".into(),
                    purpose: Some("maskable any".into()),
                },
                PwaIcon {
                    src: "/static/icons/icon-512.png".into(),
                    sizes: "512x512".into(),
                    r#type: "image/png".into(),
                    purpose: Some("maskable any".into()),
                },
            ],
            categories: vec!["music".into(), "social".into(), "entertainment".into()],
        }
    }

    pub fn with_theme(mut self, bg: impl Into<String>, theme: impl Into<String>) -> Self {
        self.background_color = bg.into();
        self.theme_color = theme.into();
        self
    }

    pub fn to_json(&self) -> Value {
        serde_json::to_value(self).unwrap()
    }

    pub fn to_json_string(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}

/// Genera el código del Service Worker para cache offline.
pub fn generate_service_worker(cache_name: &str, urls_to_cache: &[&str]) -> String {
    let urls_json = urls_to_cache
        .iter()
        .map(|u| format!("  '{u}'"))
        .collect::<Vec<_>>()
        .join(",\n");

    format!(
        r#"const CACHE_NAME = '{cache_name}';
const urlsToCache = [
{urls_json}
];

self.addEventListener('install', event => {{
  event.waitUntil(
    caches.open(CACHE_NAME).then(cache => cache.addAll(urlsToCache))
  );
  self.skipWaiting();
}});

self.addEventListener('activate', event => {{
  event.waitUntil(
    caches.keys().then(keys =>
      Promise.all(keys.filter(k => k !== CACHE_NAME).map(k => caches.delete(k)))
    )
  );
  self.clients.claim();
}});

self.addEventListener('fetch', event => {{
  event.respondWith(
    caches.match(event.request).then(cached => {{
      if (cached) return cached;
      return fetch(event.request).then(response => {{
        if (!response || response.status !== 200 || response.type !== 'basic') return response;
        const clone = response.clone();
        caches.open(CACHE_NAME).then(cache => cache.put(event.request, clone));
        return response;
      }});
    }})
  );
}});
"#
    )
}
