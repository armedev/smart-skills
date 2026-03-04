use crate::cli::Colors;
use crate::config::{self, Config, SkillSource};
use crate::skills::installer::SkillInstaller;
use crate::skills::loader::SkillLoader;
use std::fs;

fn is_initialized() -> bool {
    config::project_config_path().exists()
}

fn ensure_initialized() -> Result<(), String> {
    if !is_initialized() {
        return Err("Not initialized. Run 'smart-skills init' first.".to_string());
    }
    Ok(())
}

pub fn add(skill_names: Vec<String>) -> Result<(), String> {
    ensure_initialized()?;

    let available = SkillLoader::load_available_skills();
    let installed = SkillLoader::load_installed_skills();

    if skill_names.is_empty() {
        println!(
            "{} ({})",
            Colors::header("Available skills"),
            available.len()
        );
        for (name, skill) in &available {
            let status = if installed.contains(name) {
                Colors::success("[installed]")
            } else {
                Colors::dim("")
            };
            println!("  - {} {}", Colors::skill(name), status);
            if !skill.description.is_empty() {
                println!("    {}", Colors::dim(&skill.description));
            }
        }
        println!();
        println!(
            "To install: {}",
            Colors::skill("smart-skills add <skill-name>")
        );
        return Ok(());
    }

    for name in skill_names {
        if let Some(skill) = available.get(&name) {
            SkillInstaller::install(skill)?;
            println!(
                "  {}: {}",
                Colors::success("Installed"),
                Colors::skill(&name)
            );
        } else {
            println!(
                "  {}: skill '{}' not found",
                Colors::error("Error"),
                Colors::skill(&name)
            );
        }
    }

    Ok(())
}

pub fn remove(skill_names: Vec<String>) -> Result<(), String> {
    ensure_initialized()?;

    if skill_names.is_empty() {
        let installed = SkillLoader::load_installed_skills();
        println!(
            "{} ({})",
            Colors::header("Installed skills"),
            installed.len()
        );
        if installed.is_empty() {
            println!("  {}", Colors::dim("No skills installed"));
        } else {
            for name in installed {
                println!("  - {}", Colors::skill(&name));
            }
        }
        return Ok(());
    }

    for name in &skill_names {
        SkillInstaller::remove(name)?;
        println!("  {}: {}", Colors::success("Removed"), Colors::skill(name));
    }

    Ok(())
}

pub fn list() -> Result<(), String> {
    ensure_initialized()?;

    let available = SkillLoader::load_available_skills();
    let installed = SkillLoader::load_installed_skills();

    println!(
        "{} ({})",
        Colors::header("Available skills"),
        available.len()
    );
    for (name, skill) in &available {
        let status = if installed.contains(name) {
            Colors::success("[installed]")
        } else {
            Colors::dim("")
        };
        println!("  - {} {}", Colors::skill(name), status);
        if !skill.description.is_empty() {
            println!("    {}", Colors::dim(&skill.description));
        }
    }

    println!(
        "\n{} ({})",
        Colors::header("Installed skills"),
        installed.len()
    );
    for name in &installed {
        println!("  - {}", Colors::skill(name));
    }

    Ok(())
}

pub fn sync(remove_stale: bool) -> Result<(), String> {
    ensure_initialized()?;

    println!("{}...", Colors::header("Syncing skills"));

    let available = SkillLoader::load_available_skills();
    let installed = SkillLoader::load_installed_skills();

    if installed.is_empty() && !remove_stale {
        println!("  {}", Colors::dim("Nothing to sync"));
        println!("{}", Colors::success("Done!"));
        return Ok(());
    }

    let mut action_count = 0;

    if remove_stale {
        for name in &installed {
            if !available.contains_key(name) {
                SkillInstaller::remove(name)?;
                println!(
                    "  {} stale: {}",
                    Colors::success("Removed"),
                    Colors::skill(name)
                );
                action_count += 1;
            }
        }
        if action_count > 0 {
            println!(
                "{} {} stale skill(s)",
                Colors::success("Removed"),
                action_count
            );
        }
    }

    for name in &installed {
        if let Some(skill) = available.get(name) {
            SkillInstaller::install(skill)?;
            println!("  {}: {}", Colors::success("Synced"), Colors::skill(name));
            action_count += 1;
        }
    }

    if action_count == 0 {
        println!("  {}", Colors::dim("Nothing to sync"));
    }

    println!("{}", Colors::success("Done!"));

    Ok(())
}

