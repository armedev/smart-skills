use crate::cli::Colors;
use crate::config::{self, Config, InstallTargets, SkillSource};
use crate::skills::installer::SkillInstaller;
use crate::skills::loader::SkillLoader;
use std::fs;
use std::io::{stdout, IsTerminal};

pub fn init(source: String, targets: Option<Vec<String>>) -> Result<(), String> {
    let global_skills_path = config::global_skills_dir();
    let global_exists = global_skills_path.exists();

    if source.is_empty() && !global_exists {
        println!("No global skill source found. Please specify a skill source:");
        println!();
        println!("  # Option 1: Use local directory");
        println!("  smart-skills init --skills-source ./skills");
        println!();
        println!("  # Option 2: Use global directory (create it first)");
        println!("  mkdir -p ~/.config/smart-skills/skills");
        println!("  # Add your skills to ~/.config/smart-skills/skills/");
        println!("  smart-skills init");
        println!();
        println!("  # Option 3: Clone from a repo");
        println!("  git clone <your-skills-repo> ~/.config/smart-skills/skills");
        println!("  smart-skills init");
        println!();
        return Ok(());
    }

    // Determine which targets to use
    let final_targets: Vec<String> = match targets {
        Some(t) => t, // Explicit targets provided - use them
        None => {
            // No targets provided - check if interactive
            if stdout().is_terminal() {
                // Interactive mode - prompt user
                prompt_for_targets()?
            } else {
                // Non-interactive - default to agents only
                vec!["agents".to_string()]
            }
        }
    };

    let skill_source = if !source.is_empty() {
        source.clone()
    } else if global_exists {
        global_skills_path.to_string_lossy().to_string()
    } else {
        "skills".to_string()
    };

    let skill_path = std::path::Path::new(&skill_source);
    let source_exists = skill_path.exists();
    let source_label = if !source.is_empty() {
        format!("user-specified: {}", source)
    } else if global_exists {
        "global: ~/.config/smart-skills/skills/".to_string()
    } else {
        "default: skills/".to_string()
    };

    // Create directories only for selected targets
    fs::create_dir_all(config::project_config_dir()).map_err(|e| e.to_string())?;

    if final_targets.contains(&"agents".to_string()) {
        fs::create_dir_all(config::agents_skills_dir()).map_err(|e| e.to_string())?;
    }
    if final_targets.contains(&"cursor".to_string()) {
        fs::create_dir_all(config::cursor_rules_dir()).map_err(|e| e.to_string())?;
    }
    if final_targets.contains(&"claude".to_string()) {
        fs::create_dir_all(config::claude_rules_dir()).map_err(|e| e.to_string())?;
    }

    // Set config with only selected targets enabled
    let config = Config {
        skill_sources: vec![SkillSource {
            path: skill_source.clone(),
            priority: 10,
        }],
        install_targets: InstallTargets {
            agents: final_targets.contains(&"agents".to_string()),
            cursor: final_targets.contains(&"cursor".to_string()),
            claude: final_targets.contains(&"claude".to_string()),
        },
    };

    config.save(&config::project_config_path())?;
    println!("  Using skill source: {}", source_label);

    if !source_exists {
        println!(
            "  {}: Skill source directory does not exist",
            Colors::error("Error")
        );
        println!();
        println!("  Please create the skill source directory and add skills:");
        println!("    mkdir -p {}", skill_source.clone());
        println!("    # Add SKILL.md files to each skill directory");
        return Ok(());
    }

    let available = SkillLoader::load_available_skills();

    if available.is_empty() {
        println!(
            "  {}: No skills found in skill source",
            Colors::error("Error")
        );
        println!();
        println!("  Add skills to your skill source directory:");
        println!("    # Create skill directories with SKILL.md files");
        println!("    mkdir -p <skill-source>/my-skill");
        println!("    echo '## My Skill' > <skill-source>/my-skill/SKILL.md");
        println!();
        println!("  Then run: smart-skills sync");
        return Ok(());
    }

    for skill in available.values() {
        SkillInstaller::install(skill)?;
        println!(
            "  {}: {}",
            Colors::success("Installed"),
            Colors::skill(&skill.name)
        );
    }

    println!("{}", Colors::success("Done!"));

    Ok(())
}

fn prompt_for_targets() -> Result<Vec<String>, String> {
    println!("Select targets for skill installation:");
    println!("  [1] agents - opencode/nvim");
    println!("  [2] cursor - Cursor IDE");
    println!("  [3] claude - Claude Code");
    println!("  [4] all");
    println!();
    println!("Enter numbers separated by commas (default: 1):");

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .map_err(|e| e.to_string())?;

    let input = input.trim();

    if input.is_empty() {
        return Ok(vec!["agents".to_string()]);
    }

    let mut targets = Vec::new();

    for choice in input.split(',') {
        match choice.trim() {
            "1" => targets.push("agents".to_string()),
            "2" => targets.push("cursor".to_string()),
            "3" => targets.push("claude".to_string()),
            "4" => {
                return Ok(vec![
                    "agents".to_string(),
                    "cursor".to_string(),
                    "claude".to_string(),
                ]);
            }
            _ => return Err(format!("Invalid choice: {}", choice)),
        }
    }

    if targets.is_empty() {
        targets.push("agents".to_string());
    }

    Ok(targets)
}

pub fn add(skill_names: Vec<String>) -> Result<(), String> {
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
    let project_config = config::project_config_path();
    let global_config = config::global_config_path();

    if !project_config.exists() && !global_config.exists() {
        println!(
            "{}",
            Colors::warning("No config found. Run 'smart-skills init' first.")
        );
        return Ok(());
    }

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
    println!("{}\n", Colors::header("Smart Skills Configuration"));

    let project_config = config::project_config_path();

    if project_config.exists() {
        println!("{}", Colors::dim(&project_config.display().to_string()));
        let cfg = Config::load(&project_config);
        println!("  {}:", Colors::header("Skill sources"));
        for source in &cfg.skill_sources {
            println!(
                "    - {} (priority: {})",
                Colors::dim(&source.path),
                source.priority
            );
        }
        println!("  {}:", Colors::header("Install targets"));
        println!("    - agents: {}", cfg.install_targets.agents);
        println!("    - cursor: {}", cfg.install_targets.cursor);
        println!("    - claude: {}", cfg.install_targets.claude);

        let sources = SkillLoader::get_skill_sources();
        if !sources.is_empty() {
            println!(
                "\n{} (priority order):",
                Colors::header("Effective skill sources")
            );
            for source in &sources {
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
                    "  - {} ({} skill(s)) {}",
                    Colors::dim(&source.path),
                    count,
                    status
                );
            }
        }
    } else {
        println!("  {}", Colors::dim("No config found"));
        println!(
            "  {}: {}",
            Colors::dim("Run"),
            Colors::skill("smart-skills init")
        );
    }

    Ok(())
}

pub fn set_sources(paths: Vec<String>) -> Result<(), String> {
    let mut sources: Vec<SkillSource> = paths
        .into_iter()
        .enumerate()
        .map(|(i, path)| SkillSource {
            path,
            priority: (10 - i as u8) * 10,
        })
        .collect();

    sources.sort_by(|a, b| b.priority.cmp(&a.priority));

    let config = Config {
        skill_sources: sources,
        install_targets: InstallTargets::default(),
    };

    let project_config = config::project_config_path();
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
