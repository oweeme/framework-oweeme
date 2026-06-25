# Framework Oweeme

**CLI en Rust que genera proyectos Quasar SPA** con SEO profesional, módulos listos y herramientas que otros frameworks no tienen.

Sin servidor Node en producción. Output: `dist/` puro — desplegable en cualquier hosting estático.

---

## Instalación

```bash
cargo install --path cli
```

---

## Inicio rápido

```bash
oweeme new mi-tienda
cd mi-tienda
cp .env.example .env
npm install
npm run dev
```

---

## Comandos

### `oweeme new <nombre>`
Crea un proyecto Quasar SPA completo con:
- Vue 3 + TypeScript + Vite
- Quasar UI (componentes + paleta Koi)
- Vue Router con rutas limpias
- Pinia para estado global
- `useApi` composable tipado
- `useSeo` con schema.org automático
- `robots.txt` + `sitemap.xml`
- Logo embebido en el binario

### `oweeme add <módulo>`
Agrega módulos funcionales al proyecto:

```bash
oweeme add auth        # Login + Register + Profile + Pinia auth store
oweeme add blog        # Blog completo con schema.org Article
oweeme add ecommerce   # Productos + Carrito persistente + Checkout
oweeme add dashboard   # Panel admin con sidebar + stats cards
oweeme add rrhh        # Gestión de empleados con tabla y búsqueda
```

### `oweeme page <nombre>`
Genera una página con SEO configurado:

```bash
oweeme page Servicios
oweeme page ProductoDetalle --route="/productos/:id"
oweeme page AdminPanel --auth
```

### `oweeme component <nombre>`
Genera un componente Vue tipado:

```bash
oweeme component ProductCard --props="nombre:string,precio:number,imagen:string"
```

### `oweeme sitemap`
Genera `public/sitemap.xml` desde tus rutas:

```bash
oweeme sitemap --base https://mitienda.com
```

---

## SEO

Cada página usa `useSeo()` — una línea, todo configurado:

```ts
import { useSeo, productSchema } from '@/composables/useSeo'

useSeo({
  title:       'Nike Air Max',
  description: 'Las mejores zapatillas',
  url:         '/productos/nike-air-max',
  type:        'product',
  schema:      productSchema({ name: 'Nike Air Max', price: 150 }),
})
```

Genera automáticamente: `<title>`, `<meta>`, OpenGraph, Twitter Cards, JSON-LD schema.org, canonical URL.

---

## Stack del proyecto generado

| Tecnología | Versión |
|-----------|---------|
| Quasar | 2.17.4 |
| Vue 3 | 3.5.13 |
| Vue Router | 4.3.0 |
| Pinia | 2.2.6 |
| Vite | 5.4.11 |
| TypeScript | 5.7.3 |

---

## Estructura generada

```
mi-proyecto/
├── src/
│   ├── pages/          ← rutas automáticas
│   ├── layouts/        ← MainLayout con header/footer
│   ├── components/     ← AppHeader, AppFooter, HeroSection
│   ├── composables/    ← useApi, useSeo
│   ├── stores/         ← Pinia stores
│   ├── router/         ← Vue Router
│   ├── css/            ← Quasar + paleta Koi
│   ├── App.vue
│   └── main.ts
├── public/
│   ├── oweelogo.png
│   ├── robots.txt
│   └── sitemap.xml
├── vite.config.ts
├── package.json
└── .env.example
```

---

## Producción

```bash
npm run build          # genera dist/ listo para subir
oweeme sitemap         # actualiza sitemap.xml
```

Sube la carpeta `dist/` a cualquier hosting: Netlify, Vercel, GitHub Pages, nginx, Apache.

---

## Documentación

- [Comandos](docs/commands.md)
- [SEO](docs/seo.md)
- [Plan del proyecto](docs/plan.md)

---

## Autor

**Héctor Martínez** — [oweeme.com](https://oweeme.com)  
GitHub: [oweeme/framework-oweeme](https://github.com/oweeme/framework-oweeme)
