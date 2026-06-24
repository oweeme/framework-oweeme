use axum::{
    extract::{Path, State},
    http::{header, HeaderMap, StatusCode},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use serde_json::Value;
use std::sync::Arc;

use crate::{
    api::ApiClient,
    auth::optional_auth,
    i18n::{detect_lang_from_header, I18n},
    middleware::{request_logger, security_headers},
    pwa,
    seo::SeoMeta,
    sitemap,
    template::TemplateEngine,
    ws::{ws_chat_handler, ChatHub},
};

/// Estado compartido entre todos los handlers.
pub struct AppState {
    pub templates: TemplateEngine,
    pub api: ApiClient,
    pub i18n: I18n,
    pub chat: Arc<ChatHub>,
    pub site_url: String,
    pub site_name: String,
}

pub type SharedState = Arc<AppState>;

/// Construye el router principal con todas las rutas SEO y middlewares.
pub fn build_router(state: SharedState) -> Router {
    let chat_hub = state.chat.clone();

    Router::new()
        // Páginas SEO
        .route("/", get(handler_home))
        .route("/articulo/{slug}", get(handler_articulo))
        .route("/musica/{slug}", get(handler_musica))
        .route("/trabajo/{slug}", get(handler_trabajo))
        // SEO técnico
        .route("/sitemap.xml", get(handler_sitemap))
        .route("/robots.txt", get(handler_robots))
        // PWA
        .route("/manifest.json", get(handler_manifest))
        .route("/sw.js", get(handler_sw))
        // WebSocket chat — estado independiente del AppState principal
        .route(
            "/ws/chat/{room}",
            get(move |ws, path, query| {
                ws_chat_handler::<()>(ws, path, query, State(chat_hub))
            }),
        )
        // API interna: info de salas activas
        .route("/api/chat/rooms", get(handler_active_rooms))
        // Archivos estáticos
        .nest_service("/static", tower_http::services::ServeDir::new("static"))
        // Middlewares globales
        .layer(middleware::from_fn(optional_auth))
        .layer(middleware::from_fn(security_headers))
        .layer(middleware::from_fn(request_logger))
        .with_state(state)
}

// ─── Helpers ────────────────────────────────────────────────────────────────

fn get_lang(headers: &HeaderMap, i18n: &I18n) -> String {
    let accept = headers
        .get(header::ACCEPT_LANGUAGE)
        .and_then(|v| v.to_str().ok());
    detect_lang_from_header(accept, &i18n.available_langs())
}

fn render(
    state: &AppState,
    lang: &str,
    template: &str,
    seo: &SeoMeta,
    data: &Value,
) -> Result<Html<String>, Response> {
    // Combina contexto SEO + i18n + datos API
    let mut ctx = seo.to_tera_context();
    let i18n_ctx = state.i18n.tera_context(lang);
    for (k, v) in i18n_ctx.into_json().as_object().into_iter().flatten() {
        ctx.insert(k, v);
    }
    if let Value::Object(map) = data {
        for (k, v) in map {
            ctx.insert(k, v);
        }
    }
    ctx.insert("api_data", data);

    state
        .templates
        .render_ctx(template, &ctx)
        .map(Html)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response())
}

// ─── Handlers ────────────────────────────────────────────────────────────────

async fn handler_home(
    State(state): State<SharedState>,
    headers: HeaderMap,
) -> Response {
    let lang = get_lang(&headers, &state.i18n);
    let data: Value = state.api.get_json("/").await.unwrap_or(Value::Null);

    let hero_title = state.i18n.t(&lang, "home.hero_title");
    let hero_desc = state.i18n.t(&lang, "home.hero_desc");

    let seo = SeoMeta::new(
        format!("{} | {hero_title}", state.site_name),
        &hero_desc,
    )
    .with_canonical(format!("{}/", state.site_url));

    match render(&state, &lang, "pages/home.html", &seo, &data) {
        Ok(html) => html.into_response(),
        Err(e) => e,
    }
}

