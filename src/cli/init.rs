use crate::cli::Colors;
use crate::config::{self, Config, InstallTargets, SkillSource, DEFAULT_PRIORITY};
use std::fs;

pub fn init(source: String, targets: Option<Vec<String>>, force: bool) -> Result<(), String> {
    let path = config::global_config_path();

    if path.exists() && !force {
        println!("{}", Colors::warning("Global config already exists"));
        println!("  Use --force to overwrite");
        return Ok(());
    }

    let targets = targets.unwrap_or_else(|| vec!["agents".to_string()]);

    if source.is_empty() && targets == vec!["agents".to_string()] {
        println!("{}", Colors::dim("No arguments provided. Using defaults:"));
        println!("  {}: (none)", Colors::dim("Skill source"));
        println!("  {}: agents", Colors::dim("Targets"));
        println!();
    }

    fs::create_dir_all(config::global_config_dir()).map_err(|e| e.to_string())?;

    let config = Config {
        skill_sources: if source.is_empty() {
            vec![]
        } else {
            vec![SkillSource {
                path: config::resolve_path(&source).display().to_string(),
                priority: DEFAULT_PRIORITY,
            }]
        },
        install_targets: InstallTargets {
            agents: targets.contains(&"agents".to_string()),
            cursor: targets.contains(&"cursor".to_string()),
            claude: targets.contains(&"claude".to_string()),
        },
    };

    config.save(&path)?;

    println!("{}", Colors::dim(&path.display().to_string()));
    if config.skill_sources.is_empty() {
        println!("  {}", Colors::warning("No skill source set"));
    } else {
        println!(
            "  {}: {}",
            Colors::dim("Skill source"),
            Colors::skill(&config.skill_sources[0].path)
        );
    }
    println!(
        "  {}: agents={}, cursor={}, claude={}",
        Colors::dim("Targets"),
        config.install_targets.agents,
        config.install_targets.cursor,
        config.install_targets.claude
    );

    println!("\n{}", Colors::success("Global config created."));
    println!(
        "  {}: {}",
        Colors::dim("Set source with"),
        Colors::skill("smart-skills set-sources <path>...")
    );
    println!(
        "  {}: {}",
        Colors::dim("Add skills with"),
        Colors::skill("smart-skills add <skill>")
    );

    Ok(())
}
