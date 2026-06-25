use crate::{print, template};
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect, Select};
use indicatif::{ProgressBar, ProgressStyle};
use std::{fs, path::Path, time::Duration};

#[derive(Debug, Clone, PartialEq)]
pub enum Lang { TypeScript, JavaScript }

#[derive(Debug, Clone, PartialEq)]
pub enum Linter { EslintOxlint, None }

pub struct ProjectConfig {
    pub name:        String,
    pub site_name:   String,
    pub site_url:    String,
    pub lang:        Lang,
    pub with_pwa:    bool,
    pub linter:      Linter,
    pub icon_sets:   Vec<IconSet>,
    pub qplugins:    Vec<QuasarPlugin>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IconSet {
    MaterialIcons,
    MdiV7,
    FontAwesomeV6,
    EvaIcons,
    LineAwesome,
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuasarPlugin {
    Notify,
    Dialog,
    Loading,
    BottomSheet,
    AppFullscreen,
    Dark,
    LocalStorage,
}

impl IconSet {
    pub fn package(&self) -> &'static str {
        match self {
            IconSet::MaterialIcons  => "@quasar/extras",
            IconSet::MdiV7          => "@quasar/extras",
            IconSet::FontAwesomeV6  => "@quasar/extras",
            IconSet::EvaIcons       => "@quasar/extras",
            IconSet::LineAwesome    => "@quasar/extras",
        }
    }
    pub fn import(&self) -> &'static str {
        match self {
            IconSet::MaterialIcons => "import '@quasar/extras/material-icons/material-icons.css'",
            IconSet::MdiV7         => "import '@quasar/extras/mdi-v7/mdi-v7.css'",
            IconSet::FontAwesomeV6 => "import '@quasar/extras/fontawesome-v6/fontawesome-v6.css'",
            IconSet::EvaIcons      => "import '@quasar/extras/eva-icons/eva-icons.css'",
            IconSet::LineAwesome   => "import '@quasar/extras/line-awesome/line-awesome.css'",
        }
    }
    pub fn label(&self) -> &'static str {
        match self {
            IconSet::MaterialIcons => "material-icons  (Google — recomendado)",
            IconSet::MdiV7         => "mdi-v7          (Material Design Icons — 7000+ íconos)",
            IconSet::FontAwesomeV6 => "fontawesome-v6  (Font Awesome — muy popular)",
            IconSet::EvaIcons      => "eva-icons        (Eva — minimalista)",
            IconSet::LineAwesome   => "line-awesome     (alternativa a Font Awesome)",
        }
    }
}

impl QuasarPlugin {
    pub fn name(&self) -> &'static str {
        match self {
            QuasarPlugin::Notify       => "Notify",
            QuasarPlugin::Dialog       => "Dialog",
            QuasarPlugin::Loading      => "Loading",
            QuasarPlugin::BottomSheet  => "BottomSheet",
            QuasarPlugin::AppFullscreen=> "AppFullscreen",
            QuasarPlugin::Dark         => "Dark",
            QuasarPlugin::LocalStorage => "LocalStorage",
        }
    }
    pub fn label(&self) -> &'static str {
        match self {
            QuasarPlugin::Notify       => "Notify        — notificaciones toast",
            QuasarPlugin::Dialog       => "Dialog        — diálogos modales",
            QuasarPlugin::Loading      => "Loading       — overlay de carga",
            QuasarPlugin::BottomSheet  => "BottomSheet   — menú inferior (mobile)",
            QuasarPlugin::AppFullscreen=> "AppFullscreen — pantalla completa",
            QuasarPlugin::Dark         => "Dark          — modo oscuro programático",
            QuasarPlugin::LocalStorage => "LocalStorage  — wrapper tipado de localStorage",
        }
    }
}

impl ProjectConfig {
    pub fn is_ts(&self) -> bool { self.lang == Lang::TypeScript }
    pub fn ext(&self) -> &'static str { if self.is_ts() { "ts" } else { "js" } }
    pub fn with_linting(&self) -> bool { self.linter == Linter::EslintOxlint }
}

