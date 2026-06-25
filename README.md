# Oweeme Framework

> CLI de Rust para generar proyectos **Quasar SPA + Vue 3 + Vite** listos para producción — con SEO, PWA, dark/light mode, y arquitectura modular.

**Autor:** Héctor Martínez — [oweeme.com](https://oweeme.com)  
**Licencia:** MIT

---

## ¿Qué es?

Oweeme Framework es una herramienta de línea de comandos (CLI) construida en Rust que genera proyectos **Quasar SPA** completamente configurados en segundos. No es un wrapper de Quasar CLI — es un generador de proyectos que produce código listo para editar y desplegar.

### Características

- **Quasar 2 + Vue 3 + Vite 5** — stack moderno de primer nivel
- **TypeScript o JavaScript** — tú eliges en el scaffolding
- **Dark / Light mode** — toggle funcional desde el header
- **PWA** — manifest + service worker (vite-plugin-pwa)
- **SEO** — useMeta, Open Graph, Twitter Cards, Schema.org, sitemap.xml
- **Axios** — instancia configurada con interceptores (Bearer token + 401 redirect)
- **Icon sets** — Material Icons, MDI v7, Font Awesome v6, Eva Icons, Line Awesome
- **Quasar plugins** — Notify, Dialog, Loading, BottomSheet, AppFullscreen, Dark, LocalStorage
- **ESLint 9 + oxlint + Prettier** — linting opcional, rápido y estricto
- **Apache .htaccess** — SPA routing listo para hosting compartido
- **Módulos** — auth, blog, ecommerce, dashboard, rrhh, capacitor (Android/iOS)
- **Generadores** — `oweeme page`, `oweeme component`, `oweeme sitemap`

---

## Instalación del CLI

### Requisitos

| Herramienta | Versión mínima |
|-------------|----------------|
| Rust        | 1.75+          |
| Node.js     | 18+            |
| npm         | 9+             |

Instala Rust desde [rustup.rs](https://rustup.rs) si no lo tienes:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### Desde el código fuente

```bash
# 1. Clona el repositorio
git clone https://github.com/oweeme/framework-oweeme.git
cd framework-oweeme

# 2. Compila e instala el CLI
cargo install --path cli

# 3. Verifica la instalación
oweeme info
```

El binario queda disponible en `~/.cargo/bin/oweeme` — ya está en tu `$PATH` si Rust está bien instalado.

### Actualizar a la última versión

```bash
cd framework-oweeme
git pull
cargo install --path cli --force
```

---

## Crear un proyecto

```bash
oweeme new mi-proyecto
```

El CLI hace preguntas interactivas:

```
  Configuremos tu proyecto Quasar SPA

  ? Nombre del sitio         › Mi Proyecto
  ? URL de producción        › https://mi-proyecto.com
  ? Lenguaje                 › TypeScript (recomendado)
  ? Icon sets                › [✓] material-icons  [ ] mdi-v7  ...
  ? Quasar plugins           › [✓] Notify  [✓] Dialog  [✓] Loading  ...
  ? ¿Habilitar PWA?          › Yes
  ? Linting & formato        › ESLint 9 + oxlint (recomendado)
```

### Pasos después del scaffolding

```bash
cd mi-proyecto
cp .env.example .env        # edita VITE_SITE_URL y VITE_SITE_NAME
npm install
npm run dev                 # http://localhost:5173
```

### Build para producción

```bash
npm run build               # genera dist/
npm run preview             # sirve dist/ localmente para verificar
oweeme sitemap              # genera public/sitemap.xml
```

---

## Estructura del proyecto generado

```
mi-proyecto/
├── public/
│   ├── oweelogo.png        ← logo (reemplázalo con el tuyo, 512×512 px)
│   ├── robots.txt
│   ├── sitemap.xml
│   ├── manifest.json       ← PWA (si lo habilitaste)
│   └── .htaccess           ← Apache SPA routing
├── src/
│   ├── boot/
│   │   └── axios.ts        ← instancia axios con interceptores
│   ├── components/
│   │   ├── AppHeader.vue   ← nav + dark/light toggle
│   │   ├── AppFooter.vue
│   │   └── HeroSection.vue ← hero dos columnas
│   ├── composables/
│   │   └── useSeo.ts       ← useMeta wrapper + helpers schema.org
│   ├── css/
│   │   ├── main.css        ← design system (paleta Koi)
│   │   └── quasar.variables.scss
│   ├── layouts/
│   │   └── MainLayout.vue
│   ├── pages/
│   │   ├── Index.vue
│   │   └── ErrorNotFound.vue
│   ├── router/
│   │   ├── index.ts
│   │   └── routes.ts
│   ├── stores/
│   │   └── app.ts          ← Pinia store global
│   ├── App.vue
│   └── main.ts
├── .env.example
├── .gitignore
├── index.html
├── oweeme.config.ts        ← config del proyecto (icon sets, plugins, etc.)
├── package.json
├── tsconfig.json           ← o jsconfig.json si elegiste JavaScript
└── vite.config.ts
```

---

## Comandos del CLI

### `oweeme new <nombre>`

Genera un nuevo proyecto Quasar SPA completo con scaffolding interactivo.

### `oweeme add <módulo>`

Agrega un módulo al proyecto existente (ejecutar desde la raíz del proyecto):

```bash
oweeme add auth        # Login + Register + Profile + Pinia auth store
oweeme add blog        # Páginas blog + BlogCard + composable
oweeme add ecommerce   # Productos + Carrito + Checkout + Pinia cart
oweeme add dashboard   # Layout admin + Sidebar + StatsCard
oweeme add rrhh        # Empleados + Detalle + Pinia rrhh store
oweeme add capacitor   # Android/iOS — capacitor.config.ts + scripts
```

### `oweeme page <nombre>`

Genera una página Vue con SEO y `useSeo` pre-configurado:

```bash
oweeme page Nosotros
oweeme page Servicios
```

La página se crea en `src/pages/<Nombre>.vue`. Agrega la ruta en `src/router/routes.ts`.

### `oweeme component <nombre>`

Genera un componente Vue tipado en `src/components/`:

```bash
oweeme component MiCard
oweeme component UserAvatar
```

### `oweeme sitemap`

Lee `src/router/routes.ts` y genera `public/sitemap.xml` con todas las rutas estáticas:

```bash
oweeme sitemap
# Lee VITE_SITE_URL desde .env automáticamente

oweeme sitemap --base https://tudominio.com
# O pasa la URL directamente
```

### `oweeme info`

Muestra ayuda y todos los comandos disponibles.

---

## Dark / Light Mode

El toggle está en el header (ícono luna/sol). Funciona con la API de dark mode de Quasar.

El modo inicial es `'auto'` — respeta la preferencia del sistema operativo del usuario.

### Personalizar colores

Edita `src/css/quasar.variables.scss` (paleta Quasar):

```scss
$primary:   #e8553a;  /* coral */
$secondary: #1a5c47;  /* verde oscuro */
$accent:    #f5e2a0;  /* crema dorado */
```

Edita `src/css/main.css` (design system Koi):

```css
:root {
  --oweeme-bg:    #0a1a14;
  --oweeme-coral: #e8553a;
}
body.body--light {
  --oweeme-bg: #f4f1ea;    /* Quasar añade body--light automáticamente */
}
```

---

## PWA

Si habilitaste PWA:

- `public/manifest.json` — nombre, íconos, colores del tema
- `vite-plugin-pwa` + Workbox — cache offline automático

Reemplaza `public/oweelogo.png` con tu propio logo (512×512 px) y actualiza `manifest.json`.

---

## Axios + API

`src/boot/axios.ts` exporta una instancia lista para usar:

```ts
import api from '@/boot/axios'

const { data } = await api.get('/usuarios')
const res = await api.post('/auth/login', { email, password })
```

**Interceptores incluidos:**
- Request: adjunta `Authorization: Bearer <token>` automáticamente
- Response: redirige a `/login` en respuesta 401

Configura la URL base en `.env`:

```env
VITE_API_BASE=https://api.tudominio.com
```

---

## Deploy

### Hosting estático (Netlify, Vercel, GitHub Pages)

```bash
npm run build
# sube dist/ a tu hosting
```

Para Netlify/Vercel, crea `public/_redirects`:
```
/* /index.html 200
```

Para Nginx:
```nginx
location / {
  try_files $uri $uri/ /index.html;
}
```

Para Apache (incluido automáticamente en `public/.htaccess`):
```apache
RewriteRule . /index.html [L]
```

### VPS / Apache

```bash
npm run build
rsync -avz dist/ usuario@servidor:/var/www/html/mi-proyecto/
```

---

## Paleta de colores (Koi)

| Variable           | Dark       | Light      | Uso                 |
|--------------------|------------|------------|---------------------|
| `--oweeme-bg`      | `#0a1a14`  | `#f4f1ea`  | Fondo de página     |
| `--oweeme-surface` | `#0f2318`  | `#ffffff`  | Cards y paneles     |
| `--oweeme-coral`   | `#e8553a`  | `#e8553a`  | Color primario      |
| `--oweeme-cream`   | `#f0ede6`  | `#1a2820`  | Texto principal     |
| `--oweeme-mint`    | `#a8d5c2`  | `#a8d5c2`  | Acentos             |
| `--oweeme-muted`   | `#6b8f7e`  | `#5a7a6b`  | Texto secundario    |

---

## Tecnologías

| Paquete         | Versión | Rol                    |
|-----------------|---------|------------------------|
| Quasar          | 2.17.4  | UI + layout            |
| Vue             | 3.5.13  | Framework reactivo     |
| Vite            | 5.4.11  | Bundler y dev server   |
| Vue Router      | 4.3.0   | Routing SPA            |
| Pinia           | 2.2.6   | State management       |
| Axios           | 1.7.9   | HTTP client            |
| vite-plugin-pwa | 0.20.5  | PWA / service worker   |
| TypeScript      | 5.7.3   | Tipado (opcional)      |
| ESLint          | 9.17.0  | Linting (opcional)     |
| oxlint          | 0.14.0  | Linting rápido (opcional) |

---

## ¿Dónde publicar el framework?

Opciones por orden de recomendación:

1. **crates.io** — `cargo publish` en el directorio `cli/`. Los usuarios instalan con:
   ```bash
   cargo install oweeme
   ```

2. **GitHub Releases** — sube binarios precompilados para Linux/macOS/Windows con `cargo build --release`. Los usuarios descargan y copian a su `$PATH`.

3. **npm** — empaqueta el binario como npm package (como Biome, oxlint). Los usuarios instalan con:
   ```bash
   npm install -g oweeme-cli
   ```

4. **Homebrew tap** — crea un tap propio para macOS/Linux.

---

## Contribuir

```bash
git clone https://github.com/oweeme/framework-oweeme.git
cd framework-oweeme/cli
cargo build
cargo install --path .    # instala localmente para probar
```

Los templates están en `cli/src/template.rs`.  
El flujo del scaffolding en `cli/src/scaffold.rs`.

---

## Roadmap

- [ ] `oweeme sync` — regenera `main.ts` desde `oweeme.config.ts`
- [ ] Publicación en crates.io (`cargo install oweeme`)
- [ ] Binarios precompilados en GitHub Releases
- [ ] Soporte React (Vite + React + TanStack Router)
- [ ] Plugin `i18n` — internacionalización automática

---

*Framework Oweeme v2.0 — [Héctor Martínez](https://oweeme.com)*
