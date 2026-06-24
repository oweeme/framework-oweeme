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
        "SEO-first Rust + Vue.js Framework".bold(),
        "•".bright_black(),
        "v1.0.0".bright_black()
    );
    println!();
}

pub fn info() {
    println!("{}", "Commands".bold().underline());
    println!();
    cmd("oweeme new <name>", "Scaffold a new project");
    println!();
    println!("{}", "After creating a project:".bright_black());
    println!();
    cmd("cd <name>", "Enter the project directory");
    cmd("cp .env.example .env", "Set up environment");
    cmd("cargo run", "Start the dev server");
    println!();
    println!("{}", "Docs & source:".bright_black());
    println!("  {}", "https://github.com/oweeme/framework-oweeme".cyan());
    println!();
    println!("{}", "Author:".bright_black());
    println!(
        "  {}  {}",
        "Héctor Martínez".bold().white(),
        "—".bright_black()
    );
    println!("  {}", "oweeme.com".cyan());
    println!();
}

fn cmd(command: &str, desc: &str) {
    println!(
        "  {}  {}",
        format!("  {command}  ").on_bright_black().white().bold(),
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

pub fn done(project: &str, api_url: &str) {
    println!();
    println!("  {}", "─".repeat(54).bright_black());
    println!();
    println!(
        "  {} {}",
        "Project ready:".bold(),
        project.bright_magenta().bold()
    );
    println!();
    println!("  {}", "Next steps:".bold());
    println!();
    println!("  {}  {}", "1.".bright_magenta(), format!("cd {project}").bright_white());
    println!("  {}  {}", "2.".bright_magenta(), "cp .env.example .env  # edit API URL".bright_white());
    println!("  {}  {}", "3.".bright_magenta(), "npm install".bright_white());
    println!("  {}  {}", "4.".bright_magenta(), "npm run dev".bright_white());
    println!();
    println!("  {}", format!("Dev server → http://localhost:3000").bright_black());
    println!();
    println!("  {}", "Build for production:".bold());
    println!("  {}  {}", "→".bright_magenta(), "npm run generate  # outputs dist/ — upload anywhere".bright_white());
    println!();
    println!("  {}", format!("API backend: {api_url}").bright_black());
    println!();
}
