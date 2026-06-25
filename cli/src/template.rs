use crate::scaffold::ProjectConfig;

// ─── package.json ─────────────────────────────────────────────────────────────

pub fn package_json(cfg: &ProjectConfig) -> String {
    let name  = &cfg.name;
    let lint_script = if cfg.with_linting() {
        r#"    "lint":    "oxlint . && eslint .",
    "format": "prettier --write src/","#
    } else { "" };
    let build = if cfg.is_ts() { "vue-tsc && vite build" } else { "vite build" };
    let ts_deps = if cfg.is_ts() {
        r#"    "typescript":                    "5.7.3",
    "vue-tsc":                       "2.2.8","#
    } else { "" };
    let lint_deps = if cfg.with_linting() {
        let ts_lint = if cfg.is_ts() {
            r#"    "typescript-eslint":             "8.18.0",
    "@typescript-eslint/parser":     "8.18.0","#
        } else { "" };
        format!(r#"    "@eslint/js":                    "9.17.0",
    "eslint":                        "9.17.0",
    "eslint-plugin-vue":             "9.32.0",
    "vue-eslint-parser":             "9.4.3",
{ts_lint}
    "oxlint":                        "0.14.0",
    "prettier":                      "3.4.2","#)
    } else { String::new() };

    format!(
        r#"{{
  "name": "{name}",
  "version": "0.1.0",
  "private": true,
  "scripts": {{
    "dev":     "vite",
    "build":   "{build}",
    "preview": "vite preview"{lint_sep}{lint_script}
  }},
  "dependencies": {{
    "quasar":          "2.17.4",
    "@quasar/extras":  "1.16.13",
    "vue":             "3.5.13",
    "vue-router":      "4.3.0",
    "pinia":           "2.2.6",
    "axios":           "1.7.9"
  }},
  "devDependencies": {{
    "vite":                "5.4.11",
    "@vitejs/plugin-vue":  "5.2.1",
    "@quasar/vite-plugin": "1.7.0",
{ts_deps}
    "sass":                "1.77.6",
    "vite-plugin-pwa":     "0.20.5",
    "workbox-window":      "7.3.0"{lint_sep2}{lint_deps}
  }}
}}
"#,
        lint_sep  = if cfg.with_linting() { ",\n" } else { "" },
        lint_sep2 = if cfg.with_linting() { ",\n" } else { "" },
    )
}

// ─── vite.config.ts / vite.config.js ─────────────────────────────────────────

pub fn vite_config(cfg: &ProjectConfig) -> String {
    let dc = if cfg.is_ts() { "defineConfig" } else { "defineConfig" };
    let _ = dc;
    format!(r#"import {{ defineConfig }} from 'vite'
import vue from '@vitejs/plugin-vue'
import {{ quasar, transformAssetUrls }} from '@quasar/vite-plugin'
import {{ VitePWA }} from 'vite-plugin-pwa'
import {{ fileURLToPath, URL }} from 'node:url'

export default defineConfig({{
  plugins: [
    vue({{ template: {{ transformAssetUrls }} }}),
    quasar({{
      sassVariables: 'src/css/quasar.variables.scss',
    }}),
    VitePWA({{
      registerType: 'autoUpdate',
      includeAssets: ['oweelogo.png', 'robots.txt'],
      manifest: false,
      workbox: {{
        globPatterns: ['**/*.{{js,css,html,ico,png,svg,woff2}}'],
      }},
    }}),
  ],
  resolve: {{
    alias: {{ '@': fileURLToPath(new URL('./src', import.meta.url)) }},
  }},
}})
"#)
}

// ─── tsconfig.json ────────────────────────────────────────────────────────────

pub fn tsconfig() -> &'static str {
    r#"{
  "compilerOptions": {
    "target":           "ESNext",
    "module":           "ESNext",
    "moduleResolution": "bundler",
    "strict":           true,
    "jsx":              "preserve",
    "noEmit":           true,
    "lib":              ["ESNext", "DOM"],
    "baseUrl":          ".",
    "paths":            { "@/*": ["src/*"] },
    "types":            ["quasar", "vite/client"]
  },
  "include": ["src/**/*.ts", "src/**/*.vue"],
  "exclude": ["node_modules", "dist"]
}
"#
}

// ─── jsconfig.json ────────────────────────────────────────────────────────────

pub fn jsconfig() -> &'static str {
    r#"{
  "compilerOptions": {
    "target":           "ESNext",
    "module":           "ESNext",
    "moduleResolution": "bundler",
    "jsx":              "preserve",
    "baseUrl":          ".",
    "paths":            { "@/*": ["src/*"] }
  },
  "include": ["src/**/*.js", "src/**/*.vue"],
  "exclude": ["node_modules", "dist"]
}
"#
}

// ─── .env.example ─────────────────────────────────────────────────────────────

pub fn env_example(cfg: &ProjectConfig) -> String {
    format!(
        "VITE_SITE_URL={}\nVITE_SITE_NAME={}\n# VITE_API_BASE=https://api.tudominio.com\n",
        cfg.site_url, cfg.site_name
    )
}

// ─── .gitignore ───────────────────────────────────────────────────────────────

pub fn gitignore() -> &'static str {
    "node_modules/\ndist/\n.env\n*.local\n.DS_Store\n"
}

// ─── src/boot/axios.ts / axios.js ─────────────────────────────────────────────

pub fn boot_axios(cfg: &ProjectConfig) -> String {
    if cfg.is_ts() {
        r#"import axios, { type AxiosInstance } from 'axios'

const api: AxiosInstance = axios.create({
  baseURL: import.meta.env.VITE_API_BASE || '/api',
  timeout: 15000,
  headers: { 'Content-Type': 'application/json' },
})

// Interceptor — adjunta token si existe
api.interceptors.request.use((config) => {
  const token = localStorage.getItem('token')
  if (token) config.headers.Authorization = `Bearer ${token}`
  return config
})

// Interceptor — manejo global de errores
api.interceptors.response.use(
  (res) => res,
  (err) => {
    if (err.response?.status === 401) {
      localStorage.removeItem('token')
      window.location.href = '/login'
    }
    return Promise.reject(err)
  }
)

export { api }
export default api
"#.to_string()
    } else {
        r#"import axios from 'axios'

const api = axios.create({
  baseURL: import.meta.env.VITE_API_BASE || '/api',
  timeout: 15000,
  headers: { 'Content-Type': 'application/json' },
})

// Interceptor — adjunta token si existe
api.interceptors.request.use((config) => {
  const token = localStorage.getItem('token')
  if (token) config.headers.Authorization = `Bearer ${token}`
  return config
})

// Interceptor — manejo global de errores
api.interceptors.response.use(
  (res) => res,
  (err) => {
    if (err.response?.status === 401) {
      localStorage.removeItem('token')
      window.location.href = '/login'
    }
    return Promise.reject(err)
  }
)

export { api }
export default api
"#.to_string()
    }
}

// ─── oweeme.config.ts / oweeme.config.js ─────────────────────────────────────

