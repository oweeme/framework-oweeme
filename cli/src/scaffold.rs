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
    pub cache_ttl: u64,
    pub with_vue: bool,
    pub with_pwa: bool,
}

pub fn run(name: &str) {
    let theme = ColorfulTheme::default();

    println!("  {}", "Let's set up your project".bold());
    println!();

    // Preguntas interactivas
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
        .with_prompt("API backend URL")
        .default("http://localhost:8080".into())
        .interact_text()
        .unwrap();

    let langs = &["es — Español", "en — English", "pt — Português", "de — Deutsch", "fr — Français", "ru — Русский", "ko — 한국어", "ja — 日本語"];
    let lang_codes = &["es", "en", "pt", "de", "fr", "ru", "ko", "ja"];
    let lang_idx = Select::with_theme(&theme)
        .with_prompt("Default language")
        .default(0)
        .items(langs)
        .interact()
        .unwrap();
    let default_lang = lang_codes[lang_idx].to_string();

    let cache_opts = &["60s (aggressive)", "300s (recommended)", "600s (slow APIs)", "0 (disabled)"];
    let cache_vals = &[60u64, 300, 600, 0];
    let cache_idx = Select::with_theme(&theme)
        .with_prompt("API cache TTL")
        .default(1)
        .items(cache_opts)
        .interact()
        .unwrap();
    let cache_ttl = cache_vals[cache_idx];

    let with_vue = Confirm::with_theme(&theme)
        .with_prompt("Include Vue.js frontend?")
        .default(true)
        .interact()
        .unwrap();

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
        cache_ttl,
        with_vue,
        with_pwa,
    };

    generate_project(&config);
    print::done(name, &api_url);
}

fn generate_project(cfg: &ProjectConfig) {
    let root = Path::new(&cfg.name);
    let total = if cfg.with_vue { 7u8 } else { 6u8 };

    // Spinner
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner:.magenta} {msg}")
            .unwrap(),
    );
    pb.enable_steady_tick(Duration::from_millis(80));

    // 1. Directorios
    pb.set_message("Creating project structure...");
    for dir in &[
        "src", "templates/pages", "templates/components",
        "static/css", "static/js", "static/icons", "locales",
    ] {
        fs::create_dir_all(root.join(dir)).unwrap();
    }
    pb.finish_and_clear();

    print::step(1, total, "Project structure");
    print::ok("directories created");

    // 2. Cargo.toml del proyecto
    print::step(2, total, "Cargo.toml");
    write(root, "Cargo.toml", &template::cargo_toml(&cfg.name));
    print::ok("dependencies configured");

    // 3. Código fuente Rust
    print::step(3, total, "Rust source");
    write(root, "src/main.rs", template::main_rs());
    print::ok("src/main.rs");

    // 4. Plantillas HTML
    print::step(4, total, "HTML templates");
    write(root, "templates/base.html", template::base_html(&cfg.site_name, cfg.with_pwa));
    write(root, "templates/pages/home.html", template::page_home());
    write(root, "templates/pages/articulo.html", template::page_articulo());
    write(root, "templates/pages/musica.html", template::page_musica());
    write(root, "templates/pages/trabajo.html", template::page_trabajo());
    write(root, "templates/pages/404.html", template::page_404());
    write(root, "templates/components/Card.html", template::component_card());
    write(root, "templates/components/Hero.html", template::component_hero());
    write(root, "templates/components/Navbar.html", template::component_navbar(&cfg.site_name));
    write(root, "templates/components/Footer.html", template::component_footer(&cfg.site_name));
    print::ok("base.html + pages + components");

    // 5. Assets estáticos
    print::step(5, total, "Static assets (CSS + JS)");
    write(root, "static/css/app.css", template::css_app());
    write(root, "static/js/app.js", template::js_app());
    write(root, "static/js/chat.js", template::js_chat());
    print::ok("CSS design system + JS");

    // 6. Config & locales
    print::step(6, total, "Config & i18n");
    write(root, ".env.example", &template::env_example(cfg));
    write(root, ".gitignore", template::gitignore());
    for (code, json) in template::all_locales() {
        write(root, &format!("locales/{code}.json"), &json);
    }
    print::ok(".env + 8 locales (es/en/pt/de/fr/ru/ko/ja)");

    // 7. Frontend Vue (opcional)
    if cfg.with_vue {
        print::step(7, total, "Vue.js frontend");
        fs::create_dir_all(root.join("frontend/src/components")).unwrap();
        fs::create_dir_all(root.join("frontend/src/composables")).unwrap();
        fs::create_dir_all(root.join("frontend/src/pages")).unwrap();
        write(root, "frontend/package.json", &template::vue_package_json(&cfg.name));
        write(root, "frontend/vite.config.js", template::vue_vite_config());
        write(root, "frontend/index.html", template::vue_index_html(&cfg.site_name));
        write(root, "frontend/src/main.js", template::vue_main_js());
        write(root, "frontend/src/App.vue", template::vue_app());
        write(root, "frontend/src/composables/useOweeme.js", template::vue_composable_oweeme());
        write(root, "frontend/src/composables/useChat.js", template::vue_composable_chat());
        write(root, "frontend/src/components/ChatBox.vue", template::vue_chat_component());
        write(root, "frontend/src/pages/Home.vue", template::vue_page_home());
        write(root, "frontend/.gitignore", "node_modules/\ndist/\n");
        print::ok("Vue 3 + Vite + Quasar + composables");
    }
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
