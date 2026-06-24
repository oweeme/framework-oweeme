use framework_oweeme::{
    api::ApiClient,
    i18n::I18n,
    plugin::{HealthPlugin, PluginRegistry, RssPlugin},
    router::{build_router, AppState},
    template::TemplateEngine,
    ws::ChatHub,
};
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let site_url = std::env::var("SITE_URL")
        .unwrap_or_else(|_| "http://localhost:3000".into());
    let site_name = std::env::var("SITE_NAME")
        .unwrap_or_else(|_| "Mi Plataforma".into());
    let api_base = std::env::var("API_BASE_URL")
        .unwrap_or_else(|_| "http://localhost:8080".into());

    let cache_ttl: u64 = std::env::var("CACHE_TTL_SECS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(300);

    let templates = TemplateEngine::new("templates")
        .expect("No se pudo cargar el directorio de plantillas");

    let api = ApiClient::new(&api_base).with_cache(cache_ttl);

    let default_lang = std::env::var("DEFAULT_LANG").unwrap_or_else(|_| "es".into());
    let i18n = I18n::new(&default_lang);
    i18n.load_dir("locales").unwrap_or_else(|e| {
        tracing::warn!("i18n: {e}");
    });

    // Plugins
    let (_, plugin_router) = PluginRegistry::new()
        .register(HealthPlugin { version: env!("CARGO_PKG_VERSION") })
        .register(RssPlugin {
            site_name: site_name.clone(),
            site_url: site_url.clone(),
            api_base: api_base.clone(),
        })
        .build();

    let state = Arc::new(AppState {
        templates,
        api,
        i18n,
        chat: Arc::new(ChatHub::new()),
        site_url: site_url.clone(),
        site_name,
    });

    // Router principal + rutas de plugins
    let router = build_router(state).merge(plugin_router);

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".into());
    let addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    tracing::info!("Framework Oweeme v4 en http://{addr}");
    tracing::info!(api = %api_base, lang = %default_lang, cache_ttl_secs = cache_ttl);

    axum::serve(listener, router).await.unwrap();
}