pub fn oweeme_config(cfg: &ProjectConfig) -> String {
    let icons: Vec<String> = cfg.icon_sets.iter()
        .map(|i| format!("    '{}',", i.import().replace("import '", "").replace("'", "")))
        .collect();
    let plugins: Vec<String> = cfg.qplugins.iter()
        .map(|p| format!("    '{}',", p.name()))
        .collect();
    let lang_str    = if cfg.is_ts() { "typescript" } else { "javascript" };
    let linter_str  = if cfg.with_linting() { "eslint+oxlint" } else { "none" };
    let pwa_str     = cfg.with_pwa.to_string();
    let icons_str   = icons.join("\n");
    let plugins_str = plugins.join("\n");
    format!(r#"// oweeme.config — configuración del proyecto
// Generado por Oweeme Framework CLI
// Modifica este archivo y re-ejecuta `oweeme sync` para regenerar main.ts

export default {{
  lang:    '{lang_str}',
  pwa:     {pwa_str},
  linting: '{linter_str}',

  // Icon sets activos — ver: https://quasar.dev/options/quasar-icon-sets
  iconSets: [
{icons_str}
  ],

  // Plugins Quasar activos — ver: https://quasar.dev/quasar-plugins
  plugins: [
{plugins_str}
  ],
}}
"#)
}

// ─── ESLint 9 flat config ─────────────────────────────────────────────────────

pub fn eslint_config(cfg: &ProjectConfig) -> String {
    let ts_extra = if cfg.is_ts() {
        r#"
  // TypeScript
  ...tseslint.configs.recommended,"#
    } else { "" };
    format!(r#"import js from '@eslint/js'
import vue from 'eslint-plugin-vue'
import vueParser from 'vue-eslint-parser'{ts_import}

{ts_extra_import}export default [
  js.configs.recommended,
  ...vue.configs['flat/recommended'],{ts_extra}
  {{
    files: ['**/*.vue', '**/*.{ext}'],
    languageOptions: {{
      parser: vueParser,
      {ts_parser}
      globals: {{
        window:    'readonly',
        document:  'readonly',
        console:   'readonly',
        process:   'readonly',
        __dirname: 'readonly',
      }},
    }},
    rules: {{
      'vue/multi-word-component-names': 'off',
      'vue/no-unused-vars':             'error',
      'vue/component-api-style':        ['error', ['script-setup']],
      'no-console':                     ['warn', {{ allow: ['warn', 'error'] }}],
      'no-unused-vars':                 'warn',
    }},
  }},
  {{
    ignores: ['dist/**', 'node_modules/**'],
  }},
]
"#,
        ext          = cfg.ext(),
        ts_import    = if cfg.is_ts() { "\nimport tseslint from 'typescript-eslint'" } else { "" },
        ts_extra_import = "",
        ts_extra     = ts_extra,
        ts_parser    = if cfg.is_ts() { "parserOptions: { parser: '@typescript-eslint/parser' }," } else { "" },
    )
}

// ─── oxlint config ────────────────────────────────────────────────────────────

pub fn oxlint_config() -> &'static str {
    r#"{
  "rules": {
    "no-unused-vars":    "warn",
    "no-console":        "warn",
    "eqeqeq":           "error",
    "no-var":           "error",
    "prefer-const":     "error"
  },
  "ignore": ["dist", "node_modules"]
}
"#
}

// ─── .prettierrc ─────────────────────────────────────────────────────────────

pub fn prettier_config() -> &'static str {
    r#"{
  "semi":         false,
  "singleQuote":  true,
  "tabWidth":     2,
  "trailingComma":"es5",
  "printWidth":   100,
  "vueIndentScriptAndStyle": false
}
"#
}

// ─── public/.htaccess (Apache SPA routing) ────────────────────────────────────

pub fn htaccess() -> &'static str {
    r#"<IfModule mod_rewrite.c>
  RewriteEngine On
  RewriteBase /
  RewriteRule ^index\.html$ - [L]
  RewriteCond %{REQUEST_FILENAME} !-f
  RewriteCond %{REQUEST_FILENAME} !-d
  RewriteRule . /index.html [L]
</IfModule>
"#
}

// ─── index.html ───────────────────────────────────────────────────────────────

pub fn index_html(cfg: &ProjectConfig) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="es">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>{site_name}</title>
    <link rel="icon" href="/oweelogo.png" />
  </head>
  <body>
    <div id="app"></div>
    <script type="module" src="/src/main.ts"></script>
  </body>
</html>
"#,
        site_name = cfg.site_name
    )
}

// ─── src/main.ts ──────────────────────────────────────────────────────────────

pub fn main_ts(cfg: &ProjectConfig) -> String {
    // Icon set imports
    let icon_imports: String = cfg.icon_sets.iter()
        .map(|i| i.import().to_string())
        .collect::<Vec<_>>()
        .join("\n");

    // Plugin names for import and object
    let plugin_names: Vec<&str> = cfg.qplugins.iter().map(|p| p.name()).collect();
    let plugins_import = if plugin_names.is_empty() {
        "Quasar".to_string()
    } else {
        format!("Quasar, {}", plugin_names.join(", "))
    };
    let plugins_obj = if plugin_names.is_empty() {
        "{}".to_string()
    } else {
        format!("{{ {} }}", plugin_names.join(", "))
    };

    format!(r#"import {{ createApp }} from 'vue'
import {{ {plugins_import} }} from 'quasar'
import {{ createPinia }} from 'pinia'
import router from './router'
import App from './App.vue'

{icon_imports}
import 'quasar/dist/quasar.css'
import './css/main.css'

const app = createApp(App)

app.use(Quasar, {{
  plugins: {plugins_obj},
  config: {{
    dark: true,
    brand: {{
      primary:   '#e8553a',
      secondary: '#1a5c47',
      accent:    '#f5e2a0',
      dark:      '#0d3d2e',
      positive:  '#43b89c',
      negative:  '#c93f26',
      info:      '#a8d5c2',
      warning:   '#f5c842',
    }},
  }},
}})

app.use(createPinia())
app.use(router)
app.mount('#app')
"#)
}

// ─── src/App.vue ──────────────────────────────────────────────────────────────

pub fn app_vue() -> &'static str {
    r#"<template>
  <router-view />
</template>
"#
}

// ─── src/css/main.css ─────────────────────────────────────────────────────────

pub fn main_css() -> &'static str {
    r#"/* Oweeme Framework — Design System (paleta Koi) */
:root {
  --oweeme-bg:      #0a1a14;
  --oweeme-surface: #0f2318;
  --oweeme-teal:    #0f2318;
  --oweeme-coral:   #e8553a;
  --oweeme-cream:   #f0ede6;
  --oweeme-mint:    #a8d5c2;
  --oweeme-muted:   #6b8f7e;
  --oweeme-radius:  14px;
  --oweeme-shadow:  0 4px 32px rgba(0,0,0,0.5);
}

html, body { margin: 0; padding: 0; }
body { background: var(--oweeme-bg); font-family: 'Roboto', system-ui, sans-serif; color: var(--oweeme-cream); }

/* Layout */
.oweeme-page    { background: var(--oweeme-bg); min-height: 100vh; }
.oweeme-bg      { background: var(--oweeme-surface) !important; }

/* Header — oscuro casi negro con borde inferior sutil */
.oweeme-header  {
  background: rgba(10, 26, 20, 0.95) !important;
  backdrop-filter: blur(12px);
  border-bottom: 1px solid rgba(255,255,255,0.06);
  box-shadow: none !important;
}

/* Nav links */
.oweeme-nav-link {
  text-decoration: none;
  color: rgba(240,237,230,0.75);
  font-size: .92rem;
  font-weight: 500;
  padding: 6px 14px;
  border-radius: 6px;
  transition: color .15s, background .15s;
}
.oweeme-nav-link:hover { color: var(--oweeme-cream); background: rgba(255,255,255,.05); }
.oweeme-nav-link--active { color: var(--oweeme-coral) !important; }

