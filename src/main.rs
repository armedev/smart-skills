mod cli;
mod config;
mod skills;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "smart-skills")]
#[command(about = "Agent skill management tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init {
        #[arg(long = "skills-source", default_value = "")]
        source: String,
        #[arg(long = "targets", value_delimiter = ',')]
        targets: Option<Vec<String>>,
    },
    Add {
        skills: Vec<String>,
    },
    Remove {
        skills: Vec<String>,
    },
    List,
    Sync {
        #[arg(long = "remove-stale")]
        remove_stale: bool,
    },
    Status,
    Clear,
    Config,
    SetSources {
        #[arg(default_value = "")]
        paths: Vec<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Init { source, targets } => cli::init(source, targets),
        Commands::Add { skills } => cli::add(skills),
        Commands::Remove { skills } => cli::remove(skills),
        Commands::List => cli::list(),
        Commands::Sync { remove_stale } => cli::sync(remove_stale),
        Commands::Status => cli::status(),
        Commands::Clear => cli::clear(),
        Commands::Config => cli::config_cmd(),
        Commands::SetSources { paths } => cli::set_sources(paths),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
