use crate::scaffold::ProjectConfig;

// ─── Rust ────────────────────────────────────────────────────────────────────

pub fn cargo_toml(name: &str) -> String {
    format!(
        r#"[package]
name = "{name}"
version = "0.1.0"
edition = "2021"

[dependencies]
framework_oweeme = {{ git = "https://github.com/tu-usuario/framework-oweeme" }}
tokio = {{ version = "1", features = ["full"] }}
tracing-subscriber = {{ version = "0.3", features = ["env-filter"] }}
"#
    )
}

pub fn main_rs() -> &'static str {
    r#"use framework_oweeme::{
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

    let site_url  = std::env::var("SITE_URL").unwrap_or_else(|_| "http://localhost:3000".into());
    let site_name = std::env::var("SITE_NAME").unwrap_or_else(|_| "Mi Plataforma".into());
    let api_base  = std::env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".into());
    let cache_ttl: u64 = std::env::var("CACHE_TTL_SECS").ok()
        .and_then(|v| v.parse().ok()).unwrap_or(300);
    let default_lang = std::env::var("DEFAULT_LANG").unwrap_or_else(|_| "es".into());

    let templates = TemplateEngine::new("templates").expect("Cannot load templates/");
    let api = ApiClient::new(&api_base).with_cache(cache_ttl);
    let i18n = I18n::new(&default_lang);
    i18n.load_dir("locales").ok();

    let (_, plugin_router) = PluginRegistry::new()
        .register(HealthPlugin { version: env!("CARGO_PKG_VERSION") })
        .register(RssPlugin { site_name: site_name.clone(), site_url: site_url.clone(), api_base: api_base.clone() })
        .build();

    let state = Arc::new(AppState {
        templates, api, i18n,
        chat: Arc::new(ChatHub::new()),
        site_url: site_url.clone(),
        site_name,
    });

    let router = build_router(state).merge(plugin_router);
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".into());
    let addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("Server running at http://{addr}");
    axum::serve(listener, router).await.unwrap();
}
"#
}

// ─── HTML Templates ───────────────────────────────────────────────────────────

pub fn base_html(site_name: &str, with_pwa: bool) -> String {
    let pwa_tags = if with_pwa {
        "  <link rel=\"manifest\" href=\"/manifest.json\">\n  <meta name=\"theme-color\" content=\"#6c63ff\">\n  <meta name=\"mobile-web-app-capable\" content=\"yes\">\n  <meta name=\"apple-mobile-web-app-capable\" content=\"yes\">\n  <meta name=\"apple-mobile-web-app-status-bar-style\" content=\"black-translucent\">"
    } else {
        ""
    };

    let sw_script = if with_pwa {
        r#"  <script>
    if ('serviceWorker' in navigator) {
      navigator.serviceWorker.register('/sw.js');
    }
  </script>"#
    } else {
        ""
    };

    format!(
        r#"<!DOCTYPE html>
<html lang="{{{{ lang | default(value='es') }}}}">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">

  <title>{{{{ seo_title }}}}</title>
  <meta name="description" content="{{{{ seo_description }}}}">
  {{% if seo_keywords %}}<meta name="keywords" content="{{{{ seo_keywords }}}}">{{%  endif %}}
  {{% if seo_canonical %}}<link rel="canonical" href="{{{{ seo_canonical }}}}">{{%  endif %}}

  <meta property="og:title" content="{{{{ seo_og_title | default(value=seo_title) }}}}">
  <meta property="og:description" content="{{{{ seo_og_description | default(value=seo_description) }}}}">
  <meta property="og:type" content="{{{{ seo_og_type | default(value='website') }}}}">
  {{% if seo_og_image %}}<meta property="og:image" content="{{{{ seo_og_image }}}}">{{%  endif %}}
  {{% if seo_canonical %}}<meta property="og:url" content="{{{{ seo_canonical }}}}">{{%  endif %}}

  <meta name="twitter:card" content="{{{{ seo_twitter_card | default(value='summary_large_image') }}}}">
  <meta name="twitter:title" content="{{{{ seo_twitter_title | default(value=seo_title) }}}}">
  <meta name="twitter:description" content="{{{{ seo_twitter_description | default(value=seo_description) }}}}">
  {{% if seo_twitter_image %}}<meta name="twitter:image" content="{{{{ seo_twitter_image }}}}">{{%  endif %}}

  {{% if seo_schema_json %}}
  <script type="application/ld+json">{{{{ seo_schema_json | safe }}}}</script>
  {{%  endif %}}

{pwa_tags}
  <link rel="stylesheet" href="/static/css/app.css">
  {{% block head_extra %}}{{% endblock head_extra %}}
</head>
<body>
  {{% include "components/Navbar.html" %}}

  <main id="app">
    {{% block content %}}{{% endblock content %}}
  </main>

  {{% include "components/Footer.html" %}}

{sw_script}
  <script src="/static/js/app.js" defer></script>
  <script src="/static/js/chat.js" defer></script>
  {{% block scripts %}}{{% endblock scripts %}}
</body>
</html>
"#
    )
}

pub fn page_home() -> &'static str {
    r#"{% extends "base.html" %}
{% block content %}
{% include "components/Hero.html" %}
<section class="container section">
  {% if api_data and api_data.destacados %}
  <h2 class="section__title">{{ t["home.featured"] | default(value="Destacados") }}</h2>
  <div class="grid">
    {% for item in api_data.destacados %}
      {% set titulo = item.titulo %}
      {% set descripcion = item.descripcion %}
      {% set imagen = item.imagen %}
      {% set slug = item.slug %}
      {% set autor = item.autor %}
      {% set tipo = item.tipo | default(value="articulo") %}
      {% include "components/Card.html" %}
    {% endfor %}
  </div>
  {% else %}
  <div class="empty-state">
    <div class="empty-state__icon">🎵</div>
    <h3>Conecta tu API</h3>
    <p>Edita <code>.env</code> con tu <code>API_BASE_URL</code> y la plataforma cargará el contenido.</p>
  </div>
  {% endif %}
