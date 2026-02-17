use clap::{Parser, Subcommand};

mod commands;
mod component;
mod css;
mod install;
mod project;
mod utils;

/// CLI tool for leptos-daisyui-rs - simplify component setup and management
#[derive(Parser)]
#[command(name = "leptos-daisyui")]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize leptos-daisyui-rs in your project
    Init,

    /// List available components
    List {
        /// Show all components (default shows only installed)
        #[arg(short, long)]
        all: bool,

        /// Filter by category
        #[arg(short, long)]
        category: Option<String>,
    },

    /// Add components to your project
    Add {
        /// Component names to add
        #[arg(required = true)]
        components: Vec<String>,

        /// Overwrite existing components
        #[arg(short, long)]
        force: bool,

        /// Add interactively with prompts
        #[arg(short, long)]
        interactive: bool,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => {
            commands::init::execute()?;
        }
        Commands::List { all, category } => {
            commands::list::execute(all, category)?;
        }
        Commands::Add {
            components,
            force,
            interactive,
        } => {
            commands::add::execute(components, force, interactive)?;
        }
    }

    Ok(())
}