/* Footer */
.oweeme-footer  { background: var(--oweeme-surface) !important; border-top: 1px solid rgba(255,255,255,0.06); }

/* Cards */
.oweeme-card {
  background:    var(--oweeme-surface) !important;
  border:        1px solid rgba(232,85,58,0.12);
  border-radius: var(--oweeme-radius) !important;
  box-shadow:    var(--oweeme-shadow);
  transition:    transform 0.2s ease, box-shadow 0.2s ease;
}
.oweeme-card--hover:hover {
  transform:  translateY(-4px);
  box-shadow: 0 8px 40px rgba(232,85,58,0.2);
}

/* Tipografía */
.text-cream     { color: var(--oweeme-cream) !important; }
.text-muted     { color: var(--oweeme-muted) !important; }
.text-mint      { color: var(--oweeme-mint)  !important; }
.no-decoration  { text-decoration: none; color: inherit; }

/* Prose (contenido blog/artículo) */
.prose { line-height: 1.8; color: var(--oweeme-mint); }
.prose h2 { color: var(--oweeme-cream); margin-top: 2rem; }
.prose a  { color: var(--oweeme-coral); }
.prose code {
  background:    var(--oweeme-teal);
  padding:       2px 6px;
  border-radius: 4px;
  font-size:     0.9em;
}

/* Scrollbar */
::-webkit-scrollbar       { width: 6px; }
::-webkit-scrollbar-track { background: var(--oweeme-bg); }
::-webkit-scrollbar-thumb { background: var(--oweeme-teal); border-radius: 3px; }
"#
}

// ─── src/css/quasar.variables.scss ────────────────────────────────────────────

pub fn quasar_variables() -> &'static str {
    r#"$primary:   #e8553a;
$secondary: #1a5c47;
$accent:    #f5e2a0;
$dark:      #0d3d2e;
$positive:  #43b89c;
$negative:  #c93f26;
$info:      #a8d5c2;
$warning:   #f5c842;
"#
}

// ─── src/router/index.ts ──────────────────────────────────────────────────────

pub fn router_index(cfg: &ProjectConfig) -> String {
    if cfg.is_ts() {
        r#"import { createRouter, createWebHistory } from 'vue-router'
import routes from './routes'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
  scrollBehavior: () => ({ top: 0 }),
})

export default router
"#.to_string()
    } else {
        r#"import { createRouter, createWebHistory } from 'vue-router'
import routes from './routes'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
  scrollBehavior: () => ({ top: 0 }),
})

export default router
"#.to_string()
    }
}

// ─── src/router/routes.ts / routes.js ────────────────────────────────────────

pub fn router_routes(cfg: &ProjectConfig) -> String {
    let type_import = if cfg.is_ts() { "import type { RouteRecordRaw } from 'vue-router'\n" } else { "" };
    let type_ann    = if cfg.is_ts() { ": RouteRecordRaw[]" } else { "" };
    format!(r#"{type_import}import MainLayout from '@/layouts/MainLayout.vue'

const routes{type_ann} = [
  {{
    path: '/',
    component: MainLayout,
    children: [
      {{ path: '', name: 'index', component: () => import('@/pages/Index.vue') }},
    ],
  }},
  {{
    path: '/:catchAll(.*)*',
    component: () => import('@/pages/ErrorNotFound.vue'),
  }},
]

export default routes
"#)
}

// ─── src/layouts/MainLayout.vue ───────────────────────────────────────────────

pub fn layout_main(_cfg: &ProjectConfig) -> String {
    format!(
        r#"<template>
  <q-layout view="hHh lpR fFf">
    <AppHeader />
    <q-page-container>
      <router-view />
    </q-page-container>
    <AppFooter />
  </q-layout>
</template>

<script setup lang="ts">
import AppHeader from '@/components/AppHeader.vue'
import AppFooter from '@/components/AppFooter.vue'
</script>
"#,
    )
}

// ─── src/pages/Index.vue ──────────────────────────────────────────────────────

pub fn page_index(cfg: &ProjectConfig) -> String {
    format!(
        r#"<script setup lang="ts">
import {{ useMeta }} from 'quasar'
import HeroSection from '@/components/HeroSection.vue'

useMeta({{
  title: '{name}',
  meta: {{
    description:  {{ name: 'description',       content: 'Bienvenido a {name}' }},
    ogTitle:      {{ property: 'og:title',       content: '{name}' }},
    ogDescription:{{ property: 'og:description', content: 'Bienvenido a {name}' }},
    ogImage:      {{ property: 'og:image',       content: '/oweelogo.png' }},
    ogUrl:        {{ property: 'og:url',         content: '{url}' }},
    twitterCard:  {{ name: 'twitter:card',       content: 'summary_large_image' }},
  }},
  link: {{
    canonical: {{ rel: 'canonical', href: '{url}' }},
  }},
}})
</script>

<template>
  <q-page class="oweeme-page">
    <HeroSection />
    <section class="q-pa-xl">
      <div class="row q-col-gutter-lg justify-center">
        <div class="col-12 text-center">
          <h2 class="text-h5 text-cream">¿Listo para comenzar?</h2>
          <p class="text-muted">
            Edita <code>src/pages/Index.vue</code> para personalizar esta página.
          </p>
          <q-btn
            color="primary" unelevated size="lg" icon="rocket_launch"
            label="Comenzar" class="q-mt-md"
          />
        </div>
      </div>
    </section>
  </q-page>
</template>
"#,
        name = cfg.site_name,
        url  = cfg.site_url,
    )
}

// ─── src/pages/ErrorNotFound.vue ──────────────────────────────────────────────

pub fn page_error() -> &'static str {
    r#"<script setup lang="ts">
import { useMeta } from 'quasar'
import { useRouter } from 'vue-router'

useMeta({ title: '404 — Página no encontrada', meta: { robots: { name: 'robots', content: 'noindex' } } })
const router = useRouter()
</script>

<template>
  <q-layout view="hHh lpR fFf">
    <q-page-container>
      <q-page class="flex flex-center column q-gutter-lg oweeme-page">
        <h1 class="text-primary" style="font-size:6rem;margin:0;font-weight:800;">404</h1>
        <p class="text-h6 text-cream">Página no encontrada</p>
        <q-btn color="primary" unelevated label="Volver al inicio" icon="home" @click="router.push('/')" />
      </q-page>
    </q-page-container>
  </q-layout>
</template>
"#
}

// ─── src/components/AppHeader.vue ────────────────────────────────────────────

