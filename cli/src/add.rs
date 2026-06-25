use crate::{print, template};
use colored::Colorize;
use std::{fs, path::Path};

const MODULES: &[&str] = &["auth", "blog", "ecommerce", "dashboard", "rrhh", "capacitor"];

pub fn run(module: &str) {
    let src = Path::new("src");
    if !src.exists() {
        eprintln!(
            "{} Ejecuta este comando desde la raíz del proyecto (donde está src/).",
            "error:".red().bold()
        );
        std::process::exit(1);
    }

    match module {
        "auth"      => add_auth(),
        "blog"      => add_blog(),
        "ecommerce" => add_ecommerce(),
        "dashboard" => add_dashboard(),
        "rrhh"      => add_rrhh(),
        "capacitor" => add_capacitor(),
        other => {
            eprintln!(
                "{} Módulo '{}' no reconocido.\n  Disponibles: {}",
                "error:".red().bold(),
                other,
                MODULES.join(", ").bright_white()
            );
            std::process::exit(1);
        }
    }

    print::done_add(module);
}

// ─── Auth ─────────────────────────────────────────────────────────────────────

fn add_auth() {
    println!("  {} Agregando módulo auth...", "→".bright_magenta());

    fs::create_dir_all("src/pages/auth").unwrap();
    fs::create_dir_all("src/stores").unwrap();

    write("src/stores/auth.ts",           template::module_auth_store());
    write("src/pages/auth/Login.vue",     template::module_auth_login());
    write("src/pages/auth/Register.vue",  template::module_auth_register());
    write("src/pages/auth/Profile.vue",   template::module_auth_profile());
    write("src/router/auth.routes.ts",    template::module_auth_routes());

    print::ok("src/stores/auth.ts");
    print::ok("src/pages/auth/Login.vue + Register.vue + Profile.vue");
    print::ok("src/router/auth.routes.ts  ← agrega estas rutas a routes.ts");
}

// ─── Blog ─────────────────────────────────────────────────────────────────────

fn add_blog() {
    println!("  {} Agregando módulo blog...", "→".bright_magenta());

    fs::create_dir_all("src/pages/blog").unwrap();

    write("src/pages/blog/Index.vue",       template::module_blog_index());
    write("src/pages/blog/[slug].vue",      template::module_blog_post());
    write("src/components/BlogCard.vue",    template::module_blog_card());

    print::ok("src/pages/blog/Index.vue + [slug].vue");
    print::ok("src/components/BlogCard.vue");
    print::warn("Agrega las rutas en src/router/routes.ts:");
    println!("    {{ path: '/blog',        component: () => import('@/pages/blog/Index.vue') }},");
    println!("    {{ path: '/blog/:slug',  component: () => import('@/pages/blog/[slug].vue') }},");
}

// ─── Ecommerce ────────────────────────────────────────────────────────────────

fn add_ecommerce() {
    println!("  {} Agregando módulo ecommerce...", "→".bright_magenta());

    fs::create_dir_all("src/pages/productos").unwrap();
    fs::create_dir_all("src/pages/carrito").unwrap();
    fs::create_dir_all("src/stores").unwrap();

    write("src/stores/cart.ts",                    template::module_ecommerce_cart_store());
    write("src/components/ProductCard.vue",        template::module_ecommerce_product_card());
    write("src/pages/productos/Index.vue",         ecommerce_products_page());
    write("src/pages/carrito/Index.vue",           ecommerce_cart_page());

    print::ok("src/stores/cart.ts");
    print::ok("src/components/ProductCard.vue");
    print::ok("src/pages/productos/ + src/pages/carrito/");
    print::warn("Agrega las rutas en src/router/routes.ts:");
    println!("    {{ path: '/productos',  component: () => import('@/pages/productos/Index.vue') }},");
    println!("    {{ path: '/carrito',    component: () => import('@/pages/carrito/Index.vue') }},");
}

fn ecommerce_products_page() -> &'static str {
    r#"<script setup lang="ts">
import { useApi } from '@/composables/useApi'
import { useSeo, productSchema } from '@/composables/useSeo'
import ProductCard from '@/components/ProductCard.vue'

useSeo({ title: 'Productos', description: 'Catálogo de productos', url: '/productos' })

interface Product { id: number; nombre: string; precio: number; imagen?: string; descripcion?: string; slug?: string }
const { data: productos, loading } = useApi<Product[]>('/productos')
</script>

<template>
  <q-page class="oweeme-page q-pa-xl">
    <h1 class="text-h4 text-cream q-mb-lg">Productos</h1>
    <q-inner-loading :showing="loading" color="primary" />
    <div v-if="!loading" class="row q-col-gutter-lg">
      <div v-for="p in productos" :key="p.id" class="col-12 col-sm-6 col-md-4 col-lg-3">
        <ProductCard :product="p" />
      </div>
      <div v-if="!productos?.length" class="col-12 text-center q-pt-xl">
        <q-icon name="inventory_2" size="4rem" color="secondary" />
        <p class="text-muted q-mt-md">Sin productos. Conecta tu API.</p>
      </div>
    </div>
  </q-page>
</template>
"#
}

fn ecommerce_cart_page() -> &'static str {
    r#"<script setup lang="ts">
import { useCartStore } from '@/stores/cart'
import { useSeo } from '@/composables/useSeo'