</section>
{% endblock content %}
"#
}

pub fn page_articulo() -> &'static str {
    r#"{% extends "base.html" %}
{% block content %}
<article class="container article" itemscope itemtype="https://schema.org/Article">
  <header class="article__header">
    {% if imagen %}<img class="article__cover" src="{{ imagen }}" alt="{{ titulo }}" itemprop="image">{% endif %}
    <div class="article__meta">
      <span class="badge badge--primary">Artículo</span>
      <span itemprop="author">{{ autor | default(value="Anónimo") }}</span>
      <time itemprop="datePublished">{{ fecha | default(value="") }}</time>
    </div>
    <h1 class="article__title" itemprop="headline">{{ titulo }}</h1>
    <p class="article__lead">{{ descripcion }}</p>
  </header>
  <div class="article__body prose" itemprop="articleBody">{{ contenido | safe }}</div>
  {% if tags %}
  <footer class="article__footer">
    {% for tag in tags %}
    <a href="/tag/{{ tag }}" class="chip">#{{ tag }}</a>
    {% endfor %}
  </footer>
  {% endif %}
</article>
{% endblock content %}
"#
}

pub fn page_musica() -> &'static str {
    r#"{% extends "base.html" %}
{% block content %}
<div class="container">
  <article class="music-card" itemscope itemtype="https://schema.org/MusicRecording">
    <div class="music-card__cover-wrap">
      {% if cover %}
      <img class="music-card__cover" src="{{ cover }}" alt="{{ nombre }}" itemprop="image">
      {% else %}
      <div class="music-card__cover music-card__cover--placeholder">🎵</div>
      {% endif %}
    </div>
    <div class="music-card__info">
      <h1 class="music-card__title" itemprop="name">{{ nombre }}</h1>
      <p class="music-card__artist" itemprop="byArtist">{{ artista }}</p>
      {% if descripcion %}<p class="music-card__desc" itemprop="description">{{ descripcion }}</p>{% endif %}
      {% if audio_url %}
      <audio class="music-card__player" controls itemprop="audio">
        <source src="{{ audio_url }}" type="audio/mpeg">
      </audio>
      {% endif %}
    </div>
  </article>
</div>
{% endblock content %}
"#
}

pub fn page_trabajo() -> &'static str {
    r#"{% extends "base.html" %}
{% block content %}
<div class="container">
  <article class="job" itemscope itemtype="https://schema.org/JobPosting">
    <header class="job__header">
      <div class="job__badges">
        <span class="badge badge--primary">{{ tipo | default(value="Full-time") }}</span>
        <span class="badge badge--ghost">{{ ubicacion | default(value="Remoto") }}</span>
      </div>
      <h1 class="job__title" itemprop="title">{{ titulo }}</h1>
      <div class="job__meta">
        <span itemprop="hiringOrganization" itemscope itemtype="https://schema.org/Organization">
          <strong itemprop="name">{{ empresa | default(value="Empresa") }}</strong>
        </span>
        {% if salario %}<span class="job__salary">{{ salario }}</span>{% endif %}
      </div>
    </header>
    <div class="job__body prose" itemprop="description">{{ descripcion | safe }}</div>
    {% if requisitos %}
    <section class="job__reqs">
      <h2>Requisitos</h2>
      <ul>{% for r in requisitos %}<li>{{ r }}</li>{% endfor %}</ul>
    </section>
    {% endif %}
    {% if url_aplicar %}
    <a href="{{ url_aplicar }}" class="btn btn--primary btn--lg" itemprop="url">Aplicar ahora</a>
    {% endif %}
  </article>
</div>
{% endblock content %}
"#
}

pub fn page_404() -> &'static str {
    r#"{% extends "base.html" %}
{% block content %}
<div class="container center-page">
  <div class="error-page">
    <div class="error-page__code">404</div>
    <h1 class="error-page__title">Página no encontrada</h1>
    <p class="error-page__desc">La ruta que buscas no existe o fue movida.</p>
    <a href="/" class="btn btn--primary">Volver al inicio</a>
  </div>
</div>
{% endblock content %}
"#
}

pub fn component_navbar(site_name: &str) -> String {
    format!(
        r#"<header class="navbar">
  <div class="navbar__inner">
    <a href="/" class="navbar__brand">
      <span class="navbar__logo">◈</span>
      <span class="navbar__name">{site_name}</span>
    </a>
    <nav class="navbar__nav">
      <a href="/" class="navbar__link">{{{{ t["nav.home"] | default(value="Inicio") }}}}</a>
      <a href="/musica" class="navbar__link">{{{{ t["nav.music"] | default(value="Música") }}}}</a>
      <a href="/articulos" class="navbar__link">{{{{ t["nav.articles"] | default(value="Artículos") }}}}</a>
      <a href="/trabajos" class="navbar__link">{{{{ t["nav.jobs"] | default(value="Trabajos") }}}}</a>
    </nav>
    <button class="navbar__burger" id="burger" aria-label="Menu">
      <span></span><span></span><span></span>
    </button>
  </div>
</header>
"#
    )
}

pub fn component_footer(site_name: &str) -> String {
    format!(
        r#"<footer class="footer">
  <div class="footer__inner">
    <div class="footer__brand">
      <span class="navbar__logo">◈</span>
      <span>{site_name}</span>
    </div>
    <div class="footer__links">
      <a href="/sitemap.xml">Sitemap</a>
      <a href="/rss.xml">RSS</a>
      <a href="/health">Status</a>
    </div>
    <p class="footer__copy">© {{{{ now() | date(format="%Y") }}}} {site_name}. {{{{ t["footer.rights"] | default(value="Todos los derechos reservados") }}}}.</p>
  </div>
</footer>
"#
    )
}

