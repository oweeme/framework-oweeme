use crate::scaffold::ProjectConfig;

// ─── Package.json ─────────────────────────────────────────────────────────────

pub fn nuxt_package_json(name: &str) -> String {
    format!(
        r#"{{
  "name": "{name}",
  "version": "0.1.0",
  "private": true,
  "scripts": {{
    "dev":      "nuxt dev",
    "build":    "nuxt build",
    "generate": "nuxt generate",
    "preview":  "nuxt preview",
    "lint":     "eslint ."
  }},
  "dependencies": {{
    "nuxt":            "^3.13.0",
    "nuxt-quasar-ui":  "^2.1.9",
    "quasar":          "^2.16.0",
    "@quasar/extras":  "^1.16.0",
    "vue":             "^3.5.0",
    "vue-router":      "^4.4.0",
    "@vueuse/nuxt":    "^11.0.0",
    "@nuxtjs/i18n":    "^8.5.0",
    "axios":           "1.7.9"
  }},
  "devDependencies": {{
    "typescript":      "^5.5.0",
    "vue-tsc":         "^2.1.0"
  }}
}}
"#
    )
}

// ─── nuxt.config.ts ───────────────────────────────────────────────────────────

pub fn nuxt_config(cfg: &ProjectConfig) -> String {
    let pwa_head = if cfg.with_pwa {
        r#"{ rel: 'manifest', href: '/manifest.json' },"#
    } else {
        ""
    };
    format!(
        r#"// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({{
  devtools: {{ enabled: true }},

  // SSG — genera dist/ estático para cualquier hosting
  ssr: true,
  nitro: {{
    preset: 'static',
    prerender: {{
      crawlLinks: true,
      routes: ['/', '/servicios', '/productos', '/blog', '/contacto'],
    }},
  }},

  modules: [
    'nuxt-quasar-ui',
    '@vueuse/nuxt',
    '@nuxtjs/i18n',
  ],

  quasar: {{
    sassVariables: false,
    plugins: ['Notify', 'Dialog', 'Loading'],
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
    extras: {{
      fontIcons: ['material-icons'],
      fonts: ['Roboto'],
    }},
  }},

  i18n: {{
    locales: [
      {{ code: 'es', file: 'es.json', name: 'Español' }},
      {{ code: 'en', file: 'en.json', name: 'English' }},
      {{ code: 'pt', file: 'pt.json', name: 'Português' }},
      {{ code: 'de', file: 'de.json', name: 'Deutsch' }},
      {{ code: 'fr', file: 'fr.json', name: 'Français' }},
      {{ code: 'ru', file: 'ru.json', name: 'Русский' }},
      {{ code: 'ko', file: 'ko.json', name: '한국어' }},
      {{ code: 'ja', file: 'ja.json', name: '日本語' }},
    ],
    lazy: true,
    langDir: 'i18n/locales',
    defaultLocale: '{lang}',
    strategy: 'prefix_except_default',
  }},

  runtimeConfig: {{
    public: {{
      apiBase:  process.env.NUXT_PUBLIC_API_BASE  || '{api}',
      siteUrl:  process.env.NUXT_PUBLIC_SITE_URL  || '{url}',
      siteName: process.env.NUXT_PUBLIC_SITE_NAME || '{name}',
    }},
  }},

  app: {{
    head: {{
      charset: 'utf-8',
      viewport: 'width=device-width, initial-scale=1',
      link: [
        {pwa_head}
        {{ rel: 'icon', type: 'image/png', href: '/oweelogo.png' }},
      ],
    }},
  }},

  css: ['~/assets/css/main.css'],

  compatibilityDate: '2024-11-01',
}})
"#,
        lang = cfg.default_lang,
        api  = cfg.api_url,
        url  = cfg.site_url,
        name = cfg.site_name,
    )
}

// ─── tsconfig.json ────────────────────────────────────────────────────────────

pub fn tsconfig() -> &'static str {
    r#"{
  "extends": "./.nuxt/tsconfig.json"
}
"#
}

// ─── .env.example ─────────────────────────────────────────────────────────────

