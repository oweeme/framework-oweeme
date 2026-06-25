# SEO — Framework Oweeme

## Estrategia

El framework usa **SEO dinámico en SPA** — Google ejecuta JavaScript desde 2019 y puede indexar el contenido. La estrategia es:

1. `useMeta()` de Quasar → meta tags reactivos por ruta
2. `useSeo()` composable → helper unificado con schema.org
3. `sitemap.xml` → lista todas las URLs para Google
4. `robots.txt` → permite crawling completo

## useSeo() — el composable principal

```ts
import { useSeo, productSchema, articleSchema } from '@/composables/useSeo'

// En cualquier página:
useSeo({
  title:       'Nike Air Max',
  description: 'Las mejores zapatillas del mercado',
  image:       'https://mitienda.com/img/nike.jpg',
  url:         '/productos/nike-air-max',
  type:        'product',
  schema:      productSchema({
    name:  'Nike Air Max',
    price: 150,
    image: 'https://mitienda.com/img/nike.jpg',
  }),
})
```

**Genera automáticamente:**
- `<title>Nike Air Max | Mi Tienda</title>`
- `<meta name="description" ...>`
- `<meta property="og:title" ...>`
- `<meta property="og:image" ...>`
- `<meta name="twitter:card" content="summary_large_image">`
- `<link rel="canonical" ...>`
- `<script type="application/ld+json">` con schema.org Product

## Tipos de schema.org incluidos

### Product (e-commerce)
```ts
schema: productSchema({
  name:        'Nombre del producto',
  description: 'Descripción',
  image:       'https://...',
  price:       99.99,
  currency:    'USD',
})
```

### Article (blog)
```ts
schema: articleSchema({
  title:       'Título del artículo',
  description: 'Resumen',
  image:       'https://...',
  date:        '2025-01-15',
  author:      'Autor',
})
```

### Organization (página principal)
```ts
schema: orgSchema({
  name: 'Mi Empresa',
  url:  'https://miempresa.com',
  logo: 'https://miempresa.com/logo.png',
})
```

## Sitemap.xml

```bash
# Genera sitemap con todas las rutas estáticas
oweeme sitemap --base https://mitienda.com
```

Para rutas dinámicas (productos, artículos), consulta tu API y agrégalas manualmente:

```xml
<url>
  <loc>https://mitienda.com/productos/nike-air-max</loc>
  <lastmod>2025-01-15</lastmod>
  <priority>0.8</priority>
</url>
```

## robots.txt generado

```
User-agent: *
Allow: /
Sitemap: https://mitienda.com/sitemap.xml
```

## Páginas que NO necesitan SEO

Para páginas privadas (dashboard, perfil, carrito), usa `noindex: true`:

```ts
useSeo({ title: 'Dashboard', noindex: true })
// Genera: <meta name="robots" content="noindex, nofollow">
```