pub fn component_hero() -> &'static str {
    r#"<section class="hero">
  <div class="hero__bg"></div>
  <div class="container hero__content">
    <h1 class="hero__title">{{ t["home.hero_title"] | default(value="Bienvenido") }}</h1>
    <p class="hero__desc">{{ t["home.hero_desc"] | default(value="Tu plataforma.") }}</p>
    <div class="hero__actions">
      <a href="/musica" class="btn btn--primary btn--lg">Explorar música</a>
      <a href="/articulos" class="btn btn--ghost btn--lg">Ver artículos</a>
    </div>
  </div>
</section>
"#
}

pub fn component_card() -> &'static str {
    r#"<article class="card">
  {% if imagen %}
  <div class="card__img-wrap">
    <img class="card__img" src="{{ imagen }}" alt="{{ titulo }}" loading="lazy">
  </div>
  {% endif %}
  <div class="card__body">
    {% if tipo %}<span class="badge badge--ghost">{{ tipo }}</span>{% endif %}
    <h3 class="card__title">
      <a href="/{{ tipo | default(value='articulo') }}/{{ slug }}">{{ titulo }}</a>
    </h3>
    <p class="card__desc">{{ descripcion | truncate(length=120) }}</p>
    <div class="card__footer">
      <span class="card__author">{{ autor | default(value="Anónimo") }}</span>
      <a href="/{{ tipo | default(value='articulo') }}/{{ slug }}" class="card__link">
        {{ t["articulo.read_more"] | default(value="Ver más") }} →
      </a>
    </div>
  </div>
</article>
"#
}

// ─── CSS — Design system tipo Quasar/Material ─────────────────────────────────

pub fn css_app() -> &'static str {
    r#"/* ═══════════════════════════════════════════════════════════
   Framework Oweeme — Design System
   Inspirado en Quasar / Material Design
   ═══════════════════════════════════════════════════════════ */

/* ─── Tokens ──────────────────────────────────────────────── */
:root {
  --primary:       #6c63ff;
  --primary-dark:  #5548e0;
  --primary-light: #8b85ff;
  --secondary:     #ff6584;
  --accent:        #43e97b;
  --bg:            #0d0d1a;
  --bg-card:       #13132a;
  --bg-nav:        rgba(13,13,26,0.95);
  --surface:       #1a1a35;
  --surface-2:     #22224a;
  --border:        rgba(108,99,255,0.15);
  --text:          #e8e8f5;
  --text-muted:    #7a7a9d;
  --text-soft:     #aaaac5;
  --radius-sm:     6px;
  --radius:        12px;
  --radius-lg:     20px;
  --shadow:        0 4px 32px rgba(0,0,0,0.4);
  --shadow-card:   0 2px 16px rgba(0,0,0,0.3);
  --transition:    all 0.2s ease;
  --font:          'Inter', 'Segoe UI', system-ui, -apple-system, sans-serif;
  --font-mono:     'Fira Code', 'Cascadia Code', monospace;
}

/* ─── Reset ───────────────────────────────────────────────── */
*, *::before, *::after { box-sizing: border-box; margin: 0; padding: 0; }
html { scroll-behavior: smooth; }
body {
  font-family: var(--font);
  background: var(--bg);
  color: var(--text);
  line-height: 1.6;
  min-height: 100vh;
  -webkit-font-smoothing: antialiased;
}
img { max-width: 100%; display: block; }
a { color: inherit; text-decoration: none; }
button { cursor: pointer; border: none; background: none; font: inherit; }

/* ─── Layout ──────────────────────────────────────────────── */
.container {
  width: 100%;
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 1.5rem;
}
.section { padding: 4rem 0; }
.section__title {
  font-size: 1.75rem;
  font-weight: 700;
  margin-bottom: 2rem;
  background: linear-gradient(135deg, var(--text), var(--primary-light));
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}
.center-page {
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 60vh;
}
.grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 1.5rem;
}

/* ─── Navbar ──────────────────────────────────────────────── */
.navbar {
  position: sticky;
  top: 0;
  z-index: 100;
  background: var(--bg-nav);
  border-bottom: 1px solid var(--border);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
}
.navbar__inner {
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 1.5rem;
  height: 64px;
  display: flex;
  align-items: center;
  gap: 2rem;
}
.navbar__brand {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-weight: 700;
  font-size: 1.1rem;
  color: var(--text);
}
.navbar__logo {
  font-size: 1.4rem;
  color: var(--primary);
  filter: drop-shadow(0 0 8px var(--primary));
}
.navbar__nav {
  display: flex;
  gap: 0.25rem;
  margin-left: auto;
}
.navbar__link {
  padding: 0.4rem 0.9rem;
  border-radius: var(--radius-sm);
  color: var(--text-muted);
  font-size: 0.9rem;
  font-weight: 500;
  transition: var(--transition);
}
.navbar__link:hover {
  color: var(--text);
  background: var(--surface);
}
.navbar__burger {
  display: none;
  flex-direction: column;
  gap: 5px;
  margin-left: auto;
  padding: 0.5rem;
}
.navbar__burger span {
  display: block;
  width: 22px;
  height: 2px;
  background: var(--text);
  border-radius: 2px;
  transition: var(--transition);
}
@media (max-width: 700px) {
  .navbar__nav { display: none; }
  .navbar__burger { display: flex; }
  .navbar__nav.is-open {
    display: flex;
    flex-direction: column;
    position: absolute;
    top: 64px; left: 0; right: 0;
    background: var(--bg-nav);
    border-bottom: 1px solid var(--border);
    padding: 1rem 1.5rem;
    gap: 0.25rem;
  }
}

