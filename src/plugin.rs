use axum::Router;
use std::sync::Arc;

/// Trait que deben implementar todos los plugins del framework.
///
/// Un plugin puede:
///   - Añadir rutas propias (`routes`)
///   - Ejecutar lógica al arrancar (`on_start`)
///   - Registrar su nombre y versión (`meta`)
pub trait Plugin: Send + Sync {
    fn meta(&self) -> PluginMeta;

    /// Rutas adicionales que el plugin registra.
    /// Se montan bajo el prefijo configurado.
    fn routes(&self) -> Option<Router> {
        None
    }

    /// Se llama una sola vez cuando el servidor arranca.
    fn on_start(&self) {}
}

#[derive(Debug, Clone)]
pub struct PluginMeta {
    pub name: &'static str,
    pub version: &'static str,
    pub description: &'static str,
}

/// Registro central de plugins.
pub struct PluginRegistry {
    plugins: Vec<Arc<dyn Plugin>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        PluginRegistry { plugins: vec![] }
    }

    /// Registra un plugin. Se puede encadenar.
    pub fn register(mut self, plugin: impl Plugin + 'static) -> Self {
        let plugin = Arc::new(plugin);
        tracing::info!(
            "plugin '{}' v{} registrado",
            plugin.meta().name,
            plugin.meta().version
        );
        self.plugins.push(plugin);
        self
    }

    /// Ejecuta `on_start` en todos los plugins y devuelve el router combinado.
    pub fn build(self) -> (Vec<Arc<dyn Plugin>>, Router) {
        let mut combined = Router::new();
        for plugin in &self.plugins {
            plugin.on_start();
            if let Some(routes) = plugin.routes() {
                combined = combined.merge(routes);
            }
        }
        (self.plugins, combined)
    }

    pub fn list(&self) -> Vec<PluginMeta> {
        self.plugins.iter().map(|p| p.meta()).collect()
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Plugins integrados ───────────────────────────────────────────────────────

/// Plugin: página de salud del servidor en `/health`.
pub struct HealthPlugin {
    pub version: &'static str,
}

impl Plugin for HealthPlugin {
    fn meta(&self) -> PluginMeta {
        PluginMeta {
            name: "health",
            version: self.version,
            description: "Endpoint /health para monitoreo",
        }
    }

    fn routes(&self) -> Option<Router> {
        use axum::{routing::get, Json};
        use serde_json::json;

        let version = self.version;
        Some(Router::new().route(
            "/health",
            get(move || async move {
                Json(json!({
                    "status": "ok",
                    "framework": "oweeme",
                    "version": version
                }))
            }),
        ))
    }
}

/// Plugin: feed RSS en `/rss.xml` (consume la API para obtener artículos recientes).
pub struct RssPlugin {
    pub site_name: String,
    pub site_url: String,
    pub api_base: String,
}

impl Plugin for RssPlugin {
    fn meta(&self) -> PluginMeta {
        PluginMeta {
            name: "rss",
            version: "1.0.0",
            description: "Feed RSS en /rss.xml",
        }
    }

    fn routes(&self) -> Option<Router> {
        use axum::{
            http::{header, StatusCode},
            response::IntoResponse,
            routing::get,
        };

        let site_name = self.site_name.clone();
        let site_url = self.site_url.clone();
        let api_base = self.api_base.clone();

        Some(Router::new().route(
            "/rss.xml",
            get(move || {
                let site_name = site_name.clone();
                let site_url = site_url.clone();
                let api_base = api_base.clone();
                async move {
                    let items = fetch_rss_items(&api_base).await;
                    let xml = build_rss(&site_name, &site_url, &items);
                    (
                        StatusCode::OK,
                        [(header::CONTENT_TYPE, "application/rss+xml; charset=utf-8")],
                        xml,
                    )
                        .into_response()
                }
            }),
        ))
    }
}

async fn fetch_rss_items(api_base: &str) -> serde_json::Value {
    reqwest::Client::new()
        .get(format!("{api_base}/articulos?limit=20"))
        .send()
        .await
        .ok()
        .map(|r| async move { r.json::<serde_json::Value>().await.unwrap_or(serde_json::Value::Null) })
        .map(|f| tokio::task::block_in_place(|| tokio::runtime::Handle::current().block_on(f)))
        .unwrap_or(serde_json::Value::Null)
}

fn build_rss(site_name: &str, site_url: &str, items: &serde_json::Value) -> String {
    let mut items_xml = String::new();
    if let Some(arr) = items.as_array() {
        for item in arr.iter().take(20) {
            let title = item["titulo"].as_str().unwrap_or("Sin título");
            let slug = item["slug"].as_str().unwrap_or("");
            let desc = item["descripcion"].as_str().unwrap_or("");
            let date = item["fecha"].as_str().unwrap_or("");
            items_xml.push_str(&format!(
                "<item>\
                <title><![CDATA[{title}]]></title>\
                <link>{site_url}/articulo/{slug}</link>\
                <description><![CDATA[{desc}]]></description>\
                <pubDate>{date}</pubDate>\
                <guid>{site_url}/articulo/{slug}</guid>\
                </item>"
            ));
        }
    }
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">
<channel>
<title>{site_name}</title>
<link>{site_url}</link>
<description>Artículos recientes de {site_name}</description>
<atom:link href="{site_url}/rss.xml" rel="self" type="application/rss+xml"/>
{items_xml}
</channel>
</rss>"#
    )
}
