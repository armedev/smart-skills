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
    #[command(about = "Set up global config")]
    Init {
        #[arg(long = "skills-source", default_value = "")]
        source: String,
        #[arg(long = "targets", value_delimiter = ',')]
        targets: Option<Vec<String>>,
        #[arg(long = "force")]
        force: bool,
    },
    #[command(about = "Add skills")]
    Add {
        skills: Vec<String>,
        #[arg(long = "targets", value_delimiter = ',')]
        targets: Option<Vec<String>>,
    },
    #[command(about = "Remove installed skills")]
    Remove {
        skills: Vec<String>,
        #[arg(long = "targets", value_delimiter = ',')]
        targets: Option<Vec<String>>,
    },
    #[command(about = "List available and installed skills")]
    List,
    #[command(about = "Sync skills from sources to targets")]
    Sync {
        #[arg(long = "remove-stale")]
        remove_stale: bool,
        #[arg(long = "targets", value_delimiter = ',')]
        targets: Option<Vec<String>>,
    },
    #[command(about = "Show skill status and validation")]
    Status,
    #[command(about = "Remove all installed skills")]
    Clear {
        #[arg(long = "targets", value_delimiter = ',')]
        targets: Option<Vec<String>>,
    },
    #[command(about = "Display current configuration")]
    Config,
    #[command(about = "Set skill source directories")]
    SetSources {
        paths: Vec<String>,
        #[arg(long, short)]
        overwrite: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Init {
            source,
            targets,
            force,
        } => cli::init(source, targets, force),
        Commands::Add { skills, targets } => cli::add(skills, targets),
        Commands::Remove { skills, targets } => cli::remove(skills, targets),
        Commands::List => cli::list(),
        Commands::Sync {
            remove_stale,
            targets,
        } => cli::sync(remove_stale, targets),
        Commands::Status => cli::status(),
        Commands::Clear { targets } => cli::clear(targets),
        Commands::Config => cli::config_cmd(),
        Commands::SetSources { paths, overwrite } => cli::set_sources(paths, overwrite),
    };

    if let Err(e) = result {
        eprintln!("{}", Colors::error(&e));
        std::process::exit(1);
    }
}