/* ─── Hero ────────────────────────────────────────────────── */
.hero {
  position: relative;
  padding: 6rem 0 5rem;
  overflow: hidden;
}
.hero__bg {
  position: absolute;
  inset: 0;
  background:
    radial-gradient(ellipse 80% 60% at 50% 0%, rgba(108,99,255,0.18) 0%, transparent 70%),
    radial-gradient(ellipse 40% 40% at 80% 80%, rgba(255,101,132,0.1) 0%, transparent 60%);
  pointer-events: none;
}
.hero__content { position: relative; text-align: center; }
.hero__title {
  font-size: clamp(2.2rem, 5vw, 3.5rem);
  font-weight: 800;
  letter-spacing: -0.02em;
  line-height: 1.1;
  margin-bottom: 1.25rem;
  background: linear-gradient(135deg, #fff 30%, var(--primary-light) 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}
.hero__desc {
  font-size: 1.15rem;
  color: var(--text-soft);
  max-width: 560px;
  margin: 0 auto 2.5rem;
}
.hero__actions { display: flex; gap: 1rem; justify-content: center; flex-wrap: wrap; }

/* ─── Buttons ─────────────────────────────────────────────── */
.btn {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.6rem 1.4rem;
  border-radius: var(--radius-sm);
  font-weight: 600;
  font-size: 0.9rem;
  transition: var(--transition);
  cursor: pointer;
}
.btn--primary {
  background: var(--primary);
  color: #fff;
  box-shadow: 0 4px 20px rgba(108,99,255,0.35);
}
.btn--primary:hover {
  background: var(--primary-dark);
  transform: translateY(-1px);
  box-shadow: 0 6px 24px rgba(108,99,255,0.45);
}
.btn--ghost {
  background: transparent;
  color: var(--text);
  border: 1px solid var(--border);
}
.btn--ghost:hover {
  background: var(--surface);
  border-color: var(--primary);
  color: var(--primary);
}
.btn--lg { padding: 0.8rem 1.8rem; font-size: 1rem; border-radius: var(--radius); }

/* ─── Cards ───────────────────────────────────────────────── */
.card {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: var(--radius);
  overflow: hidden;
  transition: var(--transition);
  display: flex;
  flex-direction: column;
}
.card:hover {
  border-color: var(--primary);
  transform: translateY(-3px);
  box-shadow: 0 8px 32px rgba(108,99,255,0.2);
}
.card__img-wrap { overflow: hidden; aspect-ratio: 16/9; }
.card__img {
  width: 100%; height: 100%; object-fit: cover;
  transition: transform 0.4s ease;
}
.card:hover .card__img { transform: scale(1.04); }
.card__body { padding: 1.25rem; flex: 1; display: flex; flex-direction: column; gap: 0.75rem; }
.card__title { font-size: 1.05rem; font-weight: 600; }
.card__title a:hover { color: var(--primary); }
.card__desc { color: var(--text-muted); font-size: 0.9rem; flex: 1; }
.card__footer { display: flex; justify-content: space-between; align-items: center; font-size: 0.85rem; }
.card__author { color: var(--text-muted); }
.card__link { color: var(--primary); font-weight: 500; transition: var(--transition); }
.card__link:hover { color: var(--primary-light); }

/* ─── Badges & Chips ──────────────────────────────────────── */
.badge {
  display: inline-block;
  padding: 0.2rem 0.7rem;
  border-radius: 999px;
  font-size: 0.75rem;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}
.badge--primary { background: rgba(108,99,255,0.15); color: var(--primary); }
.badge--ghost   { background: var(--surface); color: var(--text-muted); border: 1px solid var(--border); }
.chip {
  display: inline-block;
  padding: 0.25rem 0.8rem;
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: 999px;
  font-size: 0.82rem;
  color: var(--text-muted);
  transition: var(--transition);
}
.chip:hover { border-color: var(--primary); color: var(--primary); }

/* ─── Article ─────────────────────────────────────────────── */
.article { max-width: 760px; margin: 3rem auto; }
.article__header { margin-bottom: 2.5rem; }
.article__cover { width: 100%; border-radius: var(--radius); margin-bottom: 1.5rem; aspect-ratio: 16/7; object-fit: cover; }
.article__meta { display: flex; align-items: center; gap: 1rem; font-size: 0.85rem; color: var(--text-muted); margin-bottom: 1rem; flex-wrap: wrap; }
.article__title { font-size: clamp(1.6rem, 4vw, 2.4rem); font-weight: 800; line-height: 1.2; margin-bottom: 0.75rem; }
.article__lead { font-size: 1.1rem; color: var(--text-soft); }
.article__footer { margin-top: 2.5rem; display: flex; gap: 0.5rem; flex-wrap: wrap; }

/* ─── Prose ───────────────────────────────────────────────── */
.prose { line-height: 1.85; color: var(--text-soft); }
.prose h2, .prose h3 { color: var(--text); font-weight: 700; margin: 2rem 0 0.75rem; }
.prose h2 { font-size: 1.4rem; }
.prose h3 { font-size: 1.15rem; }
.prose p  { margin-bottom: 1.25rem; }
.prose ul, .prose ol { padding-left: 1.5rem; margin-bottom: 1.25rem; }
.prose li { margin-bottom: 0.4rem; }
.prose code {
  font-family: var(--font-mono);
  background: var(--surface);
  border: 1px solid var(--border);
  padding: 0.15em 0.45em;
  border-radius: var(--radius-sm);
  font-size: 0.88em;
  color: var(--primary-light);
}
.prose pre {
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: var(--radius);
  padding: 1.25rem;
  overflow-x: auto;
  margin-bottom: 1.25rem;
}
.prose pre code { background: none; border: none; padding: 0; }
.prose blockquote {
  border-left: 3px solid var(--primary);
  padding-left: 1.25rem;
  color: var(--text-muted);
  font-style: italic;
  margin-bottom: 1.25rem;
}
.prose a { color: var(--primary); text-decoration: underline; }

/* ─── Music Card ──────────────────────────────────────────── */
.music-card {
  max-width: 720px; margin: 3rem auto;
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: var(--radius-lg);
  overflow: hidden;
  display: grid;
  grid-template-columns: 280px 1fr;
  box-shadow: var(--shadow);
}
@media (max-width: 640px) { .music-card { grid-template-columns: 1fr; } }
.music-card__cover-wrap { overflow: hidden; }
.music-card__cover { width: 100%; height: 100%; object-fit: cover; min-height: 260px; }
.music-card__cover--placeholder {
  background: var(--surface);
  display: flex; align-items: center; justify-content: center;
  font-size: 4rem;
}
.music-card__info { padding: 2rem; display: flex; flex-direction: column; gap: 1rem; }
.music-card__title { font-size: 1.8rem; font-weight: 800; }
.music-card__artist { color: var(--primary); font-size: 1.05rem; font-weight: 600; }
.music-card__desc { color: var(--text-muted); }
.music-card__player { width: 100%; border-radius: var(--radius-sm); margin-top: auto; }

/* ─── Job ─────────────────────────────────────────────────── */
.job { max-width: 800px; margin: 3rem auto; }
.job__header { background: var(--bg-card); border: 1px solid var(--border); border-radius: var(--radius); padding: 2rem; margin-bottom: 2rem; }
.job__badges { display: flex; gap: 0.5rem; margin-bottom: 1rem; }
.job__title { font-size: 1.8rem; font-weight: 800; margin-bottom: 0.75rem; }
.job__meta { display: flex; gap: 1.5rem; color: var(--text-muted); align-items: center; }
.job__salary { color: var(--accent); font-weight: 700; font-size: 1.05rem; }
.job__body { margin-bottom: 2rem; }
.job__reqs { background: var(--surface); border-radius: var(--radius); padding: 1.5rem; margin-bottom: 2rem; }
.job__reqs h2 { margin-bottom: 1rem; font-size: 1.1rem; }
.job__reqs ul { padding-left: 1.25rem; }
.job__reqs li { margin-bottom: 0.5rem; color: var(--text-soft); }

/* ─── Empty state ─────────────────────────────────────────── */
.empty-state {
  text-align: center;
  padding: 5rem 2rem;
  color: var(--text-muted);
}
.empty-state__icon { font-size: 3.5rem; margin-bottom: 1.25rem; }
.empty-state h3 { font-size: 1.3rem; color: var(--text); margin-bottom: 0.5rem; }
.empty-state code {
  background: var(--surface);
  padding: 0.1em 0.4em;
  border-radius: var(--radius-sm);
  font-family: var(--font-mono);
  color: var(--primary-light);
  font-size: 0.9em;
}

/* ─── Error page ──────────────────────────────────────────── */
.error-page { text-align: center; }
.error-page__code {
  font-size: 8rem;
  font-weight: 900;
  line-height: 1;
  background: linear-gradient(135deg, var(--primary), var(--secondary));
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}
.error-page__title { font-size: 1.8rem; font-weight: 700; margin: 1rem 0 0.5rem; }
.error-page__desc { color: var(--text-muted); margin-bottom: 2rem; }

/* ─── Chat ────────────────────────────────────────────────── */
.chat { display: flex; flex-direction: column; height: 480px; background: var(--bg-card); border: 1px solid var(--border); border-radius: var(--radius); overflow: hidden; }
.chat__messages { flex: 1; overflow-y: auto; padding: 1.25rem; display: flex; flex-direction: column; gap: 0.75rem; }
.chat__msg { display: flex; flex-direction: column; gap: 0.15rem; }
.chat__msg-header { display: flex; gap: 0.5rem; align-items: baseline; }
.chat__msg-user { font-weight: 600; color: var(--primary); font-size: 0.85rem; }
.chat__msg-time { color: var(--text-muted); font-size: 0.75rem; }
.chat__msg-text { color: var(--text-soft); font-size: 0.92rem; }
.chat__msg--system .chat__msg-text { color: var(--text-muted); font-style: italic; text-align: center; font-size: 0.82rem; }
.chat__input { display: flex; border-top: 1px solid var(--border); }
.chat__input input {
  flex: 1; background: none; border: none; outline: none;
  padding: 1rem 1.25rem; color: var(--text); font-size: 0.92rem; font-family: var(--font);
}
.chat__input input::placeholder { color: var(--text-muted); }
.chat__input button {
  padding: 0 1.25rem;
  background: var(--primary);
  color: #fff;
  font-weight: 600;
  font-size: 0.9rem;
  transition: var(--transition);
}
.chat__input button:hover { background: var(--primary-dark); }

/* ─── Footer ──────────────────────────────────────────────── */
.footer {
  margin-top: 6rem;
  border-top: 1px solid var(--border);
  padding: 2.5rem 0;
}
.footer__inner {
  max-width: 1200px; margin: 0 auto;
  padding: 0 1.5rem;
  display: flex;
  align-items: center;
  gap: 2rem;
  flex-wrap: wrap;
}
.footer__brand { display: flex; align-items: center; gap: 0.5rem; font-weight: 600; }
.footer__links { display: flex; gap: 1.5rem; margin-left: auto; }
.footer__links a { color: var(--text-muted); font-size: 0.88rem; transition: var(--transition); }
.footer__links a:hover { color: var(--primary); }
.footer__copy { color: var(--text-muted); font-size: 0.82rem; width: 100%; }

/* ─── Scrollbar ───────────────────────────────────────────── */
::-webkit-scrollbar { width: 6px; }
::-webkit-scrollbar-track { background: var(--bg); }
::-webkit-scrollbar-thumb { background: var(--surface-2); border-radius: 3px; }
::-webkit-scrollbar-thumb:hover { background: var(--primary); }

/* ─── Selection ───────────────────────────────────────────── */
::selection { background: rgba(108,99,255,0.3); color: var(--text); }
"#
}

// ─── JS ───────────────────────────────────────────────────────────────────────

pub fn js_app() -> &'static str {
    r#"// Framework Oweeme — App JS
document.getElementById('burger')?.addEventListener('click', () => {
  document.querySelector('.navbar__nav')?.classList.toggle('is-open');
});
"#
}