pub fn comp_header(cfg: &ProjectConfig) -> String {
    format!(
        r#"<script setup lang="ts">
import {{ ref }} from 'vue'
import {{ useRouter }} from 'vue-router'

const router = useRouter()
const drawer = ref(false)

const links = [
  {{ label: 'Inicio',        to: '/' }},
  {{ label: 'Nosotros',      to: '/nosotros' }},
  {{ label: 'Servicios',     to: '/servicios' }},
  {{ label: 'Productos',     to: '/productos' }},
  {{ label: 'Contacto',      to: '/contacto' }},
]
</script>

<template>
  <q-header class="oweeme-header">
    <q-toolbar style="min-height:64px; padding: 0 2rem;">
      <router-link to="/" class="flex items-center q-gutter-sm no-decoration">
        <img src="/oweelogo.png" alt="{name}" style="height:34px;border-radius:50%;" />
        <span class="text-weight-bold text-cream" style="font-size:1rem;letter-spacing:.02em;">{name}</span>
      </router-link>

      <q-space />

      <!-- Nav desktop -->
      <nav class="gt-sm flex items-center q-gutter-xs">
        <router-link
          v-for="l in links" :key="l.to"
          :to="l.to"
          class="oweeme-nav-link"
          active-class="oweeme-nav-link--active"
        >
          {{{{ l.label }}}}
        </router-link>
      </nav>

      <q-space />

      <!-- Acciones -->
      <div class="gt-sm flex items-center q-gutter-sm">
        <q-btn flat round icon="light_mode" class="text-cream" size="sm" />
        <q-btn
          unelevated color="primary" label="Ingresar"
          style="border-radius:8px; font-weight:600;"
          @click="router.push('/login')"
        />
      </div>

      <!-- Mobile -->
      <q-btn class="lt-md" flat round icon="menu" color="cream" @click="drawer = !drawer" />
    </q-toolbar>
  </q-header>

  <q-drawer v-model="drawer" side="right" overlay class="oweeme-bg" style="max-width:260px;">
    <div class="q-pa-md flex items-center q-gutter-sm" style="border-bottom:1px solid rgba(255,255,255,.08)">
      <img src="/oweelogo.png" style="height:30px;border-radius:50%;" />
      <span class="text-cream text-weight-bold">{name}</span>
    </div>
    <q-list class="q-pt-sm">
      <q-item
        v-for="l in links" :key="l.to"
        clickable v-ripple
        @click="router.push(l.to); drawer = false"
      >
        <q-item-section class="text-cream">{{{{ l.label }}}}</q-item-section>
      </q-item>
      <q-separator dark class="q-my-sm" />
      <q-item clickable v-ripple @click="router.push('/login'); drawer = false">
        <q-item-section class="text-primary text-weight-bold">Ingresar</q-item-section>
      </q-item>
    </q-list>
  </q-drawer>
</template>
"#,
        name = cfg.site_name,
    )
}

// ─── src/components/AppFooter.vue ────────────────────────────────────────────

pub fn comp_footer(cfg: &ProjectConfig) -> String {
    format!(
        r#"<script setup lang="ts">
const year = new Date().getFullYear()
</script>

<template>
  <q-footer class="oweeme-footer q-pa-lg text-center">
    <p class="text-muted q-ma-none text-caption">
      © {{{{ year }}}} {name} —
      <a
        href="https://github.com/oweeme/framework-oweeme"
        target="_blank" rel="noopener"
        class="text-primary"
      >Oweeme Framework</a>
    </p>
  </q-footer>
</template>
"#,
        name = cfg.site_name,
    )
}

// ─── src/components/HeroSection.vue ──────────────────────────────────────────

pub fn comp_hero(cfg: &ProjectConfig) -> String {
    format!(
        r#"<template>
  <section class="oweeme-hero">
    <div class="oweeme-hero__inner">
      <!-- Texto izquierda -->
      <div class="oweeme-hero__text">
        <div class="oweeme-hero__badge">
          <span class="oweeme-hero__dot"></span>
          {name_upper}
        </div>

        <h1 class="oweeme-hero__title">
          Tu próxima gran<br/>aplicación empieza aquí.
        </h1>

        <p class="oweeme-hero__subtitle">
          Construido con Quasar SPA + Vue 3 + Vite.<br/>
          Sin servidor Node. Deploy directo a cualquier hosting estático.
        </p>

        <div class="oweeme-hero__regions">
          <span>Frontend</span><span>SEO</span><span>PWA</span><span>TypeScript</span>
        </div>

        <div class="oweeme-hero__actions">
          <q-btn
            unelevated color="secondary" label="Ver servicios"
            size="md" style="border-radius:8px; font-weight:600; padding: 10px 24px;"
          />
          <button class="oweeme-hero__link-btn">
            Conocer más &rarr;
          </button>
        </div>
      </div>

      <!-- Imagen derecha -->
      <div class="oweeme-hero__image">
        <div class="oweeme-hero__logo-wrap">
          <img src="/oweelogo.png" alt="{name}" class="oweeme-hero__logo" />
        </div>
      </div>
    </div>
  </section>
</template>

<style scoped>
.oweeme-hero {{
  min-height: calc(100vh - 64px);
  display: flex;
  align-items: center;
  padding: 4rem 2rem;
  background: var(--oweeme-bg);
}}

.oweeme-hero__inner {{
  max-width: 1200px;
  margin: 0 auto;
  width: 100%;
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 4rem;
  align-items: center;
}}

.oweeme-hero__badge {{
  display: inline-flex;
  align-items: center;
  gap: 8px;
  font-size: .8rem;
  font-weight: 600;
  letter-spacing: .12em;
  color: var(--oweeme-coral);
  text-transform: uppercase;
  margin-bottom: 1.5rem;
}}

.oweeme-hero__dot {{
  width: 8px; height: 8px;
  border-radius: 50%;
  background: var(--oweeme-coral);
  display: inline-block;
}}

.oweeme-hero__title {{
  font-size: clamp(2.2rem, 5vw, 3.6rem);
  font-weight: 800;
  color: var(--oweeme-cream);
  line-height: 1.1;
  margin: 0 0 1.25rem 0;
  font-family: Georgia, 'Times New Roman', serif;
}}

.oweeme-hero__subtitle {{
  font-size: 1.05rem;
  color: var(--oweeme-muted);
  line-height: 1.7;
  margin: 0 0 1.5rem 0;
  max-width: 480px;
}}

.oweeme-hero__regions {{
  display: flex;
  gap: 1rem;
  margin-bottom: 2rem;
}}

.oweeme-hero__regions span {{
  font-size: .8rem;
  color: var(--oweeme-mint);
  opacity: .65;
  letter-spacing: .04em;
}}

.oweeme-hero__actions {{
  display: flex;
  align-items: center;
  gap: 1.5rem;
}}

.oweeme-hero__link-btn {{
  background: none;
  border: none;
  color: var(--oweeme-cream);
  font-size: 1rem;
  cursor: pointer;
  padding: 0;
  opacity: .75;
  transition: opacity .2s;
}}

.oweeme-hero__link-btn:hover {{ opacity: 1; }}

.oweeme-hero__image {{
  display: flex;
  justify-content: center;
  align-items: center;
}}

.oweeme-hero__logo-wrap {{
  width: clamp(220px, 35vw, 380px);
  height: clamp(220px, 35vw, 380px);
  border-radius: 50%;
  background: radial-gradient(circle, rgba(26,92,71,.6) 0%, rgba(13,61,46,.3) 70%, transparent 100%);
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: 0 0 80px rgba(26,92,71,.4);
}}

.oweeme-hero__logo {{
  width: 80%;
  height: 80%;
  object-fit: contain;
  border-radius: 50%;
}}

@media (max-width: 768px) {{
  .oweeme-hero__inner {{
    grid-template-columns: 1fr;
    text-align: center;
  }}
  .oweeme-hero__image {{ display: none; }}
  .oweeme-hero__badge, .oweeme-hero__regions, .oweeme-hero__actions {{ justify-content: center; }}
  .oweeme-hero__subtitle {{ max-width: 100%; }}
}}
</style>
"#,
        name       = cfg.site_name,
        name_upper = cfg.site_name.to_uppercase(),
    )
}

// ─── src/composables/useApi.ts ────────────────────────────────────────────────