pub fn nuxt_env_example(cfg: &ProjectConfig) -> String {
    format!(
        r#"NUXT_PUBLIC_API_BASE={}
NUXT_PUBLIC_SITE_URL={}
NUXT_PUBLIC_SITE_NAME={}
"#,
        cfg.api_url, cfg.site_url, cfg.site_name
    )
}

// ─── .gitignore ───────────────────────────────────────────────────────────────

pub fn nuxt_gitignore() -> &'static str {
    r#"node_modules/
.nuxt/
.output/
dist/
.env
*.local
.DS_Store
"#
}

// ─── app.vue ──────────────────────────────────────────────────────────────────

pub fn nuxt_app_vue() -> &'static str {
    r#"<template>
  <NuxtLayout>
    <NuxtPage />
  </NuxtLayout>
</template>
"#
}

// ─── layouts/default.vue ──────────────────────────────────────────────────────

pub fn nuxt_layout_default(cfg: &ProjectConfig) -> String {
    format!(
        r#"<script setup lang="ts">
const config = useRuntimeConfig()
const siteName = config.public.siteName
</script>

<template>
  <q-layout view="hHh lpR fFf">
    <AppHeader />
    <q-page-container>
      <slot />
    </q-page-container>
    <AppFooter />
  </q-layout>
</template>
"#
    )
}

// ─── error.vue ────────────────────────────────────────────────────────────────

pub fn nuxt_error_vue() -> &'static str {
    r#"<script setup lang="ts">
const props = defineProps<{ error: { statusCode: number; message: string } }>()

useSeoMeta({
  title: props.error.statusCode === 404 ? 'Página no encontrada' : 'Error',
  robots: 'noindex',
})
</script>

<template>
  <q-layout view="hHh lpR fFf">
    <AppHeader />
    <q-page-container>
      <q-page class="flex flex-center column q-gutter-md oweeme-page">
        <h1 class="text-h1 text-primary" style="font-size:6rem;margin:0;">
          {{ error.statusCode }}
        </h1>
        <p class="text-h5 text-cream">
          {{ error.statusCode === 404 ? 'Página no encontrada' : 'Error del servidor' }}
        </p>
        <q-btn to="/" color="primary" label="Volver al inicio" unelevated />
      </q-page>
    </q-page-container>
    <AppFooter />
  </q-layout>
</template>
"#
}

// ─── Pages ────────────────────────────────────────────────────────────────────

