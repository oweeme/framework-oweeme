mod scaffold;
mod template;
mod print;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "oweeme",
    about = "Framework Oweeme CLI — SEO-first Rust + Vue.js projects",
    version = "1.0.0",
    long_about = None
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Create a new Oweeme project
    New {
        /// Project name (also used as directory name)
        name: String,
    },
    /// Show framework info and available commands
    Info,
}

fn main() {
    let cli = Cli::parse();

    print::banner();

    match cli.command {
        Command::New { name } => {
            scaffold::run(&name);
        }
        Command::Info => {
            print::info();
        }
    }
}
