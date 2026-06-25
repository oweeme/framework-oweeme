use crate::print;
use colored::Colorize;
use std::{fs, path::Path};

pub fn run(base_url: Option<&str>) {
    // Detecta base URL: argumento > .env > fallback
    let base = base_url
        .map(String::from)
        .or_else(read_env_site_url)
        .unwrap_or_else(|| {
            eprintln!(
                "{} Proporciona la URL base: oweeme sitemap --base https://tudominio.com",
                "error:".red().bold()
            );
            std::process::exit(1);
        });
    let base = base.trim_end_matches('/');

    let routes_path = Path::new("src/router/routes.ts");
    if !routes_path.exists() {
        eprintln!(
            "{} No se encontró src/router/routes.ts. Ejecuta desde la raíz del proyecto.",
            "error:".red().bold()
        );
        std::process::exit(1);
    }

    let routes_content = fs::read_to_string(routes_path).unwrap_or_default();
    let urls = extract_static_routes(&routes_content);

    let today = chrono_today();
    let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    xml.push_str("<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n");

    for url in &urls {
        let priority = if url == "/" { "1.0" } else { "0.8" };
        xml.push_str(&format!(
            "  <url>\n    <loc>{base}{url}</loc>\n    <lastmod>{today}</lastmod>\n    <changefreq>weekly</changefreq>\n    <priority>{priority}</priority>\n  </url>\n"
        ));
    }
    xml.push_str("</urlset>\n");

    let out = Path::new("public/sitemap.xml");
    fs::write(out, &xml).unwrap_or_else(|e| {
        eprintln!("{} escribiendo sitemap.xml: {e}", "error".red());
        std::process::exit(1);
    });

    print::done_sitemap(urls.len(), "public/sitemap.xml");
}

/// Extrae rutas estáticas (sin parámetros :param) del archivo routes.ts
fn extract_static_routes(content: &str) -> Vec<String> {
    let mut routes = Vec::new();

    for line in content.lines() {
        // Busca líneas con path: '...' o path: "..."
        if let Some(start) = line.find("path:") {
            let after = &line[start + 5..];
            if let Some(route) = extract_quoted(after) {
                // Filtra rutas dinámicas (:param) y catch-all
                if !route.contains(':') && !route.contains('*') && !route.is_empty() {
                    if !routes.contains(&route) {
                        routes.push(route);
                    }
                }
            }
        }
    }

    // Asegura que / siempre esté primero
    if !routes.contains(&"/".to_string()) {
        routes.insert(0, "/".to_string());
    } else {
        routes.retain(|r| r != "/");
        routes.insert(0, "/".to_string());
    }

    routes
}

fn extract_quoted(s: &str) -> Option<String> {
    let s = s.trim();
    let (_open, close) = if s.starts_with('\'') { ('\'', '\'') }
    else if s.starts_with('"') { ('"', '"') }
    else { return None };

    let inner = &s[1..];
    let end = inner.find(close)?;
    Some(inner[..end].to_string())
}

fn read_env_site_url() -> Option<String> {
    let content = fs::read_to_string(".env").ok()?;
    for line in content.lines() {
        if let Some(val) = line.strip_prefix("VITE_SITE_URL=") {
            return Some(val.trim().to_string());
        }
    }
    None
}

fn chrono_today() -> String {
    // Simple date without external dependency
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    // Days since epoch
    let days  = secs / 86400;
    let years = 1970 + days / 365;
    let rem   = days % 365;
    let month = (rem / 30) + 1;
    let day   = (rem % 30) + 1;
    format!("{years}-{month:02}-{day:02}")
}