pub fn js_chat() -> &'static str {
    r#"// Framework Oweeme — Chat WebSocket client
class OweemeChat {
  constructor(room, user, onMessage) {
    this.room = room; this.user = user; this.onMessage = onMessage;
    this.ws = null; this.delay = 1000; this._connect();
  }
  _connect() {
    const proto = location.protocol === 'https:' ? 'wss' : 'ws';
    this.ws = new WebSocket(`${proto}://${location.host}/ws/chat/${this.room}?user=${encodeURIComponent(this.user)}`);
    this.ws.onopen  = () => { this.delay = 1000; };
    this.ws.onmessage = e => { try { this.onMessage(JSON.parse(e.data)); } catch(_) {} };
    this.ws.onclose = () => { setTimeout(() => this._connect(), this.delay); this.delay = Math.min(this.delay*2,30000); };
    this.ws.onerror = () => this.ws.close();
  }
  send(text) { if (this.ws?.readyState === 1) this.ws.send(text.trim()); }
  close() { this.ws?.close(); }
}
window.OweemeChat = OweemeChat;
"#
}

// ─── Config ───────────────────────────────────────────────────────────────────

pub fn env_example(cfg: &ProjectConfig) -> String {
    format!(
        "PORT=3000\nSITE_URL={}\nSITE_NAME={}\nAPI_BASE_URL={}\nCACHE_TTL_SECS={}\nDEFAULT_LANG={}\nRUST_LOG=info\n",
        cfg.site_url, cfg.site_name, cfg.api_url, cfg.cache_ttl, cfg.default_lang
    )
}

