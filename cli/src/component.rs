use crate::{print, template};
use colored::Colorize;
use std::{fs, path::Path};

pub fn run(name: &str, props_str: Option<&str>) {
    let comp_dir = Path::new("src/components");
    if !comp_dir.exists() {
        eprintln!(
            "{} Ejecuta este comando desde la raíz del proyecto.",
            "error:".red().bold()
        );
        std::process::exit(1);
    }

    let file_name = format!("{name}.vue");
    let comp_path = comp_dir.join(&file_name);

    if comp_path.exists() {
        print::warn(&format!("src/components/{file_name} ya existe — omitiendo"));
        return;
    }

    // Parsea props: "titulo:string,precio:number" → vec de tuplas
    let props: Vec<(&str, &str)> = props_str
        .unwrap_or("")
        .split(',')
        .filter(|s| !s.is_empty())
        .filter_map(|p| {
            let mut parts = p.trim().splitn(2, ':');
            let key = parts.next()?.trim();
            let typ = parts.next().unwrap_or("string").trim();
            Some((key, typ))
        })
        .collect();

    // Necesitamos owned strings para los lifetimes
    let props_owned: Vec<(String, String)> = props
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();
    let props_ref: Vec<(&str, &str)> = props_owned
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();

    let content = template::component_template(name, &props_ref);
    fs::write(&comp_path, content).unwrap_or_else(|e| {
        eprintln!("{} creando componente: {e}", "error".red());
    });

    print::done_component(name);

    if !props_ref.is_empty() {
        println!("  {} Props generadas:", "→".bright_magenta());
        for (k, t) in &props_ref {
            println!("    {} {}: {}", "·".bright_black(), k.bright_white(), t.bright_black());
        }
        println!();
    }
}