pub fn page_index(cfg: &ProjectConfig) -> String {
    format!(
        r#"<script setup lang="ts">
const config = useRuntimeConfig()

useSeoMeta({{
  title: '{name}',
  description: 'Bienvenido a {name} — {url}',
  ogTitle: '{name}',
  ogDescription: 'Bienvenido a {name}',
  ogImage: '/oweelogo.png',
  ogUrl: '{url}',
  twitterCard: 'summary_large_image',
}})
</script>

<template>
  <q-page class="oweeme-page">
    <HeroSection />

    <section class="q-pa-xl">
      <div class="row q-col-gutter-lg justify-center">
        <div class="col-12 col-md-4">
          <q-card class="oweeme-card">
            <q-card-section>
              <q-icon name="shopping_cart" color="primary" size="2rem" />
              <h3 class="text-h6 text-cream q-mt-sm">Productos</h3>
              <p class="text-muted">Explora nuestro catálogo completo.</p>
            </q-card-section>
            <q-card-actions>
              <q-btn flat color="primary" to="/productos" label="Ver productos" />
            </q-card-actions>
          </q-card>
        </div>
        <div class="col-12 col-md-4">
          <q-card class="oweeme-card">
            <q-card-section>
              <q-icon name="build" color="primary" size="2rem" />
              <h3 class="text-h6 text-cream q-mt-sm">Servicios</h3>
              <p class="text-muted">Lo que podemos hacer por ti.</p>
            </q-card-section>
            <q-card-actions>
              <q-btn flat color="primary" to="/servicios" label="Ver servicios" />
            </q-card-actions>
          </q-card>
        </div>
        <div class="col-12 col-md-4">
          <q-card class="oweeme-card">
            <q-card-section>
              <q-icon name="article" color="primary" size="2rem" />
              <h3 class="text-h6 text-cream q-mt-sm">Blog</h3>
              <p class="text-muted">Noticias y artículos.</p>
            </q-card-section>
            <q-card-actions>
              <q-btn flat color="primary" to="/blog" label="Ver blog" />
            </q-card-actions>
          </q-card>
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

pub fn page_servicios(cfg: &ProjectConfig) -> String {
    format!(
        r#"<script setup lang="ts">
const {{ data: servicios, pending }} = await useApi('/servicios')

useSeoMeta({{
  title: `Servicios | {name}`,
  description: 'Conoce todos nuestros servicios profesionales.',
  ogTitle: `Servicios | {name}`,
  ogUrl: '{url}/servicios',
}})
</script>

<template>
  <q-page class="oweeme-page q-pa-xl">
    <h1 class="text-h4 text-cream q-mb-lg">Nuestros Servicios</h1>

    <q-inner-loading :showing="pending" color="primary" />

    <div v-if="!pending" class="row q-col-gutter-lg">
      <div
        v-for="s in servicios"
        :key="s.id"
        class="col-12 col-md-6 col-lg-4"
      >
        <q-card class="oweeme-card full-height">
          <q-card-section>
            <q-icon :name="s.icono || 'star'" color="primary" size="2rem" />
            <h2 class="text-h6 text-cream q-mt-sm">{{{{ s.titulo }}}}</h2>
            <p class="text-muted">{{{{ s.descripcion }}}}</p>
          </q-card-section>
        </q-card>
      </div>

      <div v-if="!servicios?.length" class="col-12 text-center text-muted">
        <p>Conecta tu API en <code>.env</code> para ver los servicios.</p>
        <q-btn outline color="primary" to="/contacto" label="Contactar" class="q-mt-md" />
      </div>
    </div>
  </q-page>
</template>
"#,
        name = cfg.site_name,
        url  = cfg.site_url,
    )
}

pub fn page_productos(cfg: &ProjectConfig) -> String {
    format!(
        r#"<script setup lang="ts">
const {{ data: categorias, pending }} = await useApi('/productos/categorias')

useSeoMeta({{
  title: `Productos | {name}`,
  description: 'Explora nuestro catálogo de productos por categoría.',
  ogTitle: `Productos | {name}`,
  ogUrl: '{url}/productos',
}})
</script>

<template>
  <q-page class="oweeme-page q-pa-xl">
    <h1 class="text-h4 text-cream q-mb-lg">Catálogo de Productos</h1>

    <q-inner-loading :showing="pending" color="primary" />

    <div v-if="!pending" class="row q-col-gutter-lg">
      <div
        v-for="cat in categorias"
        :key="cat.slug"
        class="col-12 col-sm-6 col-md-4"
      >
        <NuxtLink :to="`/productos/${{cat.slug}}`" class="no-decoration">
          <q-card class="oweeme-card cursor-pointer oweeme-card--hover">
            <q-img
              v-if="cat.imagen"
              :src="cat.imagen"
              :alt="cat.nombre"
              height="160px"
            />
            <q-card-section>
              <h2 class="text-h6 text-cream">{{{{ cat.nombre }}}}</h2>
              <p class="text-muted text-caption">{{{{ cat.total }}}} productos</p>
            </q-card-section>
          </q-card>
        </NuxtLink>
      </div>

      <div v-if="!categorias?.length" class="col-12 text-center text-muted">
        <q-icon name="inventory_2" size="4rem" color="secondary" />
        <p class="q-mt-md">Conecta tu API para mostrar productos.</p>
      </div>
    </div>
  </q-page>
</template>
"#,
        name = cfg.site_name,
        url  = cfg.site_url,
    )
}

pub fn page_categoria() -> &'static str {
    r#"<script setup lang="ts">
const route = useRoute()
const categoria = route.params.categoria as string
const { data: productos, pending } = await useApi(`/productos/${categoria}`)

useSeoMeta({
  title: () => `${categoria} | Productos`,
  description: () => `Productos de la categoría ${categoria}`,
  ogUrl: () => `/productos/${categoria}`,
})
</script>

<template>
  <q-page class="oweeme-page q-pa-xl">
    <div class="flex items-center q-mb-lg q-gutter-sm">
      <q-btn flat icon="arrow_back" to="/productos" color="primary" />
      <h1 class="text-h4 text-cream q-ma-none" style="text-transform:capitalize;">
        {{ categoria }}
      </h1>
    </div>

    <q-inner-loading :showing="pending" color="primary" />

    <div v-if="!pending" class="row q-col-gutter-lg">
      <div
        v-for="p in productos"
        :key="p.id"
        class="col-12 col-sm-6 col-md-4 col-lg-3"
      >
        <ProductCard :product="p" :categoria="categoria" />
      </div>
    </div>
  </q-page>
</template>
"#
}

pub fn page_producto_item() -> &'static str {
    r#"<script setup lang="ts">
const route = useRoute()
const { categoria, id } = route.params as { categoria: string; id: string }
const { data: producto, pending } = await useApi(`/productos/${categoria}/${id}`)

useSeoMeta({
  title: () => producto.value?.nombre || id,
  description: () => producto.value?.descripcion || '',
  ogTitle: () => producto.value?.nombre || id,
  ogImage: () => producto.value?.imagen || '/oweelogo.png',
  ogUrl: () => `/productos/${categoria}/${id}`,
})

// Schema.org Product para SEO máximo
useHead({
  script: [{
    type: 'application/ld+json',
    children: () => JSON.stringify({
      '@context': 'https://schema.org',
      '@type': 'Product',
      name: producto.value?.nombre,
      description: producto.value?.descripcion,
      image: producto.value?.imagen,
      offers: {
        '@type': 'Offer',
        price: producto.value?.precio,
        priceCurrency: 'USD',
        availability: 'https://schema.org/InStock',
      },
    }),
  }],
})
</script>

<template>
  <q-page class="oweeme-page q-pa-xl">
    <q-inner-loading :showing="pending" color="primary" />

    <div v-if="!pending && producto" class="row q-col-gutter-xl">
      <div class="col-12 col-md-6">
        <q-img
          :src="producto.imagen"
          :alt="producto.nombre"
          class="rounded-borders"
          style="border-radius:16px;"
        />
      </div>
      <div class="col-12 col-md-6 flex column q-gutter-md">
        <div class="flex items-center q-gutter-sm">
          <q-btn flat icon="arrow_back" :to="`/productos/${categoria}`" color="primary" dense />
          <q-chip :label="categoria" color="secondary" text-color="white" />
        </div>
        <h1 class="text-h4 text-cream q-ma-none">{{ producto.nombre }}</h1>
        <p class="text-h5 text-primary q-ma-none">${{ producto.precio }}</p>
        <p class="text-muted">{{ producto.descripcion }}</p>
        <q-btn color="primary" label="Agregar al carrito" unelevated size="lg" icon="add_shopping_cart" />
      </div>
    </div>
  </q-page>
</template>
"#
}