pub fn gitignore() -> &'static str {
    "target/\ndist/\n.env\n*.pem\nstatic/wasm/\n"
}

// ─── Locales ──────────────────────────────────────────────────────────────────

pub fn all_locales() -> Vec<(&'static str, String)> {
    vec![
        ("es", locale_es()),
        ("en", locale_en()),
        ("pt", locale_pt()),
        ("de", locale_de()),
        ("fr", locale_fr()),
        ("ru", locale_ru()),
        ("ko", locale_ko()),
        ("ja", locale_ja()),
    ]
}

fn locale(data: &[(&str, &str)]) -> String {
    let inner: Vec<String> = data.iter()
        .map(|(k, v)| format!("  \"{k}\": \"{v}\""))
        .collect();
    format!("{{\n{}\n}}", inner.join(",\n"))
}

fn locale_es() -> String {
    locale(&[
        ("nav.home","Inicio"),("nav.music","Música"),("nav.articles","Artículos"),("nav.jobs","Trabajos"),
        ("home.hero_title","La plataforma para músicos"),("home.hero_desc","Comparte tu música, publica artículos y encuentra oportunidades."),("home.featured","Destacados"),
        ("articulo.by","Por {autor}"),("articulo.published","Publicado el {fecha}"),("articulo.read_more","Leer más"),("articulo.tags","Etiquetas"),
        ("musica.by","por {artista}"),("musica.listen","Escuchar"),("musica.no_audio","Audio no disponible"),
        ("trabajo.apply","Aplicar ahora"),("trabajo.location","Ubicación"),("trabajo.salary","Salario"),("trabajo.requirements","Requisitos"),
        ("error.not_found","Página no encontrada"),("error.server","Error interno del servidor"),
        ("footer.rights","Todos los derechos reservados"),
    ])
}
fn locale_en() -> String {
    locale(&[
        ("nav.home","Home"),("nav.music","Music"),("nav.articles","Articles"),("nav.jobs","Jobs"),
        ("home.hero_title","The platform for musicians"),("home.hero_desc","Share your music, publish articles and find opportunities."),("home.featured","Featured"),
        ("articulo.by","By {autor}"),("articulo.published","Published on {fecha}"),("articulo.read_more","Read more"),("articulo.tags","Tags"),
        ("musica.by","by {artista}"),("musica.listen","Listen"),("musica.no_audio","Audio not available"),
        ("trabajo.apply","Apply now"),("trabajo.location","Location"),("trabajo.salary","Salary"),("trabajo.requirements","Requirements"),
        ("error.not_found","Page not found"),("error.server","Internal server error"),
        ("footer.rights","All rights reserved"),
    ])
}
fn locale_pt() -> String {
    locale(&[
        ("nav.home","Início"),("nav.music","Música"),("nav.articles","Artigos"),("nav.jobs","Empregos"),
        ("home.hero_title","A plataforma para músicos"),("home.hero_desc","Compartilhe sua música, publique artigos e encontre oportunidades."),("home.featured","Destaques"),
        ("articulo.by","Por {autor}"),("articulo.published","Publicado em {fecha}"),("articulo.read_more","Ler mais"),("articulo.tags","Etiquetas"),
        ("musica.by","por {artista}"),("musica.listen","Ouvir"),("musica.no_audio","Áudio não disponível"),
        ("trabajo.apply","Candidatar-se"),("trabajo.location","Localização"),("trabajo.salary","Salário"),("trabajo.requirements","Requisitos"),
        ("error.not_found","Página não encontrada"),("error.server","Erro interno do servidor"),
        ("footer.rights","Todos os direitos reservados"),
    ])
}
fn locale_de() -> String {
    locale(&[
        ("nav.home","Startseite"),("nav.music","Musik"),("nav.articles","Artikel"),("nav.jobs","Jobs"),
        ("home.hero_title","Die Plattform für Musiker"),("home.hero_desc","Teile deine Musik, veröffentliche Artikel und finde Möglichkeiten."),("home.featured","Empfohlen"),
        ("articulo.by","Von {autor}"),("articulo.published","Veröffentlicht am {fecha}"),("articulo.read_more","Weiterlesen"),("articulo.tags","Tags"),
        ("musica.by","von {artista}"),("musica.listen","Anhören"),("musica.no_audio","Audio nicht verfügbar"),
        ("trabajo.apply","Jetzt bewerben"),("trabajo.location","Standort"),("trabajo.salary","Gehalt"),("trabajo.requirements","Anforderungen"),
        ("error.not_found","Seite nicht gefunden"),("error.server","Interner Serverfehler"),
        ("footer.rights","Alle Rechte vorbehalten"),
    ])
}
fn locale_fr() -> String {
    locale(&[
        ("nav.home","Accueil"),("nav.music","Musique"),("nav.articles","Articles"),("nav.jobs","Emplois"),
        ("home.hero_title","La plateforme pour les musiciens"),("home.hero_desc","Partagez votre musique, publiez des articles et trouvez des opportunités."),("home.featured","À la une"),
        ("articulo.by","Par {autor}"),("articulo.published","Publié le {fecha}"),("articulo.read_more","Lire la suite"),("articulo.tags","Étiquettes"),
        ("musica.by","par {artista}"),("musica.listen","Écouter"),("musica.no_audio","Audio non disponible"),
        ("trabajo.apply","Postuler maintenant"),("trabajo.location","Lieu"),("trabajo.salary","Salaire"),("trabajo.requirements","Exigences"),
        ("error.not_found","Page non trouvée"),("error.server","Erreur interne du serveur"),
        ("footer.rights","Tous droits réservés"),
    ])
}
fn locale_ru() -> String {
    locale(&[
        ("nav.home","Главная"),("nav.music","Музыка"),("nav.articles","Статьи"),("nav.jobs","Вакансии"),
        ("home.hero_title","Платформа для музыкантов"),("home.hero_desc","Делитесь музыкой, публикуйте статьи и находите возможности."),("home.featured","Рекомендуемое"),
        ("articulo.by","Автор: {autor}"),("articulo.published","Опубликовано {fecha}"),("articulo.read_more","Читать далее"),("articulo.tags","Теги"),
        ("musica.by","исполняет {artista}"),("musica.listen","Слушать"),("musica.no_audio","Аудио недоступно"),
        ("trabajo.apply","Откликнуться"),("trabajo.location","Местоположение"),("trabajo.salary","Зарплата"),("trabajo.requirements","Требования"),
        ("error.not_found","Страница не найдена"),("error.server","Внутренняя ошибка сервера"),
        ("footer.rights","Все права защищены"),
    ])
}
fn locale_ko() -> String {
    locale(&[
        ("nav.home","홈"),("nav.music","음악"),("nav.articles","기사"),("nav.jobs","구인"),
        ("home.hero_title","뮤지션을 위한 플랫폼"),("home.hero_desc","음악을 공유하고 기사를 게시하고 기회를 찾아보세요."),("home.featured","추천"),
        ("articulo.by","{autor} 작성"),("articulo.published","{fecha} 게시됨"),("articulo.read_more","더 읽기"),("articulo.tags","태그"),
        ("musica.by","{artista} 연주"),("musica.listen","듣기"),("musica.no_audio","오디오를 사용할 수 없습니다"),
        ("trabajo.apply","지금 지원"),("trabajo.location","위치"),("trabajo.salary","급여"),("trabajo.requirements","요구 사항"),
        ("error.not_found","페이지를 찾을 수 없습니다"),("error.server","내부 서버 오류"),
        ("footer.rights","모든 권리 보유"),
    ])
}
fn locale_ja() -> String {
    locale(&[
        ("nav.home","ホーム"),("nav.music","音楽"),("nav.articles","記事"),("nav.jobs","求人"),
        ("home.hero_title","ミュージシャンのためのプラットフォーム"),("home.hero_desc","音楽を共有し記事を投稿し機会を見つけましょう。"),("home.featured","おすすめ"),
        ("articulo.by","{autor} 著"),("articulo.published","{fecha} 公開"),("articulo.read_more","続きを読む"),("articulo.tags","タグ"),
        ("musica.by","{artista} 演奏"),("musica.listen","聴く"),("musica.no_audio","オーディオは利用できません"),
        ("trabajo.apply","今すぐ応募"),("trabajo.location","場所"),("trabajo.salary","給与"),("trabajo.requirements","要件"),
        ("error.not_found","ページが見つかりません"),("error.server","内部サーバーエラー"),
        ("footer.rights","全著作権所有"),
    ])
}

