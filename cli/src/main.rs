mod add;
mod component;
mod page;
mod print;
mod scaffold;
mod sitemap;
mod template;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "oweeme",
    about = "Framework Oweeme CLI — Quasar SPA con SEO profesional",
    version = "2.0.0",
    long_about = None
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Crea un nuevo proyecto Quasar SPA
    New {
        /// Nombre del proyecto (también es el nombre del directorio)
        name: String,
    },

    /// Agrega un módulo a un proyecto existente
    Add {
        /// Módulo a agregar: auth | blog | ecommerce | dashboard | rrhh
        module: String,
    },

    /// Genera una nueva página con SEO y ruta
    Page {
        /// Nombre de la página (ej: Productos, BlogDetalle)
        name: String,
        /// Ruta URL (ej: /productos, /blog/:slug)
        #[arg(long, short)]
        route: Option<String>,
        /// Activar guard de autenticación
        #[arg(long)]
        auth: bool,
    },

    /// Genera un componente Vue tipado
    Component {
        /// Nombre del componente (ej: ProductCard, UserAvatar)
        name: String,
        /// Props separadas por coma (ej: titulo:string,precio:number)
        #[arg(long, short)]
        props: Option<String>,
    },

    /// Genera sitemap.xml desde las rutas del proyecto
    Sitemap {
        /// URL base del sitio (ej: https://mitienda.com)
        #[arg(long, short)]
        base: Option<String>,
    },

    /// Muestra información y comandos disponibles
    Info,
}

fn main() {
    let cli = Cli::parse();

    print::banner();

    match cli.command {
        Command::New { name } => {
            scaffold::run(&name);
        }
        Command::Add { module } => {
            add::run(&module);
        }
        Command::Page { name, route, auth } => {
            page::run(&name, route.as_deref(), auth);
        }
        Command::Component { name, props } => {
            component::run(&name, props.as_deref());
        }
        Command::Sitemap { base } => {
            sitemap::run(base.as_deref());
        }
        Command::Info => {
            print::info();
        }
    }
}