pub fn page_blog(cfg: &ProjectConfig) -> String {
    format!(
        r#"<script setup lang="ts">
const {{ data: posts, pending }} = await useApi('/blog')

useSeoMeta({{
  title: `Blog | {name}`,
  description: 'Artículos, noticias y recursos de {name}.',
  ogUrl: '{url}/blog',
}})
</script>

<template>
  <q-page class="oweeme-page q-pa-xl">
    <h1 class="text-h4 text-cream q-mb-lg">Blog</h1>
    <q-inner-loading :showing="pending" color="primary" />
    <div v-if="!pending" class="row q-col-gutter-lg">
      <div v-for="post in posts" :key="post.slug" class="col-12 col-md-6 col-lg-4">
        <BlogCard :post="post" />
      </div>
      <div v-if="!posts?.length" class="col-12 text-center text-muted">
        <q-icon name="article" size="4rem" color="secondary" />
        <p class="q-mt-md">No hay artículos aún. Conecta tu API.</p>
      </div>
    </div>
  </q-page>
</template>
"#,
        name = cfg.site_name,
        url  = cfg.site_url,
    )
}

pub fn page_blog_post(cfg: &ProjectConfig) -> String {
    format!(
        r#"<script setup lang="ts">
const route = useRoute()
const slug = route.params.slug as string
const {{ data: post, pending }} = await useApi(`/blog/${{slug}}`)

useSeoMeta({{
  title: () => `${{post.value?.titulo || slug}} | {name}`,
  description: () => post.value?.descripcion || '',
  ogTitle: () => post.value?.titulo,
  ogImage: () => post.value?.imagen || '/oweelogo.png',
  articlePublishedTime: () => post.value?.fecha,
  ogUrl: () => `{url}/blog/${{slug}}`,
}})

useHead({{
  script: [{{
    type: 'application/ld+json',
    children: () => JSON.stringify({{
      '@context': 'https://schema.org',
      '@type': 'Article',
      headline: post.value?.titulo,
      description: post.value?.descripcion,
      image: post.value?.imagen,
      datePublished: post.value?.fecha,
      author: {{ '@type': 'Person', name: post.value?.autor }},
    }}),
  }}],
}})
</script>

<template>
  <q-page class="oweeme-page q-pa-xl">
    <q-inner-loading :showing="pending" color="primary" />
    <article v-if="!pending && post" style="max-width:780px;margin:0 auto;">
      <q-btn flat icon="arrow_back" to="/blog" color="primary" class="q-mb-md" />
      <q-img v-if="post.imagen" :src="post.imagen" :alt="post.titulo"
             class="q-mb-xl" style="border-radius:16px;max-height:420px;" />
      <h1 class="text-h4 text-cream">{{{{ post.titulo }}}}</h1>
      <div class="flex items-center q-gutter-sm q-mb-lg">
        <q-chip :label="post.autor" icon="person" color="secondary" text-color="white" />
        <q-chip :label="post.fecha" icon="event" flat />
      </div>
      <div class="prose text-muted" v-html="post.contenido" />
    </article>
  </q-page>
</template>
"#,
        name = cfg.site_name,
        url  = cfg.site_url,
    )
}

