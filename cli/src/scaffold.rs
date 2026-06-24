use crate::{print, template};
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use indicatif::{ProgressBar, ProgressStyle};
use std::{fs, path::Path, time::Duration};

pub struct ProjectConfig {
    pub name: String,
    pub site_name: String,
    pub site_url: String,
    pub api_url: String,
    pub default_lang: String,
    pub with_pwa: bool,
}

pub fn run(name: &str) {
    let theme = ColorfulTheme::default();

    println!("  {}", "Let's set up your Nuxt 3 + Quasar project".bold());
    println!();

    let site_name: String = Input::with_theme(&theme)
        .with_prompt("Site name")
        .default(to_title(name))
        .interact_text()
        .unwrap();

    let site_url: String = Input::with_theme(&theme)
        .with_prompt("Production URL")
        .default(format!("https://{name}.com"))
        .interact_text()
        .unwrap();

    let api_url: String = Input::with_theme(&theme)
        .with_prompt("API backend URL (REST)")
        .default("http://localhost:8080/api".into())
        .interact_text()
        .unwrap();

    let langs = &[
        "es — Español", "en — English", "pt — Português",
        "de — Deutsch", "fr — Français", "ru — Русский",
        "ko — 한국어", "ja — 日本語",
    ];
    let lang_codes = &["es", "en", "pt", "de", "fr", "ru", "ko", "ja"];
    let lang_idx = Select::with_theme(&theme)
        .with_prompt("Default language")
        .default(0)
        .items(langs)
        .interact()
        .unwrap();
    let default_lang = lang_codes[lang_idx].to_string();

    let with_pwa = Confirm::with_theme(&theme)
        .with_prompt("Enable PWA (manifest + service worker)?")
        .default(true)
        .interact()
        .unwrap();

    println!();

    let config = ProjectConfig {
        name: name.to_string(),
        site_name,
        site_url,
        api_url: api_url.clone(),
        default_lang,
        with_pwa,
    };

    generate_project(&config);
    print::done(name, &api_url);
}

fn generate_project(cfg: &ProjectConfig) {
    let root = Path::new(&cfg.name);

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner:.magenta} {msg}")
            .unwrap(),
    );
    pb.enable_steady_tick(Duration::from_millis(80));

    pb.set_message("Creating project structure...");
    for dir in &[
        "pages/productos/[categoria]",
        "layouts",
        "components",
        "composables",
        "public",
        "assets/css",
        "locales",
        "server/api",
    ] {
        fs::create_dir_all(root.join(dir)).unwrap();
    }
    pb.finish_and_clear();

    let total = 6u8;

    // 1. Config
    print::step(1, total, "Nuxt + Quasar config");
    write(root, "package.json",       &template::nuxt_package_json(&cfg.name));
    write(root, "nuxt.config.ts",     &template::nuxt_config(cfg));
    write(root, "tsconfig.json",      template::tsconfig());
    write(root, ".env.example",       &template::nuxt_env_example(cfg));
    write(root, ".gitignore",         template::nuxt_gitignore());
    print::ok("nuxt.config.ts + package.json");

    // 2. App root
    print::step(2, total, "App shell");
    write(root, "app.vue",                        template::nuxt_app_vue());
    write(root, "layouts/default.vue",            &template::nuxt_layout_default(cfg));
    write(root, "error.vue",                      template::nuxt_error_vue());
    print::ok("app.vue + layout + error page");

    // 3. Pages (rutas automáticas)
    print::step(3, total, "Pages + routing");
    write(root, "pages/index.vue",                        &template::page_index(cfg));
    write(root, "pages/servicios.vue",                    &template::page_servicios(cfg));
    write(root, "pages/productos/index.vue",              &template::page_productos(cfg));
    write(root, "pages/productos/[categoria]/index.vue",  template::page_categoria());
    write(root, "pages/productos/[categoria]/[id].vue",   template::page_producto_item());
    write(root, "pages/blog/index.vue",                   &template::page_blog(cfg));
    write(root, "pages/blog/[slug].vue",                  &template::page_blog_post(cfg));
    write(root, "pages/contacto.vue",                     &template::page_contacto(cfg));
    print::ok("index / servicios / productos / blog / contacto");

    // 4. Components
    print::step(4, total, "Components");
    write(root, "components/AppHeader.vue",    &template::comp_header(cfg));
    write(root, "components/AppFooter.vue",    &template::comp_footer(cfg));
    write(root, "components/ProductCard.vue",  template::comp_product_card());
    write(root, "components/BlogCard.vue",     template::comp_blog_card());
    write(root, "components/HeroSection.vue",  &template::comp_hero(cfg));
    print::ok("Header / Footer / Hero / Cards");

    // 5. Composables + i18n + CSS
    print::step(5, total, "Composables + i18n + styles");
    write(root, "composables/useApi.ts",       &template::composable_use_api(cfg));
    write(root, "composables/useSeo.ts",       template::composable_use_seo());
    write(root, "assets/css/main.css",         template::nuxt_css());
    for (code, json) in template::all_locales() {
        write(root, &format!("locales/{code}.json"), &json);
    }
    print::ok("useApi + useSeo + CSS + 8 locales");

    // 6. Logo
    print::step(6, total, "Assets");
    let logo_bytes: &[u8] = include_bytes!("../assets/oweelogo.png");
    let logo_path = root.join("public/oweelogo.png");
    fs::write(&logo_path, logo_bytes).unwrap_or_else(|e| {
        eprintln!("{} writing logo: {e}", "error".red());
    });
    if cfg.with_pwa {
        write(root, "public/manifest.json", &template::pwa_manifest(cfg));
    }
    print::ok("logo + assets");
}

fn write(root: &Path, relative: &str, content: impl AsRef<str>) {
    let content = content.as_ref();
    let path = root.join(relative);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(&path, content).unwrap_or_else(|e| {
        eprintln!("{} writing {}: {e}", "error".red(), path.display());
    });
}

fn to_title(s: &str) -> String {
    s.split(['-', '_'])
        .map(|w| {
            let mut c = w.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