useSeo({ title: 'Carrito', noindex: true })
const cart = useCartStore()
</script>

<template>
  <q-page class="oweeme-page q-pa-xl">
    <h1 class="text-h5 text-cream q-mb-lg">Carrito de compras</h1>
    <div v-if="cart.isEmpty" class="text-center q-pt-xl">
      <q-icon name="shopping_cart" size="4rem" color="secondary" />
      <p class="text-muted q-mt-md">Tu carrito está vacío.</p>
      <q-btn color="primary" label="Ver productos" to="/productos" unelevated />
    </div>
    <div v-else>
      <q-card class="oweeme-card q-mb-lg">
        <q-list separator>
          <q-item v-for="item in cart.items" :key="item.id" class="q-py-md">
            <q-item-section avatar>
              <q-img v-if="item.imagen" :src="item.imagen" width="60px" style="border-radius:8px;" />
              <q-icon v-else name="inventory_2" size="3rem" color="secondary" />
            </q-item-section>
            <q-item-section>
              <q-item-label class="text-cream">{{ item.nombre }}</q-item-label>
              <q-item-label caption class="text-primary">${{ item.precio }}</q-item-label>
            </q-item-section>
            <q-item-section side class="flex items-center q-gutter-sm">
              <q-btn flat round icon="remove" size="sm" color="primary" @click="cart.updateQty(item.id, item.cantidad - 1)" />
              <span class="text-cream">{{ item.cantidad }}</span>
              <q-btn flat round icon="add"    size="sm" color="primary" @click="cart.updateQty(item.id, item.cantidad + 1)" />
              <q-btn flat round icon="delete" size="sm" color="negative" @click="cart.remove(item.id)" />
            </q-item-section>
          </q-item>
        </q-list>
      </q-card>
      <div class="flex justify-between items-center q-mb-lg">
        <span class="text-h6 text-cream">Total: <strong class="text-primary">${{ cart.total }}</strong></span>
        <q-btn color="primary" label="Finalizar compra" unelevated icon="payment" size="lg" />
      </div>
      <q-btn flat color="negative" label="Vaciar carrito" icon="delete_sweep" @click="cart.clear()" />
    </div>
  </q-page>
</template>
"#
}

// ─── Dashboard ────────────────────────────────────────────────────────────────

fn add_dashboard() {
    println!("  {} Agregando módulo dashboard...", "→".bright_magenta());

    fs::create_dir_all("src/pages/dashboard").unwrap();
    fs::create_dir_all("src/layouts").unwrap();

    write("src/layouts/DashboardLayout.vue", template::module_dashboard_layout());
    write("src/pages/dashboard/Index.vue",   template::module_dashboard_index());

    print::ok("src/layouts/DashboardLayout.vue");
    print::ok("src/pages/dashboard/Index.vue");
    print::warn("Agrega las rutas en src/router/routes.ts:");
    println!("    {{ path: '/dashboard', component: DashboardLayout,");
    println!("       children: [{{ path: '', component: () => import('@/pages/dashboard/Index.vue') }}] }},");
}

// ─── RRHH ─────────────────────────────────────────────────────────────────────

fn add_rrhh() {
    println!("  {} Agregando módulo rrhh...", "→".bright_magenta());

    fs::create_dir_all("src/pages/rrhh").unwrap();
    fs::create_dir_all("src/stores").unwrap();

    write("src/stores/rrhh.ts",           template::module_rrhh_store());
    write("src/pages/rrhh/Empleados.vue", template::module_rrhh_empleados());

    print::ok("src/stores/rrhh.ts");
    print::ok("src/pages/rrhh/Empleados.vue");
    print::warn("Agrega las rutas en src/router/routes.ts:");
    println!("    {{ path: '/rrhh/empleados', component: () => import('@/pages/rrhh/Empleados.vue') }},");
}

fn add_capacitor() {
    println!("  {} Configurando Capacitor para Android/iOS...", "→".bright_magenta());

    // capacitor.config.ts
    write("capacitor.config.ts", template::capacitor_config());

    // scripts/android.sh — helper para build + sync
    fs::create_dir_all("scripts").unwrap();
    write("scripts/android.sh", template::capacitor_android_script());
    // hacerlo ejecutable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(meta) = fs::metadata("scripts/android.sh") {
            let mut perm = meta.permissions();
            perm.set_mode(0o755);
            fs::set_permissions("scripts/android.sh", perm).ok();
        }
    }

    print::ok("capacitor.config.ts");
    print::ok("scripts/android.sh");

    println!();
    println!("  {} Próximos pasos:", "Capacitor".bright_cyan().bold());
    println!("    1. npm install @capacitor/core @capacitor/cli @capacitor/android @capacitor/ios");
    println!("    2. npx cap init");
    println!("    3. npx cap add android");
    println!("    4. npm run build && npx cap sync");
    println!("    5. npx cap open android   # abre Android Studio");
    println!();
    println!("  {} O usa el script: bash scripts/android.sh", "→".bright_white());
}

// ─── Util ─────────────────────────────────────────────────────────────────────

fn write(path: &str, content: &str) {
    let p = Path::new(path);
    if p.exists() {
        print::warn(&format!("{path} ya existe — omitiendo"));
        return;
    }
    if let Some(parent) = p.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(p, content).unwrap_or_else(|e| {
        eprintln!("{} escribiendo {path}: {e}", "error".red());
    });
}