pub fn page_contacto(cfg: &ProjectConfig) -> String {
    format!(
        r#"<script setup lang="ts">
const form = reactive({{ nombre: '', email: '', mensaje: '' }})
const sending = ref(false)
const sent = ref(false)

useSeoMeta({{
  title: `Contacto | {name}`,
  description: 'Ponte en contacto con nosotros.',
  ogUrl: '{url}/contacto',
}})

async function enviar() {{
  sending.value = true
  // Conecta con tu API aquí
  await new Promise(r => setTimeout(r, 1000))
  sent.value = true
  sending.value = false
}}
</script>

<template>
  <q-page class="oweeme-page q-pa-xl flex flex-center">
    <q-card class="oweeme-card" style="width:100%;max-width:560px;">
      <q-card-section>
        <h1 class="text-h5 text-cream q-mb-lg">Contacto</h1>
        <q-form v-if="!sent" @submit.prevent="enviar" class="q-gutter-md">
          <q-input v-model="form.nombre" label="Nombre" outlined dark
                   :rules="[v => !!v || 'Requerido']" />
          <q-input v-model="form.email" label="Email" type="email" outlined dark
                   :rules="[v => !!v || 'Requerido']" />
          <q-input v-model="form.mensaje" label="Mensaje" type="textarea" outlined dark
                   rows="4" :rules="[v => !!v || 'Requerido']" />
          <q-btn type="submit" color="primary" label="Enviar mensaje"
                 unelevated :loading="sending" class="full-width" />
        </q-form>
        <div v-else class="text-center q-gutter-md">
          <q-icon name="check_circle" color="positive" size="4rem" />
          <p class="text-h6 text-cream">¡Mensaje enviado!</p>
          <q-btn outline color="primary" label="Enviar otro" @click="sent = false" />
        </div>
      </q-card-section>
    </q-card>
  </q-page>
</template>
"#,
        name = cfg.site_name,
        url  = cfg.site_url,
    )
}

// ─── Components ───────────────────────────────────────────────────────────────

