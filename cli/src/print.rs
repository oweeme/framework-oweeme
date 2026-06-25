use colored::Colorize;

pub fn banner() {
    println!();
    println!("{}", "  ██████╗ ██╗    ██╗███████╗███████╗███╗   ███╗███████╗".bright_magenta());
    println!("{}", "  ██╔═══██╗██║    ██║██╔════╝██╔════╝████╗ ████║██╔════╝".bright_magenta());
    println!("{}", "  ██║   ██║██║ █╗ ██║█████╗  █████╗  ██╔████╔██║█████╗  ".bright_magenta());
    println!("{}", "  ██║   ██║██║███╗██║██╔══╝  ██╔══╝  ██║╚██╔╝██║██╔══╝  ".magenta());
    println!("{}", "  ╚██████╔╝╚███╔███╔╝███████╗███████╗██║ ╚═╝ ██║███████╗".magenta());
    println!("{}", "   ╚═════╝  ╚══╝╚══╝ ╚══════╝╚══════╝╚═╝     ╚═╝╚══════╝".magenta());
    println!();
    println!(
        "  {} {}  {}",
        "Quasar SPA Framework".bold(),
        "•".bright_black(),
        "v2.0.0".bright_black()
    );
    println!();
}

pub fn info() {
    println!("{}", "Comandos disponibles".bold().underline());
    println!();

    cmd("oweeme new <nombre>",      "Crea un nuevo proyecto Quasar SPA");
    cmd("oweeme add <módulo>",      "Agrega módulo al proyecto actual");
    cmd("oweeme page <nombre>",     "Genera una página con SEO y ruta");
    cmd("oweeme component <nombre>","Genera un componente Vue tipado");
    cmd("oweeme sitemap",           "Genera sitemap.xml desde las rutas");
    cmd("oweeme info",              "Muestra esta ayuda");

    println!();
    println!("{}", "Módulos disponibles (oweeme add):".bright_black());
    println!();
    module("auth",       "Login + Register + Profile + Pinia auth store");
    module("blog",       "Páginas blog + BlogCard + composable");
    module("ecommerce",  "Productos + Carrito + Checkout + Pinia cart");
    module("dashboard",  "Layout admin + Sidebar + StatsCard");
    module("rrhh",       "Empleados + Detalle + Pinia rrhh store");
    module("capacitor",  "Android + iOS — capacitor.config.ts + script build");

    println!();
    println!("{}", "Docs & fuente:".bright_black());
    println!("  {}", "https://github.com/oweeme/framework-oweeme".cyan());
    println!();
    println!("{}", "Autor:".bright_black());
    println!("  {}  —  {}", "Héctor Martínez".bold().white(), "oweeme.com".cyan());
    println!();
}

fn cmd(command: &str, desc: &str) {
    println!(
        "  {}  {}",
        format!(" {command} ").on_bright_black().white().bold(),
        desc.bright_black()
    );
}

fn module(name: &str, desc: &str) {
    println!(
        "  {} {}  {}",
        "→".bright_magenta(),
        name.bold().white(),
        desc.bright_black()
    );
}

pub fn step(n: u8, total: u8, msg: &str) {
    println!(
        "  {} {}",
        format!("[{n}/{total}]").bright_magenta().bold(),
        msg.bold()
    );
}

pub fn ok(msg: &str) {
    println!("  {}  {}", "✓".bright_green().bold(), msg);
}

pub fn warn(msg: &str) {
    println!("  {}  {}", "!".yellow().bold(), msg.yellow());
}

pub fn done_new(project: &str) {
    println!();
    println!("  {}", "─".repeat(54).bright_black());
    println!();
    println!(
        "  {} {}",
        "Proyecto listo:".bold(),
        project.bright_magenta().bold()
    );
    println!();
    println!("  {}", "Pasos siguientes:".bold());
    println!();
    println!("  {}  {}", "1.".bright_magenta(), format!("cd {project}").bright_white());
    println!("  {}  {}", "2.".bright_magenta(), "cp .env.example .env".bright_white());
    println!("  {}  {}", "3.".bright_magenta(), "npm install".bright_white());
    println!("  {}  {}", "4.".bright_magenta(), "npm run dev".bright_white());
    println!();
    println!("  {}", "Dev →  http://localhost:5173".bright_black());
    println!();
    println!("  {}", "Producción:".bold());
    println!("  {}  {}", "→".bright_magenta(), "npm run build   # genera dist/ listo para subir".bright_white());
    println!("  {}  {}", "→".bright_magenta(), "oweeme sitemap  # genera sitemap.xml".bright_white());
    println!();
}

pub fn done_add(module: &str) {
    println!();
    println!("  {}  Módulo {} agregado", "✓".bright_green().bold(), module.bright_magenta().bold());
    println!();
    println!("  {}", "Recuerda registrar las rutas nuevas en src/router/routes.ts".bright_black());
    println!();
}

pub fn done_page(name: &str, route: &str) {
    println!();
    println!(
        "  {}  Página {} creada — ruta: {}",
        "✓".bright_green().bold(),
        name.bright_magenta().bold(),
        route.bright_white()
    );
    println!("  {}", "Agrega la ruta en src/router/routes.ts si no fue automático".bright_black());
    println!();
}

pub fn done_component(name: &str) {
    println!();
    println!(
        "  {}  Componente {} creado en src/components/",
        "✓".bright_green().bold(),
        name.bright_magenta().bold()
    );
    println!();
}

pub fn done_sitemap(count: usize, path: &str) {
    println!();
    println!(
        "  {}  sitemap.xml generado — {} rutas → {}",
        "✓".bright_green().bold(),
        count.to_string().bright_white(),
        path.bright_white()
    );
    println!();
}
