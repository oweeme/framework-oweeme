use crate::{print, template};
use colored::Colorize;
use std::{fs, path::Path};

pub fn run(name: &str, route: Option<&str>, with_auth: bool) {
    let src = Path::new("src/pages");
    if !src.exists() {
        eprintln!(
            "{} Ejecuta este comando desde la raíz del proyecto.",
            "error:".red().bold()
        );
        std::process::exit(1);
    }

    let file_name  = format!("{name}.vue");
    let page_path  = src.join(&file_name);
    let default_route = format!("/{}", name.to_lowercase());
    let route_str  = route.unwrap_or(&default_route);

    if page_path.exists() {
        print::warn(&format!("src/pages/{file_name} ya existe — omitiendo"));
        return;
    }

    let content = template::page_template(name, route_str, with_auth);
    fs::write(&page_path, content).unwrap_or_else(|e| {
        eprintln!("{} creando página: {e}", "error".red());
    });

    // Append route hint to routes.ts if it exists
    let routes_path = Path::new("src/router/routes.ts");
    if routes_path.exists() {
        let hint = format!(
            "\n// TODO: agrega esta ruta en el array routes:\n// {{ path: '{route_str}', name: '{name_lower}', component: () => import('@/pages/{file_name}') }},\n",
            route_str  = route_str,
            name_lower = name.to_lowercase(),
            file_name  = file_name,
        );
        let mut content = fs::read_to_string(routes_path).unwrap_or_default();
        content.push_str(&hint);
        fs::write(routes_path, content).ok();
    }

    print::done_page(name, route_str);
}