pub fn composable_use_api(cfg: &ProjectConfig) -> String {
    format!(
        r#"import {{ ref, type Ref }} from 'vue'

const API_BASE = import.meta.env.VITE_API_BASE || '{api}'

interface ApiResult<T> {{
  data:    Ref<T | null>
  loading: Ref<boolean>
  error:   Ref<string | null>
  refresh: () => Promise<void>
}}

export function useApi<T = unknown>(path: string | (() => string)): ApiResult<T> {{
  const data    = ref<T | null>(null) as Ref<T | null>
  const loading = ref(false)
  const error   = ref<string | null>(null)

  async function refresh() {{
    const url = typeof path === 'function' ? path() : path
    loading.value = true
    error.value   = null
    try {{
      const res = await fetch(API_BASE + url)
      if (!res.ok) throw new Error(`${{res.status}} ${{res.statusText}}`)
      data.value = await res.json()
    }} catch (e: any) {{
      error.value = e.message
      console.error('[useApi]', url, e.message)
    }} finally {{
      loading.value = false
    }}
  }}

  refresh()
  return {{ data, loading, error, refresh }}
}}

export async function apiPost<T = unknown>(path: string, body: unknown): Promise<T> {{
  const res = await fetch(API_BASE + path, {{
    method:  'POST',
    headers: {{ 'Content-Type': 'application/json' }},
    body:    JSON.stringify(body),
  }})
  if (!res.ok) throw new Error(`${{res.status}} ${{res.statusText}}`)
  return res.json()
}}

export async function apiPut<T = unknown>(path: string, body: unknown): Promise<T> {{
  const res = await fetch(API_BASE + path, {{
    method:  'PUT',
    headers: {{ 'Content-Type': 'application/json' }},
    body:    JSON.stringify(body),
  }})
  if (!res.ok) throw new Error(`${{res.status}} ${{res.statusText}}`)
  return res.json()
}}

export async function apiDelete(path: string): Promise<void> {{
  const res = await fetch(API_BASE + path, {{ method: 'DELETE' }})
  if (!res.ok) throw new Error(`${{res.status}} ${{res.statusText}}`)
}}
"#,
        api = "/api",
    )
}

// ─── src/composables/useSeo.ts ────────────────────────────────────────────────

pub fn composable_use_seo(cfg: &ProjectConfig) -> String {
    let _ = cfg;
    r#"import { useMeta } from 'quasar'

const SITE_URL  = import.meta.env.VITE_SITE_URL  || ''
const SITE_NAME = import.meta.env.VITE_SITE_NAME || ''

interface SeoOptions {
  title:        string
  description?: string
  image?:       string
  url?:         string
  type?:        'website' | 'article' | 'product'
  schema?:      Record<string, unknown>
  noindex?:     boolean
}

export function useSeo(opts: SeoOptions) {
  const fullTitle = opts.title.includes(SITE_NAME)
    ? opts.title
    : `${opts.title} | ${SITE_NAME}`

  const canonical = opts.url ? SITE_URL + opts.url : SITE_URL
  const image     = opts.image || '/oweelogo.png'
  const desc      = opts.description || ''

  const meta: Record<string, any> = {
    description:   { name: 'description',       content: desc },
    ogTitle:       { property: 'og:title',       content: fullTitle },
    ogDescription: { property: 'og:description', content: desc },
    ogImage:       { property: 'og:image',       content: image },
    ogUrl:         { property: 'og:url',         content: canonical },
    ogType:        { property: 'og:type',        content: opts.type || 'website' },
    twitterCard:   { name: 'twitter:card',       content: 'summary_large_image' },
    twitterTitle:  { name: 'twitter:title',      content: fullTitle },
    twitterImage:  { name: 'twitter:image',      content: image },
  }

  if (opts.noindex) {
    meta.robots = { name: 'robots', content: 'noindex, nofollow' }
  }

  const extraScript: Record<string, Record<string, string>> = {}
  if (opts.schema) {
    extraScript.ldJson = {
      type:      'application/ld+json',
      innerHTML: JSON.stringify({ '@context': 'https://schema.org', ...opts.schema }),
    }
  }

  useMeta({
    title: fullTitle,
    meta,
    link:   { canonical: { rel: 'canonical', href: canonical } },
    ...(opts.schema ? { script: extraScript } : {}),
  })
}

// Helpers para schema.org
export function productSchema(p: {
  name: string; description?: string; image?: string; price?: number; currency?: string
}) {
  return {
    '@type': 'Product',
    name:    p.name,
    description: p.description,
    image:   p.image,
    offers:  {
      '@type':        'Offer',
      price:          p.price,
      priceCurrency:  p.currency || 'USD',
      availability:   'https://schema.org/InStock',
    },
  }
}

export function articleSchema(a: {
  title: string; description?: string; image?: string; date?: string; author?: string
}) {
  return {
    '@type':       'Article',
    headline:      a.title,
    description:   a.description,
    image:         a.image,
    datePublished: a.date,
    author:        { '@type': 'Person', name: a.author },
  }
}

export function orgSchema(o: { name: string; url: string; logo?: string }) {
  return {
    '@type': 'Organization',
    name:    o.name,
    url:     o.url,
    logo:    o.logo,
  }
}
"#.to_string()
}

// ─── src/stores/app.ts / app.js ───────────────────────────────────────────────

pub fn store_app(cfg: &ProjectConfig) -> String {
    let set_loading = if cfg.is_ts() {
        "function setLoading(v: boolean) { loading.value = v }"
    } else {
        "function setLoading(v) { loading.value = v }"
    };
    format!(
        r#"import {{ defineStore }} from 'pinia'
import {{ ref }} from 'vue'

export const useAppStore = defineStore('app', () => {{
  const siteName = ref(import.meta.env.VITE_SITE_NAME || '{site}')
  const siteUrl  = ref(import.meta.env.VITE_SITE_URL  || '{url}')
  const loading  = ref(false)

  {set_loading}

  return {{ siteName, siteUrl, loading, setLoading }}
}})
"#,
        site = cfg.site_name,
        url  = cfg.site_url,
    )
}

// ─── public/robots.txt ────────────────────────────────────────────────────────

pub fn robots_txt(cfg: &ProjectConfig) -> String {
    format!(
        "User-agent: *\nAllow: /\nSitemap: {}/sitemap.xml\n",
        cfg.site_url
    )
}

// ─── public/sitemap.xml (placeholder) ────────────────────────────────────────

pub fn sitemap_placeholder(cfg: &ProjectConfig) -> String {
    let today = "2025-01-01"; // placeholder — oweeme sitemap lo regenera
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <url>
    <loc>{url}/</loc>
    <lastmod>{today}</lastmod>
    <changefreq>weekly</changefreq>
    <priority>1.0</priority>
  </url>
</urlset>
"#,
        url   = cfg.site_url,
        today = today,
    )
}

// ─── public/manifest.json ────────────────────────────────────────────────────

pub fn pwa_manifest(cfg: &ProjectConfig) -> String {
    let n = &cfg.site_name;
    format!(
        r##"{{
  "name": "{n}",
  "short_name": "{n}",
  "start_url": "/",
  "display": "standalone",
  "background_color": "#0d3d2e",
  "theme_color": "#e8553a",
  "icons": [
    {{ "src": "/oweelogo.png", "sizes": "512x512", "type": "image/png" }}
  ]
}}
"##
    )
}

// ─── Helpers para generadores (add/page/component) ───────────────────────────