pub fn run(name: &str) {
    let theme = ColorfulTheme::default();

    println!("  {}", "Configuremos tu proyecto Quasar SPA".bold());
    println!();

    // ── Nombre y URL ──────────────────────────────────────────────────────────
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

    // ── Lenguaje ──────────────────────────────────────────────────────────────
    let lang_idx = Select::with_theme(&theme)
        .with_prompt("Lenguaje")
        .items(&[
            "TypeScript  (recomendado — tipado, autocompletado)",
            "JavaScript  (sin tipos)",
        ])
        .default(0)
        .interact()
        .unwrap();
    let lang = if lang_idx == 0 { Lang::TypeScript } else { Lang::JavaScript };

    // ── Icon sets (multi) ─────────────────────────────────────────────────────
    let all_icons = vec![
        IconSet::MaterialIcons,
        IconSet::MdiV7,
        IconSet::FontAwesomeV6,
        IconSet::EvaIcons,
        IconSet::LineAwesome,
    ];
    let icon_labels: Vec<&str> = all_icons.iter().map(|i| i.label()).collect();
    let icon_sel = MultiSelect::with_theme(&theme)
        .with_prompt("Icon sets  (espacio = seleccionar, enter = confirmar)")
        .items(&icon_labels)
        .defaults(&[true, false, false, false, false])
        .interact()
        .unwrap();
    let mut icon_sets: Vec<IconSet> = icon_sel.iter().map(|&i| all_icons[i].clone()).collect();
    if icon_sets.is_empty() {
        icon_sets.push(IconSet::MaterialIcons); // mínimo uno
    }

    // ── Quasar plugins (multi) ────────────────────────────────────────────────
    let all_plugins = vec![
        QuasarPlugin::Notify,
        QuasarPlugin::Dialog,
        QuasarPlugin::Loading,
        QuasarPlugin::BottomSheet,
        QuasarPlugin::AppFullscreen,
        QuasarPlugin::Dark,
        QuasarPlugin::LocalStorage,
    ];
    let plugin_labels: Vec<&str> = all_plugins.iter().map(|p| p.label()).collect();
    let plugin_sel = MultiSelect::with_theme(&theme)
        .with_prompt("Quasar plugins  (espacio = seleccionar, enter = confirmar)")
        .items(&plugin_labels)
        .defaults(&[true, true, true, false, false, false, false])
        .interact()
        .unwrap();
    let qplugins: Vec<QuasarPlugin> = plugin_sel.iter().map(|&i| all_plugins[i].clone()).collect();

    // ── PWA ───────────────────────────────────────────────────────────────────
    let with_pwa = Confirm::with_theme(&theme)
        .with_prompt("¿Habilitar PWA (manifest + service worker + offline)?")
        .default(true)
        .interact()
        .unwrap();

    // ── Linting ───────────────────────────────────────────────────────────────
    let lint_idx = Select::with_theme(&theme)
        .with_prompt("Linting & formato")
        .items(&[
            "ESLint 9 + oxlint  (recomendado — rápido, Vue 3 + TS)",
            "Ninguno",
        ])
        .default(0)
        .interact()
        .unwrap();
    let linter = if lint_idx == 0 { Linter::EslintOxlint } else { Linter::None };

    println!();

    let config = ProjectConfig {
        name: name.to_string(),
        site_name,
        site_url,
        lang,
        with_pwa,
        linter,
        icon_sets,
        qplugins,
    };

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
        "src/pages", "src/layouts", "src/components",
        "src/composables", "src/boot", "src/stores",
        "src/router", "src/css", "public",
    ] {
        fs::create_dir_all(root.join(dir)).unwrap();
    }
    pb.finish_and_clear();

    let total = 8u8;
    let ext   = cfg.ext();

    // 1. Config raíz
    print::step(1, total, "Configuración del proyecto");
    write(root, "index.html",   &template::index_html(cfg));
    write(root, "package.json", &template::package_json(cfg));
    write(root, &format!("vite.config.{ext}"), &template::vite_config(cfg));
    write(root, &format!("oweeme.config.{ext}"), &template::oweeme_config(cfg));
    write(root, ".env.example", &template::env_example(cfg));
    write(root, ".gitignore",   template::gitignore());
    if cfg.is_ts() {
        write(root, "tsconfig.json", template::tsconfig());
    } else {
        write(root, "jsconfig.json", template::jsconfig());
    }
    if cfg.with_linting() {
        write(root, "eslint.config.js",  &template::eslint_config(cfg));
        write(root, ".oxlintrc.json",     template::oxlint_config());
        write(root, ".prettierrc",        template::prettier_config());
    }
    print::ok(&format!("index.html + package.json + oweeme.config.{ext} + vite.config.{ext}"));

    // 2. App shell
    print::step(2, total, "App shell");
    write(root, "src/App.vue",                   template::app_vue());
    write(root, &format!("src/main.{ext}"),      &template::main_ts(cfg));
    write(root, "src/css/main.css",              template::main_css());
    write(root, "src/css/quasar.variables.scss", template::quasar_variables());
    print::ok(&format!("App.vue + main.{ext} + CSS"));

    // 3. Boot
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
    write(root, "src/components/AppHeader.vue",            &template::comp_header(cfg));
    write(root, "src/components/AppFooter.vue",            &template::comp_footer(cfg));
    write(root, "src/components/HeroSection.vue",          &template::comp_hero(cfg));
    write(root, &format!("src/composables/useSeo.{ext}"), &template::composable_use_seo(cfg));
    write(root, &format!("src/stores/app.{ext}"),         &template::store_app(cfg));
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

    // 8. Linting
    if cfg.with_linting() {
        print::step(8, total, "Linting — ESLint 9 + oxlint");
        print::ok(".eslintrc + .oxlintrc.json + .prettierrc");
    } else {
        print::step(8, total, "Linting omitido");
        print::ok("sin linter configurado");
    }
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