pub fn comp_header(cfg: &ProjectConfig) -> String {
    format!(
        r#"<script setup lang="ts">
const links = [
  {{ label: 'Productos', to: '/productos', icon: 'shopping_cart' }},
  {{ label: 'Servicios',  to: '/servicios',  icon: 'build' }},
  {{ label: 'Blog',       to: '/blog',       icon: 'article' }},
  {{ label: 'Contacto',   to: '/contacto',   icon: 'mail' }},
]
const drawer = ref(false)
</script>

<template>
  <q-header elevated class="oweeme-header">
    <q-toolbar>
      <NuxtLink to="/" class="flex items-center q-gutter-sm no-decoration">
        <img src="/oweelogo.png" alt="{name}" style="height:36px;border-radius:50%;" />
        <span class="text-weight-bold text-cream" style="font-size:1.1rem;">{name}</span>
      </NuxtLink>

      <q-space />

      <!-- Desktop nav -->
      <div class="gt-sm flex q-gutter-sm">
        <q-btn
          v-for="l in links" :key="l.to"
          :to="l.to" flat :label="l.label"
          class="text-cream"
        />
      </div>

      <!-- Mobile hamburger -->
      <q-btn class="lt-md" flat round icon="menu" @click="drawer = !drawer" />
    </q-toolbar>
  </q-header>

  <!-- Mobile drawer -->
  <q-drawer v-model="drawer" side="right" overlay class="oweeme-bg">
    <q-list>
      <q-item
        v-for="l in links" :key="l.to"
        :to="l.to" clickable v-ripple @click="drawer = false"
      >
        <q-item-section avatar>
          <q-icon :name="l.icon" color="primary" />
        </q-item-section>
        <q-item-section class="text-cream">{{{{ l.label }}}}</q-item-section>
      </q-item>
    </q-list>
  </q-drawer>
</template>
"#,
        name = cfg.site_name,
    )
}

pub fn comp_footer(cfg: &ProjectConfig) -> String {
    format!(
        r#"<template>
  <q-footer class="oweeme-footer q-pa-lg text-center">
    <p class="text-muted q-ma-none text-caption">
      © {{{{ new Date().getFullYear() }}}} {name} — Powered by
      <a href="https://github.com/oweeme/framework-oweeme" target="_blank" class="text-primary">
        Oweeme Framework
      </a>
    </p>
  </q-footer>
</template>
"#,
        name = cfg.site_name,
    )
}

pub fn comp_hero(cfg: &ProjectConfig) -> String {
    format!(
        r#"<template>
  <section class="oweeme-hero flex flex-center column text-center q-pa-xl q-gutter-lg">
    <img src="/oweelogo.png" alt="{name}"
         style="width:120px;border-radius:50%;box-shadow:0 0 48px #e8553a66;" />
    <h1 style="color:#f5e2a0;font-size:clamp(2rem,5vw,3.5rem);font-weight:800;margin:0;">
      {name}
    </h1>
    <p style="color:#a8d5c2;font-size:1.2rem;max-width:600px;margin:0;">
      Bienvenido — explora nuestros productos, servicios y mucho más.
    </p>
    <div class="flex q-gutter-md">
      <q-btn to="/productos" color="primary" label="Ver productos" unelevated size="lg" />
      <q-btn to="/contacto" outline color="accent" label="Contactar" size="lg" />
    </div>
  </section>
</template>
"#,
        name = cfg.site_name,
    )
}

pub fn comp_product_card() -> &'static str {
    r#"<script setup lang="ts">
defineProps<{
  product: { id: string; nombre: string; precio: number; imagen?: string; descripcion?: string }
  categoria: string
}>()
</script>

<template>
  <NuxtLink :to="`/productos/${categoria}/${product.id}`" class="no-decoration">
    <q-card class="oweeme-card oweeme-card--hover cursor-pointer full-height">
      <q-img v-if="product.imagen" :src="product.imagen" :alt="product.nombre" height="180px" />
      <q-card-section>
        <h3 class="text-body1 text-cream q-ma-none">{{ product.nombre }}</h3>
        <p class="text-primary text-weight-bold q-mt-xs">${{ product.precio }}</p>
        <p v-if="product.descripcion" class="text-muted text-caption text-ellipsis-2-lines">
          {{ product.descripcion }}
        </p>
      </q-card-section>
    </q-card>
  </NuxtLink>
</template>
"#
}