// ─── Vue.js frontend ──────────────────────────────────────────────────────────

pub fn vue_package_json(name: &str) -> String {
    format!(
        r#"{{
  "name": "{name}-frontend",
  "version": "0.1.0",
  "scripts": {{
    "dev": "vite",
    "build": "vite build",
    "preview": "vite preview"
  }},
  "dependencies": {{
    "vue": "^3.4.0",
    "quasar": "^2.16.0",
    "@quasar/extras": "^1.16.0"
  }},
  "devDependencies": {{
    "@vitejs/plugin-vue": "^5.0.0",
    "vite": "^5.0.0",
    "unplugin-vue-components": "^0.27.0"
  }}
}}
"#
    )
}

pub fn vue_vite_config() -> &'static str {
    r#"import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import { quasar, transformAssetUrls } from '@quasar/vite-plugin'

export default defineConfig({
  plugins: [
    vue({ template: { transformAssetUrls } }),
    quasar({ sassVariables: false }),
  ],
  build: {
    outDir: '../static/js',
    rollupOptions: {
      input: 'src/main.js',
      output: { entryFileNames: 'vue-app.js', chunkFileNames: '[name].js' }
    }
  },
  server: { proxy: { '/api': 'http://localhost:3000', '/ws': { target: 'ws://localhost:3000', ws: true } } }
})
"#
}