pub fn page_template(name: &str, route: &str, with_auth: bool) -> String {
    let auth_guard = if with_auth {
        r#"
import { useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
const auth = useAuthStore()
const router = useRouter()
if (!auth.isAuthenticated) router.push('/login')
"#
    } else {
        ""
    };

    format!(
        r#"<script setup lang="ts">
import {{ useSeo }} from '@/composables/useSeo'
{auth_guard}
useSeo({{
  title: '{name}',
  description: 'Descripción de {name}',
  url: '{route}',
}})
</script>

<template>
  <q-page class="oweeme-page q-pa-xl">
    <h1 class="text-h4 text-cream q-mb-lg">{name}</h1>
    <p class="text-muted">Contenido de la página {name}.</p>
  </q-page>
</template>
"#,
        name  = name,
        route = route,
        auth_guard = auth_guard,
    )
}

pub fn component_template(name: &str, props: &[(&str, &str)]) -> String {
    let props_def = if props.is_empty() {
        String::new()
    } else {
        let lines = props
            .iter()
            .map(|(k, t)| format!("  {k}: {t}"))
            .collect::<Vec<_>>()
            .join("\n");
        format!("\ndefineProps<{{\n{lines}\n}}>()\n")
    };

    format!(
        r#"<script setup lang="ts">
{props_def}
</script>

<template>
  <div class="oweeme-card q-pa-md">
    <!-- {name} -->
    <slot />
  </div>
</template>
"#,
        name      = name,
        props_def = props_def,
    )
}

// ─── Módulo: auth ─────────────────────────────────────────────────────────────

pub fn module_auth_store() -> &'static str {
    r#"import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { apiPost } from '@/composables/useApi'

interface User { id: number; nombre: string; email: string; rol: string }

export const useAuthStore = defineStore('auth', () => {
  const user  = ref<User | null>(JSON.parse(localStorage.getItem('user') || 'null'))
  const token = ref<string | null>(localStorage.getItem('token'))

  const isAuthenticated = computed(() => !!token.value)
  const isAdmin         = computed(() => user.value?.rol === 'admin')

  async function login(email: string, password: string) {
    const res = await apiPost<{ user: User; token: string }>('/auth/login', { email, password })
    user.value  = res.user
    token.value = res.token
    localStorage.setItem('user',  JSON.stringify(res.user))
    localStorage.setItem('token', res.token)
  }

  function logout() {
    user.value  = null
    token.value = null
    localStorage.removeItem('user')
    localStorage.removeItem('token')
  }

  async function register(data: { nombre: string; email: string; password: string }) {
    await apiPost('/auth/register', data)
  }

  return { user, token, isAuthenticated, isAdmin, login, logout, register }
})
"#
}

pub fn module_auth_login() -> &'static str {
    r#"<script setup lang="ts">
import { reactive, ref } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import { useSeo } from '@/composables/useSeo'

useSeo({ title: 'Iniciar sesión', noindex: true })

const router = useRouter()
const auth   = useAuthStore()
const form   = reactive({ email: '', password: '' })
const error  = ref('')
const loading= ref(false)

async function submit() {
  loading.value = true
  error.value   = ''
  try {
    await auth.login(form.email, form.password)
    router.push('/dashboard')
  } catch (e: any) {
    error.value = e.message || 'Credenciales incorrectas'
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <q-page class="oweeme-page flex flex-center">
    <q-card class="oweeme-card q-pa-lg" style="width:100%;max-width:420px;">
      <q-card-section class="text-center q-mb-md">
        <img src="/oweelogo.png" style="height:56px;border-radius:50%;" />
        <h1 class="text-h6 text-cream q-mt-md q-mb-none">Iniciar sesión</h1>
      </q-card-section>
      <q-card-section>
        <q-form @submit.prevent="submit" class="q-gutter-md">
          <q-input v-model="form.email"    label="Email"      type="email"    outlined dark filled />
          <q-input v-model="form.password" label="Contraseña" type="password" outlined dark filled />
          <q-banner v-if="error" class="bg-negative text-white" dense rounded>{{ error }}</q-banner>
          <q-btn type="submit" color="primary" label="Entrar" unelevated class="full-width" :loading="loading" />
          <p class="text-center text-muted text-caption q-mt-sm">
            ¿No tienes cuenta?
            <router-link to="/registro" class="text-primary">Regístrate</router-link>
          </p>
        </q-form>
      </q-card-section>
    </q-card>
  </q-page>
</template>
"#
}

pub fn module_auth_register() -> &'static str {
    r#"<script setup lang="ts">
import { reactive, ref } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import { useSeo } from '@/composables/useSeo'

useSeo({ title: 'Crear cuenta', noindex: true })

const router  = useRouter()
const auth    = useAuthStore()
const form    = reactive({ nombre: '', email: '', password: '' })
const error   = ref('')
const loading = ref(false)

async function submit() {
  loading.value = true
  error.value   = ''
  try {
    await auth.register(form)
    await auth.login(form.email, form.password)
    router.push('/dashboard')
  } catch (e: any) {
    error.value = e.message || 'Error al registrarse'
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <q-page class="oweeme-page flex flex-center">
    <q-card class="oweeme-card q-pa-lg" style="width:100%;max-width:420px;">
      <q-card-section class="text-center q-mb-md">
        <img src="/oweelogo.png" style="height:56px;border-radius:50%;" />
        <h1 class="text-h6 text-cream q-mt-md q-mb-none">Crear cuenta</h1>
      </q-card-section>
      <q-card-section>
        <q-form @submit.prevent="submit" class="q-gutter-md">
          <q-input v-model="form.nombre"   label="Nombre"      outlined dark filled />
          <q-input v-model="form.email"    label="Email"       type="email"    outlined dark filled />
          <q-input v-model="form.password" label="Contraseña"  type="password" outlined dark filled />
          <q-banner v-if="error" class="bg-negative text-white" dense rounded>{{ error }}</q-banner>
          <q-btn type="submit" color="primary" label="Registrarse" unelevated class="full-width" :loading="loading" />
          <p class="text-center text-muted text-caption q-mt-sm">
            ¿Ya tienes cuenta?
            <router-link to="/login" class="text-primary">Inicia sesión</router-link>
          </p>
        </q-form>
      </q-card-section>
    </q-card>
  </q-page>
</template>
"#
}

pub fn module_auth_profile() -> &'static str {
    r#"<script setup lang="ts">
import { useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import { useSeo } from '@/composables/useSeo'

useSeo({ title: 'Mi perfil', noindex: true })

const router = useRouter()
const auth   = useAuthStore()
if (!auth.isAuthenticated) router.push('/login')

function logout() {
  auth.logout()
  router.push('/')
}
</script>

<template>
  <q-page class="oweeme-page q-pa-xl">
    <div style="max-width:600px;margin:0 auto;">
      <h1 class="text-h5 text-cream q-mb-lg">Mi Perfil</h1>
      <q-card class="oweeme-card q-pa-md q-mb-lg">
        <q-card-section class="q-gutter-sm">
          <p class="text-cream"><strong>Nombre:</strong> {{ auth.user?.nombre }}</p>
          <p class="text-cream"><strong>Email:</strong>  {{ auth.user?.email }}</p>
          <p class="text-cream"><strong>Rol:</strong>    {{ auth.user?.rol }}</p>
        </q-card-section>
      </q-card>
      <q-btn color="negative" outline label="Cerrar sesión" icon="logout" @click="logout" />
    </div>
  </q-page>
</template>
"#
}

pub fn module_auth_routes() -> &'static str {
    r#"// Agrega estas rutas en src/router/routes.ts dentro del array principal:
{
  path: '/login',    name: 'login',    component: () => import('@/pages/auth/Login.vue') },
{ path: '/registro', name: 'registro', component: () => import('@/pages/auth/Register.vue') },
{ path: '/perfil',   name: 'perfil',   component: () => import('@/pages/auth/Profile.vue'),
  meta: { requiresAuth: true } },
"#
}

// ─── Módulo: blog ─────────────────────────────────────────────────────────────

pub fn module_blog_index() -> &'static str {
    r#"<script setup lang="ts">
import { useApi } from '@/composables/useApi'
import { useSeo } from '@/composables/useSeo'
import BlogCard from '@/components/BlogCard.vue'

useSeo({ title: 'Blog', description: 'Artículos y noticias', url: '/blog' })

interface Post { slug: string; titulo: string; descripcion: string; imagen?: string; fecha?: string; autor?: string }
const { data: posts, loading } = useApi<Post[]>('/blog')
</script>

<template>
  <q-page class="oweeme-page q-pa-xl">
    <h1 class="text-h4 text-cream q-mb-lg">Blog</h1>
    <q-inner-loading :showing="loading" color="primary" />
    <div v-if="!loading" class="row q-col-gutter-lg">
      <div v-for="post in posts" :key="post.slug" class="col-12 col-md-6 col-lg-4">
        <BlogCard :post="post" />
      </div>
      <div v-if="!posts?.length" class="col-12 text-center q-pt-xl">
        <q-icon name="article" size="4rem" color="secondary" />
        <p class="text-muted q-mt-md">No hay artículos. Conecta tu API.</p>
      </div>
    </div>
  </q-page>
</template>
"#
}

