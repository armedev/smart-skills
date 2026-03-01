use crate::config::{self, Config, InstallTargets, SkillSource};
use crate::skills::installer::SkillInstaller;
use crate::skills::loader::SkillLoader;
use std::fs;

pub fn init(source: String) -> Result<(), String> {
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

    fs::create_dir_all(config::project_config_dir()).map_err(|e| e.to_string())?;
    fs::create_dir_all(config::agents_skills_dir()).map_err(|e| e.to_string())?;
    fs::create_dir_all(config::cursor_rules_dir()).map_err(|e| e.to_string())?;

    let config = Config {
        skill_sources: vec![SkillSource {
            path: skill_source.clone(),
            priority: 10,
        }],
        install_targets: InstallTargets::default(),
    };

    config.save(&config::project_config_path())?;
    println!("  Using skill source: {}", source_label);

    if !source_exists {
        println!("  Error: Skill source directory does not exist");
        println!();
        println!("  Please create the skill source directory and add skills:");
        println!("    mkdir -p {}", skill_source.clone());
        println!("    # Add SKILL.md files to each skill directory");
        return Ok(());
    }

    let available = SkillLoader::load_available_skills();

    if available.is_empty() {
        println!("  Error: No skills found in skill source");
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
        println!("  Installed: {}", skill.name);
    }

    println!("Done!");

    Ok(())
}

pub fn add(skill_names: Vec<String>) -> Result<(), String> {
    let available = SkillLoader::load_available_skills();

    if skill_names.is_empty() {
        println!("Available skills:");
        for (name, skill) in &available {
            let installed = if SkillInstaller::is_installed(name) {
                " [installed]"
            } else {
                ""
            };
            println!("  - {}{}", name, installed);
            if !skill.description.is_empty() {
                println!("    {}", skill.description);
            }
        }
        return Ok(());
    }

    for name in skill_names {
        if let Some(skill) = available.get(&name) {
            SkillInstaller::install(skill)?;
            println!("  Installed: {}", name);
        } else {
            println!("  Skill not found: {}", name);
        }
    }

    Ok(())
}

pub fn remove(skill_names: Vec<String>) -> Result<(), String> {
    if skill_names.is_empty() {
        let installed = SkillLoader::load_installed_skills();
        println!("Installed skills:");
        for name in installed {
            println!("  - {}", name);
        }
        return Ok(());
    }

    for name in &skill_names {
        SkillInstaller::remove(name)?;
        println!("  Removed: {}", name);
    }

    Ok(())
}

pub fn list() -> Result<(), String> {
    let available = SkillLoader::load_available_skills();
    let installed = SkillLoader::load_installed_skills();

    println!("Available skills ({}):", available.len());
    for (name, skill) in &available {
        let status = if installed.contains(name) {
            "[installed]"
        } else {
            ""
        };
        println!("  - {} {}", name, status);
        if !skill.description.is_empty() {
            println!("    {}", skill.description);
        }
    }

    println!("\nInstalled skills ({}):", installed.len());
    for name in &installed {
        println!("  - {}", name);
    }

    Ok(())
}

pub fn sync(remove_stale: bool) -> Result<(), String> {
    println!("Syncing skills...");

    let available = SkillLoader::load_available_skills();
    let installed = SkillLoader::load_installed_skills();

    // Remove stale skills if requested
    if remove_stale {
        let mut removed_count = 0;
        for name in &installed {
            if !available.contains_key(name) {
                SkillInstaller::remove(name)?;
                println!("  Removed stale: {}", name);
                removed_count += 1;
            }
        }
        if removed_count > 0 {
            println!("Removed {} stale skill(s)", removed_count);
        }
    }

    // Sync remaining installed skills
    for name in &installed {
        if let Some(skill) = available.get(name) {
            SkillInstaller::install(skill)?;
            println!("  Synced: {}", name);
        }
    }

    println!("Done!");

    Ok(())
}

pub fn status() -> Result<(), String> {
    println!("=== Skill Status ===\n");

    let installed = SkillLoader::load_installed_skills();

    if installed.is_empty() {
        println!("No skills installed.");
        return Ok(());
    }

    println!("Installed skills:");
    for name in &installed {
        println!("  - {}", name);
    }

    println!("\n=== Skill Validation ===\n");

    let validation = SkillLoader::validate_skills();

    if validation.valid {
        println!("All skills are valid!");
    } else {
        println!("Found {} error(s):", validation.errors.len());
        for error in &validation.errors {
            println!("  [ERROR] {}: {}", error.skill, error.message);
        }
    }

    if !validation.warnings.is_empty() {
        println!("\nWarnings ({}):", validation.warnings.len());
        for warning in &validation.warnings {
            println!("  [WARN] {}", warning);
        }
    }

    println!("\n=== Skill Sources ===\n");

    let sources = SkillLoader::get_skill_sources();
    if sources.is_empty() {
        println!("No skill sources configured.");
        println!("  Run 'smart-skills init' to set up default sources");
    } else {
        println!("Configured sources (priority order):");
        for source in &sources {
            let exists = std::path::Path::new(&source.path).exists();
            let status = if exists { "[exists]" } else { "[not found]" };
            println!(
                "  - {} (priority: {}) {}",
                source.path, source.priority, status
            );
        }
    }

    Ok(())
}

pub fn clear() -> Result<(), String> {
    println!("Clearing all skills...");

    let installed = SkillLoader::load_installed_skills();

    for name in &installed {
        SkillInstaller::remove(name)?;
        println!("  Removed: {}", name);
    }

    println!("Done!");

    Ok(())
}

pub fn config_cmd() -> Result<(), String> {
    let project_config = config::project_config_path();
    let global_config = config::global_config_path();

    println!("=== Smart Skills Configuration ===\n");

    if project_config.exists() {
        println!("Project config: {}", project_config.display());
        let cfg = Config::load(&project_config);
        println!("  Skill sources:");
        for source in &cfg.skill_sources {
            println!("    - {} (priority: {})", source.path, source.priority);
        }
        println!("  Install targets:");
        println!("    - agents: {}", cfg.install_targets.agents);
        println!("    - cursor: {}", cfg.install_targets.cursor);
    } else if global_config.exists() {
        println!("Global config: {}", global_config.display());
        let cfg = Config::load(&global_config);
        println!("  Skill sources:");
        for source in &cfg.skill_sources {
            println!("    - {} (priority: {})", source.path, source.priority);
        }
    } else {
        println!("No config found.");
        println!("  Run 'smart-skills init' to create a project config");
        println!("  Or set up global config at: {}", global_config.display());
    }

    let sources = SkillLoader::get_skill_sources();
    if !sources.is_empty() {
        println!("\nEffective skill sources (priority order):");
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
            println!(
                "  - {} ({} skill(s)) {}",
                source.path,
                count,
                if exists { "[ok]" } else { "[missing]" }
            );
        }
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
        println!("  - {} (priority: {})", source.path, source.priority);
    }

    Ok(())
}