pub fn comp_blog_card() -> &'static str {
    r#"<script setup lang="ts">
defineProps<{
  post: { slug: string; titulo: string; descripcion?: string; imagen?: string; fecha?: string; autor?: string }
}>()
</script>

<template>
  <NuxtLink :to="`/blog/${post.slug}`" class="no-decoration">
    <q-card class="oweeme-card oweeme-card--hover cursor-pointer full-height">
      <q-img v-if="post.imagen" :src="post.imagen" :alt="post.titulo" height="180px" />
      <q-card-section class="q-gutter-sm">
        <h3 class="text-body1 text-cream q-ma-none">{{ post.titulo }}</h3>
        <p v-if="post.descripcion" class="text-muted text-caption text-ellipsis-2-lines">
          {{ post.descripcion }}
        </p>
        <div class="flex items-center q-gutter-xs text-caption text-muted">
          <q-icon name="person" size="xs" />
          <span>{{ post.autor }}</span>
          <q-icon name="event" size="xs" class="q-ml-sm" />
          <span>{{ post.fecha }}</span>
        </div>
      </q-card-section>
    </q-card>
  </NuxtLink>
</template>
"#
}

// ─── Composables ──────────────────────────────────────────────────────────────

pub fn composable_use_api(cfg: &ProjectConfig) -> String {
    format!(
        r#"// Wrapper sobre $fetch de Nuxt con base URL automática desde .env
export function useApi<T = unknown>(path: string, options?: Parameters<typeof useFetch>[1]) {{
  const config = useRuntimeConfig()
  const base   = config.public.apiBase || '{api}'

  return useFetch<T>(`${{base}}${{path}}`, {{
    ...options,
    onResponseError({{ response }}) {{
      console.error(`[useApi] ${{path}} → ${{response.status}}`)
    }},
  }})
}}
"#,
        api = cfg.api_url,
    )
}

pub fn composable_use_seo() -> &'static str {
    r#"// Helper para generar SEO meta + schema.org de forma sencilla
interface SeoOptions {
  title: string
  description: string
  image?: string
  url?: string
  type?: 'website' | 'article' | 'product'
  schema?: Record<string, unknown>
}

export function useSeo(opts: SeoOptions) {
  const config = useRuntimeConfig()
  const siteUrl = config.public.siteUrl

  useSeoMeta({
    title: opts.title,
    description: opts.description,
    ogTitle: opts.title,
    ogDescription: opts.description,
    ogImage: opts.image || '/oweelogo.png',
    ogUrl: opts.url ? `${siteUrl}${opts.url}` : siteUrl,
    ogType: opts.type || 'website',
    twitterCard: 'summary_large_image',
    twitterTitle: opts.title,
    twitterDescription: opts.description,
    twitterImage: opts.image || '/oweelogo.png',
  })

  if (opts.schema) {
    useHead({
      script: [{
        type: 'application/ld+json',
        children: JSON.stringify({ '@context': 'https://schema.org', ...opts.schema }),
      }],
    })
  }
}
"#
}

// ─── CSS ──────────────────────────────────────────────────────────────────────

pub fn nuxt_css() -> &'static str {
    r#"/* Oweeme Framework — Design System (Koi palette) */

:root {
  --oweeme-bg:       #0d3d2e;
  --oweeme-surface:  #112e22;
  --oweeme-teal:     #1a5c47;
  --oweeme-coral:    #e8553a;
  --oweeme-cream:    #f5e2a0;
  --oweeme-mint:     #a8d5c2;
  --oweeme-muted:    #5a9e80;
  --oweeme-radius:   14px;
  --oweeme-shadow:   0 4px 32px rgba(0,0,0,0.4);
}

/* Base */
body {
  background: var(--oweeme-bg);
  font-family: 'Roboto', system-ui, sans-serif;
}