pub fn status() -> Result<(), String> {
    ensure_initialized()?;

    println!("{}\n", Colors::header("Skill Status"));

    let installed = SkillLoader::load_installed_skills();

    if installed.is_empty() {
        println!("  {}", Colors::dim("No skills installed"));
        return Ok(());
    }

    println!("{}:", Colors::header("Installed"));
    for name in &installed {
        println!("  - {}", Colors::skill(name));
    }

    println!("\n{}\n", Colors::header("Validation"));

    let validation = SkillLoader::validate_skills();

    if validation.valid {
        println!("  {}", Colors::success("All skills are valid!"));
    } else {
        println!(
            "  {} {} error(s):",
            Colors::error("Found"),
            validation.errors.len()
        );
        for error in &validation.errors {
            println!(
                "    {} {}: {}",
                Colors::error("[ERROR]"),
                Colors::skill(&error.skill),
                error.message
            );
        }
    }

    if !validation.warnings.is_empty() {
        println!(
            "\n  {} ({}):",
            Colors::warning("Warnings"),
            validation.warnings.len()
        );
        for warning in &validation.warnings {
            println!("    {}", Colors::warning(warning));
        }
    }

    println!("\n{}", Colors::header("Skill Sources"));

    let sources = SkillLoader::get_skill_sources();
    if sources.is_empty() {
        println!("  {}", Colors::dim("No skill sources configured"));
        println!("  {}: smart-skills init", Colors::dim("Run"));
    } else {
        println!("  {}", Colors::dim("Configured sources (priority order):"));
        for source in &sources {
            let exists = std::path::Path::new(&source.path).exists();
            let status = if exists {
                Colors::success("[ok]")
            } else {
                Colors::error("[not found]")
            };
            println!(
                "    - {} (priority: {}) {}",
                Colors::dim(&source.path),
                source.priority,
                status
            );
        }
    }

    Ok(())
}

pub fn clear() -> Result<(), String> {
    ensure_initialized()?;

    let installed = SkillLoader::load_installed_skills();

    if installed.is_empty() {
        println!("{}", Colors::dim("Nothing to clear"));
        return Ok(());
    }

    println!("{}...", Colors::header("Clearing all skills"));

    for name in &installed {
        SkillInstaller::remove(name)?;
        println!("  {}: {}", Colors::success("Removed"), Colors::skill(name));
    }

    println!("{}", Colors::success("Done!"));

    Ok(())
}

pub fn config_cmd() -> Result<(), String> {
    ensure_initialized()?;

    println!("{}\n", Colors::header("Smart Skills Configuration"));

    let project_config = config::project_config_path();
    println!("{}", Colors::dim(&project_config.display().to_string()));

    let cfg = Config::load(&project_config);
    println!("  {}:", Colors::header("Skill sources"));
    for source in &cfg.skill_sources {
        let path = std::path::Path::new(&source.path);
        let exists = path.exists();
        let count = if exists {
            std::fs::read_dir(path)
                .map(|entries| {
                    entries
                        .filter_map(|e| e.ok())
                        .filter(|e| e.path().is_dir())
                        .count()
                })
                .unwrap_or(0)
        } else {
            0
        };
        let status = if exists {
            Colors::success("[ok]")
        } else {
            Colors::error("[missing]")
        };
        println!(
            "    - {} (priority: {}, {} skill(s)) {}",
            Colors::dim(&source.path),
            source.priority,
            count,
            status
        );
    }

    println!("  {}:", Colors::header("Install targets"));
    println!("    - agents: {}", cfg.install_targets.agents);
    println!("    - cursor: {}", cfg.install_targets.cursor);
    println!("    - claude: {}", cfg.install_targets.claude);

    Ok(())
}

pub fn set_sources(paths: Vec<String>) -> Result<(), String> {
    if paths.is_empty() {
        println!(
            "{}",
            Colors::dim("Usage: smart-skills set-sources <path>...")
        );
        println!(
            "{}",
            Colors::dim("Example: smart-skills set-sources ./skills ~/my-skills")
        );
        return Ok(());
    }

    ensure_initialized()?;

    let mut sources: Vec<SkillSource> = paths
        .into_iter()
        .enumerate()
        .map(|(i, path)| SkillSource {
            path,
            priority: (10 - i as u8) * 10,
        })
        .collect();

    sources.sort_by(|a, b| b.priority.cmp(&a.priority));

    // Load existing config to preserve targets
    let project_config = config::project_config_path();
    let existing_config = Config::load(&project_config);

    let config = Config {
        skill_sources: sources,
        install_targets: existing_config.install_targets,
    };

    fs::create_dir_all(config::project_config_dir()).map_err(|e| e.to_string())?;
    config.save(&project_config)?;

    println!("Updated skill sources:");
    for source in &config.skill_sources {
        println!(
            "  - {} (priority: {})",
            Colors::skill(&source.path),
            source.priority
        );
    }

    Ok(())
}
