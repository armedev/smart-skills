use crate::cli::Colors;
use crate::config::{self, Config, InstallTargets, SkillSource};
use crate::skills::installer::SkillInstaller;
use crate::skills::loader::SkillLoader;
use std::fs;
use std::io::{stdout, IsTerminal};
use std::path::PathBuf;

pub fn init(cli_source: String, cli_targets: Option<Vec<String>>) -> Result<(), String> {
    let has_cli_args = !cli_source.is_empty() || cli_targets.is_some();
    let global_config_exists = config::global_config_path().exists();

    let (source, targets) = if has_cli_args {
        resolve_with_args(cli_source, cli_targets, global_config_exists)?
    } else {
        resolve_without_args(global_config_exists)?
    };

    let resolved_source = resolve_source_path(&source)?;
    let config = create_config(&resolved_source, &targets);
    setup_project(&config)?;

    if !resolved_source.is_empty() {
        install_skills(&resolved_source)?;
    }

    println!("{}", Colors::success("Done!"));
    Ok(())
}

fn resolve_with_args(
    cli_source: String,
    cli_targets: Option<Vec<String>>,
    global_config_exists: bool,
) -> Result<(String, Vec<String>), String> {
    let global_skills_path = config::global_skills_dir();
    let global_skills_exists = global_skills_path.exists();

    if global_config_exists {
        let global_cfg = Config::load(&config::global_config_path());

        let source = if cli_source.is_empty() {
            resolve_global_source_to_absolute()
        } else {
            cli_source
        };

        let targets = match cli_targets {
            Some(t) => t,
            None => targets_from_config(&global_cfg),
        };

        Ok((source, targets))
    } else {
        let source = if cli_source.is_empty() {
            if global_skills_exists {
                global_skills_path.to_string_lossy().to_string()
            } else {
                String::new()
            }
        } else {
            cli_source
        };

        let targets = match cli_targets {
            Some(t) => t,
            None => resolve_targets_interactive_or_default(),
        };

        Ok((source, targets))
    }
}

fn resolve_without_args(global_config_exists: bool) -> Result<(String, Vec<String>), String> {
    let global_skills_path = config::global_skills_dir();
    let global_skills_exists = global_skills_path.exists();

    if global_config_exists {
        let global_cfg = Config::load(&config::global_config_path());
        let source = resolve_global_source_to_absolute();
        let targets = targets_from_config(&global_cfg);
        Ok((source, targets))
    } else {
        let source = if global_skills_exists {
            global_skills_path.to_string_lossy().to_string()
        } else {
            String::new()
        };
        let targets = resolve_targets_interactive_or_default();
        Ok((source, targets))
    }
}

fn resolve_global_source_to_absolute() -> String {
    let global_cfg = Config::load(&config::global_config_path());
    global_cfg
        .skill_sources
        .first()
        .map(|s| {
            let path = PathBuf::from(&s.path);
            if path.is_absolute() {
                s.path.clone()
            } else {
                let resolved = config::global_config_dir().join(&s.path);
                normalize_path(resolved)
            }
        })
        .unwrap_or_default()
}

fn normalize_path(path: PathBuf) -> String {
    path.components()
        .filter(|c| !matches!(c, std::path::Component::CurDir))
        .collect::<PathBuf>()
        .to_string_lossy()
        .to_string()
}

fn targets_from_config(cfg: &Config) -> Vec<String> {
    let mut targets = Vec::new();
    if cfg.install_targets.agents {
        targets.push("agents".to_string());
    }
    if cfg.install_targets.cursor {
        targets.push("cursor".to_string());
    }
    if cfg.install_targets.claude {
        targets.push("claude".to_string());
    }
    if targets.is_empty() {
        targets.push("agents".to_string());
    }
    targets
}

fn resolve_targets_interactive_or_default() -> Vec<String> {
    if stdout().is_terminal() {
        prompt_for_targets().unwrap_or_else(|_| vec!["agents".to_string()])
    } else {
        vec!["agents".to_string()]
    }
}

fn resolve_source_path(source: &str) -> Result<String, String> {
    if source.is_empty() {
        return Ok(String::new());
    }

    let path = PathBuf::from(source);

    if path.is_absolute() {
        return Ok(source.to_string());
    }

    let normalized = path
        .components()
        .filter(|c| !matches!(c, std::path::Component::CurDir))
        .collect::<PathBuf>();

    let cwd = std::env::current_dir().map_err(|e| e.to_string())?;
    let cwd_path = cwd.join(&normalized);

    if cwd_path.exists() {
        return Ok(cwd_path.to_string_lossy().to_string());
    }

    Ok(source.to_string())
}

fn create_config(source: &str, targets: &[String]) -> Config {
    let skill_sources = if source.is_empty() {
        Vec::new()
    } else {
        vec![SkillSource {
            path: source.to_string(),
            priority: 10,
        }]
    };

    Config {
        skill_sources,
        install_targets: InstallTargets {
            agents: targets.contains(&"agents".to_string()),
            cursor: targets.contains(&"cursor".to_string()),
            claude: targets.contains(&"claude".to_string()),
        },
    }
}

fn setup_project(config: &Config) -> Result<(), String> {
    fs::create_dir_all(config::project_config_dir()).map_err(|e| e.to_string())?;

    if config.install_targets.agents {
        fs::create_dir_all(config::agents_skills_dir()).map_err(|e| e.to_string())?;
    }
    if config.install_targets.cursor {
        fs::create_dir_all(config::cursor_rules_dir()).map_err(|e| e.to_string())?;
    }
    if config.install_targets.claude {
        fs::create_dir_all(config::claude_rules_dir()).map_err(|e| e.to_string())?;
    }

    config.save(&config::project_config_path())?;

    if config.skill_sources.is_empty() {
        println!("{}", Colors::warning("No skill source configured"));
        println!(
            "  {}: {}",
            Colors::dim("Add source with"),
            Colors::skill("smart-skills set-sources <path>")
        );
    } else {
        println!(
            "  {}: {}",
            Colors::dim("Skill source"),
            Colors::skill(&config.skill_sources[0].path)
        );
    }

    Ok(())
}

fn install_skills(source: &str) -> Result<(), String> {
    let source_path = PathBuf::from(source);

    if !source_path.exists() {
        println!(
            "  {}: Skill source directory does not exist",
            Colors::warning("Warning")
        );
        println!("  {}: mkdir -p {}", Colors::dim("Create it with"), source);
        return Ok(());
    }

    let available = SkillLoader::load_available_skills();

    if available.is_empty() {
        println!(
            "  {}: No skills found in source",
            Colors::warning("Warning")
        );
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