pub fn vue_index_html(site_name: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="es">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>{site_name} — Dev</title>
</head>
<body>
  <div id="app"></div>
  <script type="module" src="/src/main.js"></script>
</body>
</html>
"#
    )
}

pub fn vue_main_js() -> &'static str {
    r#"import { createApp } from 'vue'
import { Quasar } from 'quasar'
import '@quasar/extras/material-icons/material-icons.css'
import 'quasar/src/css/index.sass'
import App from './App.vue'

const app = createApp(App)
app.use(Quasar, {
  config: {
    dark: true,
    brand: { primary: '#6c63ff', secondary: '#ff6584', accent: '#43e97b' }
  }
})
app.mount('#app')
"#
}

pub fn vue_app() -> &'static str {
    r#"<script setup>
import { RouterView } from 'vue-router'
</script>

<template>
  <router-view />
</template>
"#
}

pub fn vue_composable_oweeme() -> &'static str {
    r#"// Composable para usar el módulo WASM de Oweeme en Vue
let _initialized = false

export async function useOweeme() {
  if (!_initialized) {
    const { default: init } = await import('/static/wasm/oweeme_wasm.js')
    await init()
    _initialized = true
  }
  const mod = await import('/static/wasm/oweeme_wasm.js')
  return {
    slug: mod.slug,
    validateSeo: mod.validate_seo,
    readingTime: mod.reading_time_minutes,
    truncateSeo: mod.truncate_seo,
  }
}
"#
}

pub fn vue_composable_chat() -> &'static str {
    r#"import { ref, onUnmounted } from 'vue'

export function useChat(room, username) {
  const messages = ref([])
  const connected = ref(false)
  let chat = null

  if (typeof window !== 'undefined' && window.OweemeChat) {
    chat = new window.OweemeChat(room, username, (msg) => {
      messages.value.push(msg)
      // Mantener máximo 200 mensajes en memoria
      if (messages.value.length > 200) messages.value.shift()
    })
    connected.value = true
  }

  const send = (text) => { if (text.trim()) chat?.send(text) }

  onUnmounted(() => chat?.close())

  return { messages, connected, send }
}
"#
}

pub fn vue_chat_component() -> &'static str {
    r#"<script setup>
import { ref } from 'vue'
import { useChat } from '@/composables/useChat'

const props = defineProps({
  room:     { type: String, required: true },
  username: { type: String, required: true },
})

const input = ref('')
const { messages, connected, send } = useChat(props.room, props.username)

const submit = () => {
  send(input.value)
  input.value = ''
}
</script>

<template>
  <q-card class="chat-box" dark bordered>
    <q-card-section class="q-py-sm row items-center">
      <span class="text-subtitle2">{{ room }}</span>
      <q-badge :color="connected ? 'positive' : 'negative'" class="q-ml-sm">
        {{ connected ? 'En línea' : 'Desconectado' }}
      </q-badge>
    </q-card-section>

    <q-separator dark />

    <q-scroll-area class="chat-box__messages">
      <div class="q-pa-md q-gutter-sm">
        <div v-for="m in messages" :key="m.id">
          <div v-if="m.type === 'system'" class="text-caption text-grey text-center">{{ m.text }}</div>
          <div v-else>
            <span class="text-primary text-weight-bold text-caption">{{ m.user }}</span>
            <span class="text-grey-5 text-caption q-ml-xs">{{ new Date(m.timestamp).toLocaleTimeString() }}</span>
            <div class="text-body2">{{ m.text }}</div>
          </div>
        </div>
      </div>
    </q-scroll-area>

    <q-separator dark />

    <q-card-section class="q-py-sm">
      <q-input
        v-model="input"
        dark dense filled
        placeholder="Escribe un mensaje..."
        @keyup.enter="submit"
      >
        <template #append>
          <q-btn flat round icon="send" color="primary" @click="submit" />
        </template>
      </q-input>
    </q-card-section>
  </q-card>
</template>

<style scoped>
.chat-box { height: 480px; display: flex; flex-direction: column; }
.chat-box__messages { flex: 1; height: 380px; }
</style>
"#
}

pub fn vue_page_home() -> &'static str {
    r#"<script setup>
import { ref } from 'vue'
import ChatBox from '@/components/ChatBox.vue'

// Datos que Rust ya puso en el HTML (cero fetch extra)
const serverData = ref(null)
try {
  const el = document.getElementById('__oweeme_data__')
  if (el) serverData.value = JSON.parse(el.textContent)
} catch (_) {}
</script>

<template>
  <q-page padding>
    <div class="row q-col-gutter-lg">
      <div class="col-12 col-md-8">
        <h2 class="text-h5 q-mb-md">Contenido destacado</h2>
        <div class="row q-col-gutter-md">
          <div
            v-for="item in serverData?.destacados"
            :key="item.slug"
            class="col-12 col-sm-6"
          >
            <q-card dark bordered class="full-height">
              <q-img v-if="item.imagen" :src="item.imagen" :ratio="16/9" />
              <q-card-section>
                <div class="text-subtitle2">{{ item.titulo }}</div>
                <div class="text-caption text-grey">{{ item.descripcion }}</div>
              </q-card-section>
              <q-card-actions>
                <q-btn flat color="primary" :href="`/articulo/${item.slug}`" label="Ver más" />
              </q-card-actions>
            </q-card>
          </div>
        </div>
      </div>

      <div class="col-12 col-md-4">
        <h2 class="text-h5 q-mb-md">Chat en vivo</h2>
        <ChatBox room="general" username="Visitante" />
      </div>
    </div>
  </q-page>
</template>
"#
}
