use crate::{print, template};
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use indicatif::{ProgressBar, ProgressStyle};
use std::{fs, path::Path, time::Duration};

#[derive(Debug, Clone, PartialEq)]
pub enum Lang { TypeScript, JavaScript }

pub struct ProjectConfig {
    pub name:      String,
    pub site_name: String,
    pub site_url:  String,
    pub lang:      Lang,
    pub with_pwa:  bool,
}

impl ProjectConfig {
    pub fn is_ts(&self) -> bool { self.lang == Lang::TypeScript }
    pub fn ext(&self) -> &'static str { if self.is_ts() { "ts" } else { "js" } }
}

pub fn run(name: &str) {
    let theme = ColorfulTheme::default();

    println!("  {}", "Configuremos tu proyecto Quasar SPA".bold());
    println!();

    let site_name: String = Input::with_theme(&theme)
        .with_prompt("Nombre del sitio")
        .default(to_title(name))
        .interact_text()
        .unwrap();

    let site_url: String = Input::with_theme(&theme)
        .with_prompt("URL de producción")
        .default(format!("https://{name}.com"))
        .interact_text()
        .unwrap();

    let lang_idx = Select::with_theme(&theme)
        .with_prompt("Lenguaje")
        .items(&["TypeScript  (recomendado — tipado, autocompletado)", "JavaScript  (sin tipos)"])
        .default(0)
        .interact()
        .unwrap();

    let lang = if lang_idx == 0 { Lang::TypeScript } else { Lang::JavaScript };

    let with_pwa = Confirm::with_theme(&theme)
        .with_prompt("¿Habilitar PWA (manifest + service worker + offline)?")
        .default(true)
        .interact()
        .unwrap();

    println!();

    let config = ProjectConfig { name: name.to_string(), site_name, site_url, lang, with_pwa };
    generate_project(&config);
    print::done_new(name);
}

fn generate_project(cfg: &ProjectConfig) {
    let root = Path::new(&cfg.name);

    if root.exists() {
        eprintln!("{} El directorio '{}' ya existe.", "error:".red().bold(), cfg.name);
        std::process::exit(1);
    }

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner:.magenta} {msg}")
            .unwrap(),
    );
    pb.enable_steady_tick(Duration::from_millis(80));
    pb.set_message("Creando estructura...");

    for dir in &[
        "src/pages",
        "src/layouts",
        "src/components",
        "src/composables",
        "src/boot",
        "src/stores",
        "src/router",
        "src/css",
        "public",
    ] {
        fs::create_dir_all(root.join(dir)).unwrap();
    }
    pb.finish_and_clear();

    let total = 7u8;
    let ext = cfg.ext();

    // 1. Config raíz
    print::step(1, total, "Configuración del proyecto");
    write(root, "index.html",   &template::index_html(cfg));
    write(root, "package.json", &template::package_json(cfg));
    write(root, &format!("vite.config.{ext}"), &template::vite_config(cfg));
    write(root, ".env.example", &template::env_example(cfg));
    write(root, ".gitignore",   template::gitignore());
    if cfg.is_ts() {
        write(root, "tsconfig.json", template::tsconfig());
    } else {
        write(root, "jsconfig.json", template::jsconfig());
    }
    print::ok(&format!("index.html + package.json + vite.config.{ext}"));

    // 2. App shell
    print::step(2, total, "App shell");
    write(root, "src/App.vue",                    template::app_vue());
    write(root, &format!("src/main.{ext}"),       &template::main_ts(cfg));
    write(root, "src/css/main.css",               template::main_css());
    write(root, "src/css/quasar.variables.scss",  template::quasar_variables());
    print::ok(&format!("App.vue + main.{ext} + CSS"));

    // 3. Boot — axios
    print::step(3, total, "Boot — axios");
    write(root, &format!("src/boot/axios.{ext}"), &template::boot_axios(cfg));
    print::ok(&format!("src/boot/axios.{ext}"));

    // 4. Router
    print::step(4, total, "Router + rutas");
    write(root, &format!("src/router/index.{ext}"),  &template::router_index(cfg));
    write(root, &format!("src/router/routes.{ext}"), &template::router_routes(cfg));
    print::ok("src/router/");

    // 5. Páginas + layouts
    print::step(5, total, "Páginas + layouts");
    write(root, "src/layouts/MainLayout.vue",  &template::layout_main(cfg));
    write(root, "src/pages/Index.vue",         &template::page_index(cfg));
    write(root, "src/pages/ErrorNotFound.vue", template::page_error());
    print::ok("layouts/ + pages/");

    // 6. Componentes + composables + stores
    print::step(6, total, "Componentes + composables + stores");
    write(root, "src/components/AppHeader.vue",              &template::comp_header(cfg));
    write(root, "src/components/AppFooter.vue",              &template::comp_footer(cfg));
    write(root, "src/components/HeroSection.vue",            &template::comp_hero(cfg));
    write(root, &format!("src/composables/useSeo.{ext}"),   &template::composable_use_seo(cfg));
    write(root, &format!("src/stores/app.{ext}"),           &template::store_app(cfg));
    print::ok("components/ + composables/ + stores/");

    // 7. Assets públicos
    print::step(7, total, "Assets públicos");
    let logo: &[u8] = include_bytes!("../assets/oweelogo.png");
    fs::write(root.join("public/oweelogo.png"), logo).unwrap_or_else(|e| {
        eprintln!("{} logo: {e}", "error".red());
    });
    write(root, "public/robots.txt",  &template::robots_txt(cfg));
    write(root, "public/sitemap.xml", &template::sitemap_placeholder(cfg));
    write(root, "public/.htaccess",   template::htaccess());
    if cfg.with_pwa {
        write(root, "public/manifest.json", &template::pwa_manifest(cfg));
    }
    print::ok("public/ — logo + robots.txt + sitemap.xml + .htaccess");
}

fn write(root: &Path, relative: &str, content: impl AsRef<str>) {
    let path = root.join(relative);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(&path, content.as_ref()).unwrap_or_else(|e| {
        eprintln!("{} escribiendo {}: {e}", "error".red(), path.display());
    });
}

pub fn to_title(s: &str) -> String {
    s.split(['-', '_'])
        .map(|w| {
            let mut c = w.chars();
            match c.next() {
                None    => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
