mod cli;
mod config;
mod skills;

use clap::{Parser, Subcommand};
use cli::Colors;

#[derive(Parser)]
#[command(name = "smart-skills")]
#[command(about = "Agent skill management tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Initialize project with skill sources and targets")]
    Init {
        #[arg(long = "skills-source", default_value = "")]
        source: String,
        #[arg(long = "targets", value_delimiter = ',')]
        targets: Option<Vec<String>>,
    },
    #[command(about = "Add skills to your project")]
    Add { skills: Vec<String> },
    #[command(about = "Remove installed skills")]
    Remove { skills: Vec<String> },
    #[command(about = "List available and installed skills")]
    List,
    #[command(about = "Sync skills from sources to targets")]
    Sync {
        #[arg(long = "remove-stale")]
        remove_stale: bool,
    },
    #[command(about = "Show skill status and validation")]
    Status,
    #[command(about = "Remove all installed skills")]
    Clear,
    #[command(about = "Display current configuration")]
    Config,
    #[command(about = "Set skill source directories")]
    SetSources { paths: Vec<String> },
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
        eprintln!("{}", Colors::error(&e));
        std::process::exit(1);
    }
}
