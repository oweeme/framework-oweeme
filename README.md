# Framework Oweeme

**SEO-first web framework written in Rust.** Renders complete HTML on the server, consumes any external API (PHP, Django, Flask, Go, Node), and delivers fully indexable pages to search engines — no Node.js required on the backend.

> Built for musicians, artists, and content-heavy platforms that need top-tier SEO without sacrificing speed or modern interactivity.

---

## Table of Contents

- [Why Oweeme](#why-oweeme)
- [Requirements](#requirements)
- [Installation](#installation)
- [Creating a project](#creating-a-project)
- [Project structure](#project-structure)
- [Running the project](#running-the-project)
- [Environment variables](#environment-variables)
- [Templates & Components](#templates--components)
- [Internationalization — 8 languages](#internationalization--8-languages)
- [API Connector & Cache](#api-connector--cache)
- [SEO Module](#seo-module)
- [Real-time Chat (WebSocket)](#real-time-chat-websocket)
- [Vue.js + Quasar Integration](#vuejs--quasar-integration)
- [Plugin System](#plugin-system)
- [WASM Module](#wasm-module)
- [Production Deployment](#production-deployment)
- [SEO Checklist](#seo-checklist)

---

## Why Oweeme

| | Oweeme | Next.js | Laravel |
|---|---|---|---|
| Language | Rust | Node.js | PHP |
| Node.js required | **No** | Yes | No |
| Single binary deploy | **Yes** | No | No |
| RAM usage | **~5 MB** | ~200 MB | ~50 MB |
| SEO out of the box | **Full** | Partial | Manual |
| WebSockets built-in | **Yes** | External | External |
| i18n built-in | **8 languages** | External | External |
| Vue.js + Quasar ready | **Yes** | Yes | No |

---

## Requirements

- **Rust 1.75+**
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Git**
- A running API backend in any language (PHP, Python, Go, Node, etc.)

---

## Installation

### Step 1 — Clone the framework

```bash
git clone https://github.com/oweeme/framework-oweeme
cd framework-oweeme
```

### Step 2 — Install the CLI globally

```bash
cargo install --path cli
```

This compiles and installs the `oweeme` command on your system. You only do this once.

Verify it works:

```bash
oweeme --version
# oweeme 1.0.0

oweeme info
```

You will see:

```
  ██████╗ ██╗    ██╗███████╗███████╗███╗   ███╗███████╗
  ...

  SEO-first Rust + Vue.js Framework • v1.0.0

Commands

    oweeme new <name>    Scaffold a new project
    oweeme info          Show framework info
```

---

## Creating a project

From any folder on your system:

```bash
oweeme new my-project
```

The CLI will ask you a series of questions:

```
Site name          → My Music Platform
Production URL     → https://myplatform.com
API backend URL    → https://api.myplatform.com
Default language   → es — Español
API cache TTL      → 300s (recommended)
Include Vue.js?    → Yes
Enable PWA?        → Yes
```

After answering, it generates the complete project instantly:

```
[1/7] Project structure     ✓ directories created
[2/7] Cargo.toml            ✓ dependencies configured
[3/7] Rust source           ✓ src/main.rs
[4/7] HTML templates        ✓ base.html + pages + components
[5/7] Static assets         ✓ CSS design system + JS
[6/7] Config & i18n         ✓ .env + 8 locales (es/en/pt/de/fr/ru/ko/ja)
[7/7] Vue.js frontend       ✓ Vue 3 + Vite + Quasar + composables

──────────────────────────────────────────────────

  Project ready: my-project

  Next steps:

  1.  cd my-project
  2.  cp .env.example .env
  3.  # Edit .env → set API_BASE_URL=https://api.myplatform.com
  4.  cargo run

  Server will start at http://localhost:3000

  Vue.js frontend: cd frontend && npm install && npm run dev
```

---

## Project structure

```
my-project/
│
├── src/
│   └── main.rs                  ← Rust server entry point
│
├── Cargo.toml                   ← Dependencies (uses framework_oweeme)
│
├── templates/                   ← Server-side HTML (what Google indexes)
│   ├── base.html                ← Master layout with full SEO tags
│   ├── pages/
│   │   ├── home.html
│   │   ├── articulo.html        ← Article page (schema.org/Article)
│   │   ├── musica.html          ← Music page (schema.org/MusicRecording)
│   │   ├── trabajo.html         ← Job page (schema.org/JobPosting)
│   │   └── 404.html
│   └── components/
│       ├── Navbar.html          ← Navigation bar
│       ├── Footer.html
│       ├── Hero.html            ← Landing hero section
│       └── Card.html            ← Reusable content card
│
├── static/
│   ├── css/
│   │   └── app.css              ← Dark design system (Material-inspired)
│   └── js/
│       ├── app.js               ← Minimal vanilla JS
│       └── chat.js              ← WebSocket chat client
│
├── locales/                     ← Translation files
│   ├── es.json                  ← Spanish
│   ├── en.json                  ← English
│   ├── pt.json                  ← Portuguese
│   ├── de.json                  ← German
│   ├── fr.json                  ← French
│   ├── ru.json                  ← Russian (Cyrillic)
│   ├── ko.json                  ← Korean (Hangul)
│   └── ja.json                  ← Japanese (Kanji/Hiragana)
│
├── frontend/                    ← Vue 3 + Quasar (optional)
│   ├── src/
│   │   ├── main.js
│   │   ├── App.vue
│   │   ├── composables/
│   │   │   ├── useChat.js       ← WebSocket chat composable
│   │   │   └── useOweeme.js     ← WASM composable
│   │   ├── components/
│   │   │   └── ChatBox.vue      ← Real-time chat with Quasar UI
│   │   └── pages/
│   │       └── Home.vue
│   ├── vite.config.js
│   └── package.json
│
├── .env.example                 ← Environment config template
└── .gitignore
```

---

## Running the project

### Development

```bash
# Terminal 1 — Rust server (port 3000)
cd my-project
cp .env.example .env
cargo run

# Terminal 2 — Vue.js frontend (port 5173, optional)
cd my-project/frontend
npm install
npm run dev
```

Visit `http://localhost:3000` — the Rust server serves the full SEO HTML.

Visit `http://localhost:5173` — Vue.js dev server with hot reload.

### Build for production

```bash
cargo build --release
# Binary at: target/release/my-project (~8 MB, no dependencies needed)
```

---

## Environment variables

Edit `.env` (copied from `.env.example`):

| Variable | Default | Description |
|---|---|---|
| `PORT` | `3000` | Server port |
| `SITE_URL` | `http://localhost:3000` | Public URL — used in canonical links, Open Graph, sitemap |
| `SITE_NAME` | `Mi Plataforma` | Site name for `<title>` tags and PWA manifest |
| `API_BASE_URL` | `http://localhost:8080` | Your backend API base URL |
| `CACHE_TTL_SECS` | `300` | How long to cache API responses (seconds) |
| `DEFAULT_LANG` | `es` | Default language code |
| `RUST_LOG` | `info` | Log level: `error` / `warn` / `info` / `debug` |

---

## Templates & Components

Templates use **Tera** syntax (similar to Jinja2 / Django templates), extended with Vue-like self-closing component tags.

### Pages extend `base.html`

```html
{% extends "base.html" %}

{% block content %}
<div class="container">
  <h1>{{ titulo }}</h1>
  <p>{{ descripcion }}</p>
</div>
{% endblock content %}
```

### Reusable components

Define `templates/components/ArtistCard.html`:

```html
<div class="card">
  <img src="{{ foto }}" alt="{{ nombre }}">
  <h3>{{ nombre }}</h3>
  <p>{{ genero }}</p>
</div>
```

Use it anywhere with the self-closing Vue-like syntax:

```html
<ArtistCard nombre="Héctor" genero="Rock" foto="/img/hector.jpg" />
```

The framework pre-processes this into:

```html
{% set nombre = "Héctor" %}
{% set genero = "Rock" %}
{% set foto = "/img/hector.jpg" %}
{% include "components/ArtistCard.html" %}
```

### Variables available in every template

All these are injected automatically — you don't need to set them manually:

```
{{ seo_title }}           → page title
{{ seo_description }}     → meta description
{{ seo_og_image }}        → Open Graph image
{{ seo_schema_json }}     → schema.org JSON-LD
{{ lang }}                → detected language code ("es", "en", ...)
{{ t["nav.home"] }}       → translated string
{{ api_data }}            → full JSON from your API
```

---

## Internationalization — 8 languages

The framework automatically detects the user's language from the `Accept-Language` browser header.

### Supported languages out of the box

| Code | Language | Script |
|------|----------|--------|
| `es` | Spanish | Latin |
| `en` | English | Latin |
| `pt` | Portuguese | Latin |
| `de` | German | Latin |
| `fr` | French | Latin |
| `ru` | Russian | Cyrillic (кириллица) |
| `ko` | Korean | Hangul (한글) |
| `ja` | Japanese | Kanji/Hiragana/Katakana (漢字) |

### Using translations in templates

```html
<a href="/">{{ t["nav.home"] }}</a>
<h2>{{ t["home.featured"] }}</h2>
<span>{{ t["articulo.by"] }}</span>
```

### Adding a new language

Simply create a new file in `locales/`:

```bash
touch locales/ar.json
```

```json
{
  "nav.home": "الرئيسية",
  "nav.music": "موسيقى",
  "home.hero_title": "منصة الموسيقيين",
  "footer.rights": "جميع الحقوق محفوظة"
}
```

Restart the server — it loads automatically.

---

## API Connector & Cache

The framework connects to your existing backend via HTTP. Your API can be written in **any language**.

```rust
// In your route handler:
let data = state.api.get_json("/articulos/my-slug").await?;
```

### Expected API format

```json
// GET /articulos/my-slug
{
  "titulo": "My Article Title",
  "descripcion": "Short description for SEO...",
  "autor": "Héctor",
  "fecha": "2024-06-23",
  "imagen": "https://cdn.example.com/cover.jpg",
  "tags": "music,rock,indie",
  "contenido": "<p>Full HTML content...</p>"
}
```

### Cache configuration

Responses are cached in memory automatically:

```bash
CACHE_TTL_SECS=300   # cache for 5 minutes (default)
CACHE_TTL_SECS=0     # disable cache
CACHE_TTL_SECS=3600  # cache for 1 hour
```

### Sitemap API endpoints

For the dynamic sitemap to work, expose these from your API:

```
GET /sitemap/articulos  →  [{ "slug": "my-article", "updated_at": "2024-06-23" }]
GET /sitemap/musica     →  [{ "slug": "my-song",    "updated_at": "2024-06-20" }]
GET /sitemap/trabajos   →  [{ "slug": "job-offer",  "updated_at": "2024-06-22" }]
```

---

## SEO Module

Every route automatically generates full SEO metadata. Example for an article:

```
Title:        My Article Title
Description:  Short description for SEO...
Canonical:    https://mysite.com/articulo/my-article
OG title:     My Article Title
OG image:     https://cdn.example.com/cover.jpg
OG type:      article
Twitter card: summary_large_image
Schema.org:   { "@type": "Article", "author": "Héctor", ... }
```

### Schema.org types supported

| Route | Schema type |
|---|---|
| `/articulo/:slug` | `Article` |
| `/musica/:slug` | `MusicRecording` |
| `/trabajo/:slug` | `JobPosting` |

---

## Real-time Chat (WebSocket)

Chat rooms are built in. No external service needed.

### Connect from JavaScript

```html
<!-- Already included in base.html -->
<script src="/static/js/chat.js"></script>
```

```js
const chat = new OweemeChat('room-name', 'Username', (msg) => {
  console.log(msg.user, ':', msg.text)
})

chat.send('Hello everyone!')
chat.close()
```

### WebSocket endpoint

```
ws://your-domain.com/ws/chat/{room}?user={username}
```

### Message format

```json
{
  "id": "uuid-v4",
  "room": "room-name",
  "user": "Username",
  "text": "Hello everyone!",
  "timestamp": 1719100000000,
  "type": "chat"
}
```

Message types: `chat` | `join` | `leave` | `system`

### Active rooms API

```
GET /api/chat/rooms  →  ["room-name", "another-room"]
```

---

## Vue.js + Quasar Integration

The generated `frontend/` folder contains a complete Vue 3 + Quasar setup.

### How it works

```
1. User requests /articulo/my-article
2. Rust renders complete HTML with all SEO tags → Google indexes this ✓
3. Vue.js loads and hydrates the page → user gets interactivity ✓
4. WebSocket connects → real-time chat works ✓
```

### Start the Vue dev server

```bash
cd frontend
npm install
npm run dev        # runs on http://localhost:5173
```

### Build Vue for production (outputs to static/)

```bash
cd frontend
npm run build      # outputs to ../static/js/vue-app.js
```

### Pass server data to Vue (zero extra API calls)

In your Tera template, Rust embeds the data:

```html
<script id="__oweeme_data__" type="application/json">
  {{ api_data | json_encode | safe }}
</script>
```

In Vue, read it directly:

```js
const serverData = JSON.parse(
  document.getElementById('__oweeme_data__').textContent
)
```

### Chat composable (useChat.js)

```vue
<script setup>
import { ref } from 'vue'
import { useChat } from '@/composables/useChat'

const input = ref('')
const { messages, connected, send } = useChat('general', 'MyUser')
</script>

<template>
  <div v-for="m in messages" :key="m.id">
    <strong>{{ m.user }}</strong>: {{ m.text }}
  </div>
  <input v-model="input" @keyup.enter="send(input); input = ''" />
</template>
```

### ChatBox.vue — ready to use with Quasar

```vue
<template>
  <ChatBox room="general" username="MyUser" />
</template>

<script setup>
import ChatBox from '@/components/ChatBox.vue'
</script>
```

---

## Plugin System

Extend the framework with custom routes and logic.

```rust
use framework_oweeme::plugin::{Plugin, PluginMeta};
use axum::{routing::get, Router, Json};

pub struct MyPlugin;

impl Plugin for MyPlugin {
    fn meta(&self) -> PluginMeta {
        PluginMeta {
            name: "my-plugin",
            version: "1.0.0",
            description: "My custom plugin",
        }
    }

    fn routes(&self) -> Option<Router> {
        Some(Router::new().route("/api/custom", get(|| async {
            Json(serde_json::json!({ "ok": true }))
        })))
    }
}
```

Register in `main.rs`:

```rust
PluginRegistry::new()
    .register(HealthPlugin { version: "1.0.0" })
    .register(RssPlugin { ... })
    .register(MyPlugin)
    .build();
```

### Built-in plugins

| Plugin | Route | Description |
|---|---|---|
| `HealthPlugin` | `GET /health` | Server health check |
| `RssPlugin` | `GET /rss.xml` | RSS feed from your API |

---

## WASM Module

Rust logic compiled to WebAssembly that runs in the browser.

### Build

```bash
# Install wasm-pack (once)
cargo install wasm-pack

# Build the WASM module
cd wasm
wasm-pack build --target web --out-dir ../static/wasm --release
```

### Use in Vue (useOweeme.js composable)

```js
import { useOweeme } from '@/composables/useOweeme'

const { slug, validateSeo, readingTime, truncateSeo } = await useOweeme()

slug('La Novia Baila!')          // → "la-novia-baila"
readingTime(longText)            // → 4  (minutes)
truncateSeo(text, 160)           // → "Truncated text without cutting words…"

const result = validateSeo('My title', 'My description')
result.valid          // true / false
result.issues_text()  // "Description too short (13 chars, min 50)"
```

---

## Production Deployment

### 1. Build the binary

```bash
cargo build --release
# → target/release/my-project  (~8 MB, single file, no dependencies)
```

### 2. Copy files to your server

```bash
scp target/release/my-project  user@server:/srv/myapp/
scp -r templates static locales  user@server:/srv/myapp/
scp .env.example  user@server:/srv/myapp/.env
# Edit .env on the server with production values
```

### 3. Nginx configuration

```nginx
server {
    listen 443 ssl http2;
    server_name myplatform.com;

    # Static files — long cache
    location /static/ {
        proxy_pass http://127.0.0.1:3000;
        expires 1y;
        add_header Cache-Control "public, immutable";
    }

    # WebSocket
    location /ws/ {
        proxy_pass http://127.0.0.1:3000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_read_timeout 86400;
    }

    # Everything else
    location / {
        proxy_pass http://127.0.0.1:3000;
        proxy_set_header Host $host;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

### 4. Run as a systemd service

```bash
sudo nano /etc/systemd/system/myapp.service
```

```ini
[Unit]
Description=My Oweeme App
After=network.target

[Service]
Type=simple
User=www-data
WorkingDirectory=/srv/myapp
EnvironmentFile=/srv/myapp/.env
ExecStart=/srv/myapp/my-project
Restart=on-failure

[Install]
WantedBy=multi-user.target
```

```bash
sudo systemctl enable myapp
sudo systemctl start myapp
sudo systemctl status myapp
```

---

## SEO Checklist

Everything handled automatically on every page:

- [x] `<title>` — dynamic per page
- [x] `<meta name="description">` — dynamic
- [x] `<meta name="keywords">` — optional
- [x] `<link rel="canonical">` — prevents duplicate content penalties
- [x] Open Graph tags (`og:title`, `og:description`, `og:image`, `og:type`, `og:url`)
- [x] Twitter Card tags (`summary_large_image`)
- [x] `schema.org` JSON-LD (Article / MusicRecording / JobPosting)
- [x] `<html lang="...">` — set from i18n detection
- [x] `/sitemap.xml` — dynamic, built from your API slugs
- [x] `/robots.txt` — auto-generated with correct Sitemap reference
- [x] `/manifest.json` — PWA ready
- [x] `/rss.xml` — RSS feed via built-in plugin
- [x] `Cache-Control` headers — tuned per content type
- [x] Security headers (CSP, X-Frame-Options, Referrer-Policy, Permissions-Policy)

---

## License

MIT — free to use, modify and distribute.

---

*Framework Oweeme — Built with Rust.*