pub fn module_blog_post() -> &'static str {
    r#"<script setup lang="ts">
import { computed } from 'vue'
import { useRoute } from 'vue-router'
import { useApi } from '@/composables/useApi'
import { useSeo, articleSchema } from '@/composables/useSeo'

const route = useRoute()
const slug  = computed(() => route.params.slug as string)

interface Post { slug: string; titulo: string; descripcion: string; contenido: string; imagen?: string; fecha?: string; autor?: string }
const { data: post, loading } = useApi<Post>(() => `/blog/${slug.value}`)

useSeo({
  title:       computed(() => post.value?.titulo || slug.value).value,
  description: computed(() => post.value?.descripcion).value,
  image:       computed(() => post.value?.imagen).value,
  url:         `/blog/${slug.value}`,
  type:        'article',
  schema:      computed(() => post.value ? articleSchema({
    title: post.value.titulo, description: post.value.descripcion,
    image: post.value.imagen, date: post.value.fecha, author: post.value.autor,
  }) : undefined).value,
})
</script>

<template>
  <q-page class="oweeme-page q-pa-xl">
    <q-inner-loading :showing="loading" color="primary" />
    <article v-if="!loading && post" style="max-width:780px;margin:0 auto;">
      <q-btn flat icon="arrow_back" to="/blog" color="primary" class="q-mb-md" label="Blog" />
      <q-img v-if="post.imagen" :src="post.imagen" :alt="post.titulo"
             class="q-mb-xl" style="border-radius:16px;max-height:420px;" />
      <h1 class="text-h4 text-cream">{{ post.titulo }}</h1>
      <div class="flex items-center q-gutter-sm q-mb-lg">
        <q-chip :label="post.autor" icon="person" color="secondary" text-color="white" />
        <q-chip :label="post.fecha" icon="event" flat />
      </div>
      <div class="prose" v-html="post.contenido" />
    </article>
  </q-page>
</template>
"#
}

pub fn module_blog_card() -> &'static str {
    r#"<script setup lang="ts">
defineProps<{
  post: { slug: string; titulo: string; descripcion?: string; imagen?: string; fecha?: string; autor?: string }
}>()
</script>

<template>
  <router-link :to="'/blog/' + post.slug" class="no-decoration">
    <q-card class="oweeme-card oweeme-card--hover cursor-pointer full-height">
      <q-img v-if="post.imagen" :src="post.imagen" :alt="post.titulo" height="180px" />
      <q-card-section class="q-gutter-sm">
        <h3 class="text-body1 text-cream q-ma-none">{{ post.titulo }}</h3>
        <p v-if="post.descripcion" class="text-muted text-caption q-ma-none">{{ post.descripcion }}</p>
        <div class="flex items-center q-gutter-xs text-caption text-muted q-mt-xs">
          <q-icon name="person" size="xs" /><span>{{ post.autor }}</span>
          <q-icon name="event"  size="xs" class="q-ml-sm" /><span>{{ post.fecha }}</span>
        </div>
      </q-card-section>
    </q-card>
  </router-link>
</template>
"#
}

// ─── Módulo: ecommerce ────────────────────────────────────────────────────────

pub fn module_ecommerce_cart_store() -> &'static str {
    r#"import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

interface CartItem { id: number | string; nombre: string; precio: number; imagen?: string; cantidad: number }

export const useCartStore = defineStore('cart', () => {
  const items = ref<CartItem[]>(JSON.parse(localStorage.getItem('cart') || '[]'))

  const total    = computed(() => items.value.reduce((s, i) => s + i.precio * i.cantidad, 0))
  const count    = computed(() => items.value.reduce((s, i) => s + i.cantidad, 0))
  const isEmpty  = computed(() => items.value.length === 0)

  function add(product: Omit<CartItem, 'cantidad'>, qty = 1) {
    const existing = items.value.find(i => i.id === product.id)
    if (existing) { existing.cantidad += qty }
    else { items.value.push({ ...product, cantidad: qty }) }
    save()
  }

  function remove(id: number | string) {
    items.value = items.value.filter(i => i.id !== id)
    save()
  }

  function updateQty(id: number | string, qty: number) {
    const item = items.value.find(i => i.id === id)
    if (item) { item.cantidad = Math.max(1, qty) }
    save()
  }

  function clear() { items.value = []; save() }

  function save() { localStorage.setItem('cart', JSON.stringify(items.value)) }

  return { items, total, count, isEmpty, add, remove, updateQty, clear }
})
"#
}

pub fn module_ecommerce_product_card() -> &'static str {
    r#"<script setup lang="ts">
import { useCartStore } from '@/stores/cart'
import { useQuasar } from 'quasar'

const props = defineProps<{
  product: { id: number | string; nombre: string; precio: number; imagen?: string; descripcion?: string; slug?: string }
  baseRoute?: string
}>()

const cart  = useCartStore()
const $q    = useQuasar()

function addToCart() {
  cart.add({ id: props.product.id, nombre: props.product.nombre, precio: props.product.precio, imagen: props.product.imagen })
  $q.notify({ color: 'positive', message: `${props.product.nombre} agregado al carrito`, icon: 'check' })
}
</script>

<template>
  <q-card class="oweeme-card oweeme-card--hover cursor-pointer full-height">
    <router-link :to="(baseRoute || '/productos') + '/' + (product.slug || product.id)" class="no-decoration">
      <q-img v-if="product.imagen" :src="product.imagen" :alt="product.nombre" height="180px" />
      <q-card-section>
        <h3 class="text-body1 text-cream q-ma-none">{{ product.nombre }}</h3>
        <p class="text-primary text-weight-bold q-mt-xs q-mb-none">${{ product.precio }}</p>
        <p v-if="product.descripcion" class="text-muted text-caption q-mt-xs q-mb-none">{{ product.descripcion }}</p>
      </q-card-section>
    </router-link>
    <q-card-actions>
      <q-btn flat color="primary" icon="add_shopping_cart" label="Agregar" @click="addToCart" class="full-width" />
    </q-card-actions>
  </q-card>