/* Layout helpers */
.oweeme-page    { background: var(--oweeme-bg); min-height: 100vh; }
.oweeme-bg      { background: var(--oweeme-surface) !important; }
.oweeme-header  { background: var(--oweeme-teal) !important; }
.oweeme-footer  { background: var(--oweeme-surface) !important; border-top: 1px solid rgba(232,85,58,0.15); }
.oweeme-hero    { background: radial-gradient(ellipse at 50% 0%, #1a5c4780 0%, var(--oweeme-bg) 70%); min-height: 70vh; }

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

/* Text helpers */
.text-cream  { color: var(--oweeme-cream) !important; }
.text-muted  { color: var(--oweeme-muted) !important; }
.text-mint   { color: var(--oweeme-mint)  !important; }

/* Links */
.no-decoration { text-decoration: none; color: inherit; }

/* Prose (blog content) */
.prose { line-height: 1.8; color: var(--oweeme-mint); }
.prose h2 { color: var(--oweeme-cream); margin-top: 2rem; }
.prose a  { color: var(--oweeme-coral); }
.prose code {
  background: var(--oweeme-teal);
  padding: 2px 6px;
  border-radius: 4px;
  font-size: 0.9em;
}

/* Scrollbar */
::-webkit-scrollbar       { width: 6px; }
::-webkit-scrollbar-track { background: var(--oweeme-bg); }
::-webkit-scrollbar-thumb { background: var(--oweeme-teal); border-radius: 3px; }
"#
}

// ─── PWA manifest ─────────────────────────────────────────────────────────────

pub fn pwa_manifest(cfg: &ProjectConfig) -> String {
    let n = &cfg.site_name;
    format!(
        "{{\n  \"name\": \"{n}\",\n  \"short_name\": \"{n}\",\n  \"start_url\": \"/\",\n  \"display\": \"standalone\",\n  \"background_color\": \"#0d3d2e\",\n  \"theme_color\": \"#e8553a\",\n  \"icons\": [\n    {{ \"src\": \"/oweelogo.png\", \"sizes\": \"512x512\", \"type\": \"image/png\" }}\n  ]\n}}\n"
    )
}

// ─── i18n locales ─────────────────────────────────────────────────────────────

pub fn all_locales() -> Vec<(&'static str, &'static str)> {
    vec![
        ("es", r#"{"nav":{"home":"Inicio","products":"Productos","services":"Servicios","blog":"Blog","contact":"Contacto"},"hero":{"cta":"Ver productos","cta2":"Contactar"},"footer":{"rights":"Todos los derechos reservados"}}"#),
        ("en", r#"{"nav":{"home":"Home","products":"Products","services":"Services","blog":"Blog","contact":"Contact"},"hero":{"cta":"View products","cta2":"Contact us"},"footer":{"rights":"All rights reserved"}}"#),
        ("pt", r#"{"nav":{"home":"Início","products":"Produtos","services":"Serviços","blog":"Blog","contact":"Contato"},"hero":{"cta":"Ver produtos","cta2":"Contatar"},"footer":{"rights":"Todos os direitos reservados"}}"#),
        ("de", r#"{"nav":{"home":"Startseite","products":"Produkte","services":"Dienstleistungen","blog":"Blog","contact":"Kontakt"},"hero":{"cta":"Produkte ansehen","cta2":"Kontaktieren"},"footer":{"rights":"Alle Rechte vorbehalten"}}"#),
        ("fr", r#"{"nav":{"home":"Accueil","products":"Produits","services":"Services","blog":"Blog","contact":"Contact"},"hero":{"cta":"Voir les produits","cta2":"Nous contacter"},"footer":{"rights":"Tous droits réservés"}}"#),
        ("ru", r#"{"nav":{"home":"Главная","products":"Товары","services":"Услуги","blog":"Блог","contact":"Контакт"},"hero":{"cta":"Смотреть товары","cta2":"Связаться"},"footer":{"rights":"Все права защищены"}}"#),
        ("ko", r#"{"nav":{"home":"홈","products":"제품","services":"서비스","blog":"블로그","contact":"연락처"},"hero":{"cta":"제품 보기","cta2":"문의하기"},"footer":{"rights":"모든 권리 보유"}}"#),
        ("ja", r#"{"nav":{"home":"ホーム","products":"製品","services":"サービス","blog":"ブログ","contact":"お問い合わせ"},"hero":{"cta":"製品を見る","cta2":"お問い合わせ"},"footer":{"rights":"全著作権所有"}}"#),
    ]
}
