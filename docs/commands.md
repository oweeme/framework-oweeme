# Comandos — Framework Oweeme CLI

## oweeme new \<nombre\>

Crea un nuevo proyecto Quasar SPA completo.

```bash
oweeme new mi-tienda
```

**Preguntas interactivas:**
- Nombre del sitio
- URL de producción
- URL del API backend
- ¿Habilitar PWA?

**Output:**
```
mi-tienda/
├── src/
│   ├── pages/Index.vue        ← landing con SEO
│   ├── layouts/MainLayout.vue
│   ├── components/            ← AppHeader, AppFooter, HeroSection
│   ├── composables/           ← useApi, useSeo
│   ├── stores/app.ts          ← Pinia
│   ├── router/                ← Vue Router
│   ├── css/                   ← Quasar + paleta Koi
│   ├── App.vue
│   └── main.ts
├── public/
│   ├── oweelogo.png
│   ├── robots.txt
│   └── sitemap.xml
├── vite.config.ts
├── package.json
├── tsconfig.json
└── .env.example
```

---

## oweeme add \<módulo\>

Agrega un módulo funcional al proyecto actual.

```bash
oweeme add auth        # sistema de autenticación
oweeme add blog        # módulo de blog
oweeme add ecommerce   # tienda con carrito
oweeme add dashboard   # panel de administración
oweeme add rrhh        # gestión de empleados
```

### Módulo: auth
```
src/stores/auth.ts            ← Pinia store con login/logout/register
src/pages/auth/Login.vue      ← formulario de login
src/pages/auth/Register.vue   ← formulario de registro
src/pages/auth/Profile.vue    ← perfil del usuario
src/router/auth.routes.ts     ← rutas a agregar manualmente
```

### Módulo: blog
```
src/pages/blog/Index.vue      ← listado de artículos
src/pages/blog/[slug].vue     ← artículo individual con schema.org Article
src/components/BlogCard.vue   ← tarjeta de artículo
```

### Módulo: ecommerce
```
src/stores/cart.ts            ← Pinia cart (localStorage persistente)
src/components/ProductCard.vue
src/pages/productos/Index.vue
src/pages/carrito/Index.vue   ← carrito completo con cantidades
```

### Módulo: dashboard
```
src/layouts/DashboardLayout.vue ← layout admin con sidebar colapsable
src/pages/dashboard/Index.vue   ← estadísticas con cards
```

### Módulo: rrhh
```
src/stores/rrhh.ts              ← CRUD de empleados
src/pages/rrhh/Empleados.vue    ← tabla con búsqueda
```

---

## oweeme page \<nombre\>

Genera una nueva página con SEO configurado.

```bash
oweeme page Servicios
oweeme page ProductoDetalle --route="/productos/:id"
oweeme page AdminPanel --route="/admin" --auth
```

**Opciones:**
- `--route <ruta>` — URL de la página (default: /nombre en minúsculas)
- `--auth` — agrega guard de autenticación

**Output:** `src/pages/<Nombre>.vue` con `useSeo()` listo.

---

## oweeme component \<nombre\>

Genera un componente Vue tipado con props.

```bash
oweeme component ProductCard
oweeme component UserAvatar --props="nombre:string,avatar:string,tamaño:number"
```

**Opciones:**
- `--props <lista>` — props en formato `nombre:tipo,nombre:tipo`

**Output:** `src/components/<Nombre>.vue` con defineProps tipado.

---

## oweeme sitemap

Genera `public/sitemap.xml` leyendo las rutas estáticas de `src/router/routes.ts`.

```bash
oweeme sitemap                              # usa VITE_SITE_URL del .env
oweeme sitemap --base https://mitienda.com  # URL explícita
```

**Output:** `public/sitemap.xml` con todas las rutas estáticas con prioridades automáticas.

> Las rutas dinámicas (`:id`, `:slug`) se omiten — agrégalas manualmente si necesitas indexar productos/artículos específicos.

---

## oweeme info

Muestra todos los comandos y módulos disponibles.

```bash
oweeme info
```