</template>
"#
}

// ─── Módulo: dashboard ────────────────────────────────────────────────────────

pub fn module_dashboard_layout() -> &'static str {
    r#"<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'

const router  = useRouter()
const auth    = useAuthStore()
const drawer  = ref(true)
const miniMode= ref(false)

const navItems = [
  { label: 'Dashboard', icon: 'dashboard',   to: '/dashboard' },
  { label: 'Perfil',    icon: 'person',      to: '/perfil' },
]

if (!auth.isAuthenticated) router.push('/login')
</script>

<template>
  <q-layout view="lHh Lpr lFf">
    <q-header elevated class="oweeme-header">
      <q-toolbar>
        <q-btn flat round icon="menu" @click="drawer = !drawer" />
        <q-toolbar-title class="text-cream text-weight-bold">Dashboard</q-toolbar-title>
        <q-btn flat round icon="logout" @click="auth.logout(); router.push('/')" />
      </q-toolbar>
    </q-header>

    <q-drawer v-model="drawer" :mini="miniMode" class="oweeme-bg" bordered>
      <q-list>
        <q-item-label header class="text-muted">Navegación</q-item-label>
        <q-item
          v-for="item in navItems" :key="item.to"
          :to="item.to" clickable v-ripple exact
        >
          <q-item-section avatar>
            <q-icon :name="item.icon" color="primary" />
          </q-item-section>
          <q-item-section class="text-cream">{{ item.label }}</q-item-section>
        </q-item>
      </q-list>
      <div class="absolute-bottom q-pa-sm">
        <q-btn flat round :icon="miniMode ? 'chevron_right' : 'chevron_left'"
               @click="miniMode = !miniMode" color="primary" />
      </div>
    </q-drawer>

    <q-page-container>
      <router-view />
    </q-page-container>
  </q-layout>
</template>
"#
}

pub fn module_dashboard_index() -> &'static str {
    r#"<script setup lang="ts">
import { useSeo } from '@/composables/useSeo'

useSeo({ title: 'Dashboard', noindex: true })

const stats = [
  { label: 'Usuarios',  value: '—', icon: 'people',        color: 'primary' },
  { label: 'Ventas',    value: '—', icon: 'shopping_cart',  color: 'positive' },
  { label: 'Ingresos',  value: '—', icon: 'attach_money',   color: 'accent' },
  { label: 'Pendientes',value: '—', icon: 'pending_actions',color: 'warning' },
]
</script>

<template>
  <q-page class="oweeme-page q-pa-xl">
    <h1 class="text-h5 text-cream q-mb-lg">Resumen</h1>
    <div class="row q-col-gutter-lg q-mb-xl">
      <div v-for="stat in stats" :key="stat.label" class="col-12 col-sm-6 col-md-3">
        <q-card class="oweeme-card q-pa-md">
          <q-card-section class="flex items-center q-gutter-md no-wrap">
            <q-icon :name="stat.icon" :color="stat.color" size="2.5rem" />
            <div>
              <div class="text-h5 text-cream text-weight-bold">{{ stat.value }}</div>
              <div class="text-muted text-caption">{{ stat.label }}</div>
            </div>
          </q-card-section>
        </q-card>
      </div>
    </div>
    <p class="text-muted text-caption">
      Conecta tu API en <code>.env</code> → <code>VITE_API_BASE</code> para ver datos reales.
    </p>
  </q-page>
</template>
"#
}

// ─── Módulo: rrhh ─────────────────────────────────────────────────────────────

pub fn module_rrhh_store() -> &'static str {
    r#"import { defineStore } from 'pinia'
import { ref } from 'vue'
import { useApi, apiPost, apiPut, apiDelete } from '@/composables/useApi'

export interface Empleado {
  id:         number
  nombre:     string
  email:      string
  cargo:      string
  departamento: string
  fecha_ingreso?: string
  activo:     boolean
}

export const useRrhhStore = defineStore('rrhh', () => {
  const { data: empleados, loading, refresh } = useApi<Empleado[]>('/rrhh/empleados')
  const selected = ref<Empleado | null>(null)

  async function crear(data: Omit<Empleado, 'id'>) {
    await apiPost('/rrhh/empleados', data)
    refresh()
  }

  async function actualizar(id: number, data: Partial<Empleado>) {
    await apiPut(`/rrhh/empleados/${id}`, data)
    refresh()
  }

  async function eliminar(id: number) {
    await apiDelete(`/rrhh/empleados/${id}`)
    refresh()
  }

  return { empleados, loading, selected, crear, actualizar, eliminar, refresh }
})
"#
}

pub fn module_rrhh_empleados() -> &'static str {
    r#"<script setup lang="ts">
import { ref } from 'vue'
import { useRrhhStore } from '@/stores/rrhh'
import { useSeo } from '@/composables/useSeo'

useSeo({ title: 'Empleados', noindex: true })

const rrhh   = useRrhhStore()
const search = ref('')
const cols = [
  { name: 'nombre',       label: 'Nombre',       field: 'nombre',       sortable: true },
  { name: 'cargo',        label: 'Cargo',        field: 'cargo',        sortable: true },
  { name: 'departamento', label: 'Departamento', field: 'departamento', sortable: true },
  { name: 'activo',       label: 'Estado',       field: 'activo' },
]
</script>

<template>
  <q-page class="oweeme-page q-pa-xl">
    <div class="flex items-center justify-between q-mb-lg">
      <h1 class="text-h5 text-cream q-ma-none">Empleados</h1>
      <q-btn color="primary" icon="add" label="Nuevo" unelevated />
    </div>
    <q-input v-model="search" placeholder="Buscar..." outlined dark dense class="q-mb-md">
      <template #prepend><q-icon name="search" color="primary" /></template>
    </q-input>
    <q-table
      :rows="rrhh.empleados || []"
      :columns="cols"
      :filter="search"
      :loading="rrhh.loading"
      flat dark
      class="oweeme-card"
      no-data-label="Sin empleados — conecta tu API"
    >
      <template #body-cell-activo="{ value }">
        <q-td>
          <q-chip :color="value ? 'positive' : 'grey'" text-color="white" dense :label="value ? 'Activo' : 'Inactivo'" />
        </q-td>
      </template>
    </q-table>
  </q-page>
</template>
"#
}

// ─── capacitor.config.ts ──────────────────────────────────────────────────────

pub fn capacitor_config() -> &'static str {
    r#"import type { CapacitorConfig } from '@capacitor/cli'

const config: CapacitorConfig = {
  appId:     'com.oweeme.app',
  appName:   'MiApp',
  webDir:    'dist',
  server: {
    androidScheme: 'https',
  },
  plugins: {
    SplashScreen: {
      launchShowDuration: 2000,
      backgroundColor: '#0a1a14',
      showSpinner: false,
    },
  },
}

export default config
"#
}

// ─── scripts/android.sh ───────────────────────────────────────────────────────

pub fn capacitor_android_script() -> &'static str {
    r#"#!/bin/bash
# Oweeme Framework — Build + sync Android
set -e

echo "→ Building Vite SPA..."
npm run build

echo "→ Syncing Capacitor..."
npx cap sync android

echo "→ Abriendo Android Studio..."
npx cap open android
"#
}