async fn handler_articulo(
    State(state): State<SharedState>,
    headers: HeaderMap,
    Path(slug): Path<String>,
) -> Response {
    let lang = get_lang(&headers, &state.i18n);
    let data: Value = match state.api.get_json(&format!("/articulos/{slug}")).await {
        Ok(d) => d,
        Err(_) => {
            return (StatusCode::NOT_FOUND, state.i18n.t(&lang, "error.not_found"))
                .into_response();
        }
    };

    let title = data["titulo"].as_str().unwrap_or(&slug).to_string();
    let description = data["descripcion"].as_str().unwrap_or("").to_string();

    let mut seo = SeoMeta::new(&title, &description)
        .with_canonical(format!("{}/articulo/{}", state.site_url, slug))
        .with_keywords(data["tags"].as_str().unwrap_or(""));

    if let Some(img) = data["imagen"].as_str() {
        seo = seo.with_image(img);
    }
    seo = seo.with_article_schema(
        data["autor"].as_str().unwrap_or("Anónimo"),
        data["fecha"].as_str().unwrap_or(""),
    );

    match render(&state, &lang, "pages/articulo.html", &seo, &data) {
        Ok(html) => html.into_response(),
        Err(e) => e,
    }
}

async fn handler_musica(
    State(state): State<SharedState>,
    headers: HeaderMap,
    Path(slug): Path<String>,
) -> Response {
    let lang = get_lang(&headers, &state.i18n);
    let data: Value = state
        .api
        .get_json(&format!("/musica/{slug}"))
        .await
        .unwrap_or(Value::Null);

    let title = data["nombre"].as_str().unwrap_or(&slug).to_string();
    let description = data["descripcion"].as_str().unwrap_or("").to_string();

    let mut seo = SeoMeta::new(&title, &description)
        .with_canonical(format!("{}/musica/{}", state.site_url, slug));

    if let Some(img) = data["cover"].as_str() {
        seo = seo.with_image(img);
    }
    seo = seo.with_music_schema(data["artista"].as_str().unwrap_or("Artista desconocido"));

    match render(&state, &lang, "pages/musica.html", &seo, &data) {
        Ok(html) => html.into_response(),
        Err(e) => e,
    }
}

async fn handler_trabajo(
    State(state): State<SharedState>,
    headers: HeaderMap,
    Path(slug): Path<String>,
) -> Response {
    let lang = get_lang(&headers, &state.i18n);
    let data: Value = state
        .api
        .get_json(&format!("/trabajos/{slug}"))
        .await
        .unwrap_or(Value::Null);

    let title = data["titulo"].as_str().unwrap_or(&slug).to_string();
    let description = data["descripcion"].as_str().unwrap_or("").to_string();

    let seo = SeoMeta::new(&title, &description)
        .with_canonical(format!("{}/trabajo/{}", state.site_url, slug))
        .with_job_schema(
            data["empresa"].as_str().unwrap_or("Empresa"),
            data["ubicacion"].as_str().unwrap_or("Remoto"),
        );

    match render(&state, &lang, "pages/trabajo.html", &seo, &data) {
        Ok(html) => html.into_response(),
        Err(e) => e,
    }
}

async fn handler_sitemap(State(state): State<SharedState>) -> Response {
    let xml = sitemap::build_dynamic_sitemap(&state.api, &state.site_url).await;
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/xml; charset=utf-8")],
        xml,
    )
        .into_response()
}

async fn handler_robots(State(state): State<SharedState>) -> Response {
    let txt = sitemap::generate_robots(&state.site_url);
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        txt,
    )
        .into_response()
}

async fn handler_manifest(State(state): State<SharedState>) -> Response {
    let manifest = pwa::PwaManifest::new(
        &state.site_name,
        &state.site_name[..state.site_name.len().min(12)],
        "Plataforma para músicos y artistas",
    )
    .with_theme("#0f0f1a", "#6c63ff");

    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/json")],
        manifest.to_json_string(),
    )
        .into_response()
}

async fn handler_sw() -> Response {
    let sw = pwa::generate_service_worker(
        "framework-oweeme-v3",
        &["/", "/static/css/app.css", "/static/js/app.js"],
    );
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/javascript")],
        sw,
    )
        .into_response()
}

async fn handler_active_rooms(State(state): State<SharedState>) -> Response {
    let rooms = state.chat.active_rooms();
    (StatusCode::OK, axum::Json(rooms)).into_response()
}
