use crate::cli::Colors;
use crate::config::{self, Config, InstallTargets, SkillSource, DEFAULT_PRIORITY};
use crate::skills::installer::SkillInstaller;
use crate::skills::loader::SkillLoader;
use std::fs;

fn ensure_initialized() -> Result<(), String> {
    if !config::global_config_path().exists() {
        return Err("No global config. Run 'smart-skills init' first.".to_string());
    }
    Ok(())
}

fn parse_targets(targets: Option<Vec<String>>) -> Result<Option<InstallTargets>, String> {
    let targets = targets.ok_or_else(|| "Target cannot be empty".to_string())?;

    if targets.is_empty() {
        return Err("Target cannot be empty".to_string());
    }

    let mut install_targets = InstallTargets::default();

    for t in &targets {
        match t.as_str() {
            "agents" => install_targets.agents = true,
            "cursor" => install_targets.cursor = true,
            "claude" => install_targets.claude = true,
            _ => {
                return Err(format!(
                    "Invalid target: {}. Valid: agents, cursor, claude",
                    t
                ))
            }
        }
    }

    Ok(Some(install_targets))
}

pub fn add(skills: Vec<String>, targets: Option<Vec<String>>) -> Result<(), String> {
    ensure_initialized()?;

    let targets_override = parse_targets(targets)?;
    if let Some(ref t) = targets_override {
        println!(
            "  {}: agents={}, cursor={}, claude={}",
            Colors::dim("Target"),
            t.agents,
            t.cursor,
            t.claude
        );
    }

    let available = SkillLoader::load_available_skills();
    let installed = SkillLoader::load_installed_skills();

    if skills.is_empty() {
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
            "\nTo install: {}",
            Colors::skill("smart-skills add <skill>")
        );
        return Ok(());
    }

    for name in skills {
        if let Some(skill) = available.get(&name) {
            SkillInstaller::install(skill, targets_override.clone())?;
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

pub fn remove(skills: Vec<String>, targets: Option<Vec<String>>) -> Result<(), String> {
    ensure_initialized()?;

    let targets_override = parse_targets(targets)?;
    if let Some(ref t) = targets_override {
        println!(
            "  {}: agents={}, cursor={}, claude={}",
            Colors::dim("Target"),
            t.agents,
            t.cursor,
            t.claude
        );
    }

    if skills.is_empty() {
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

    for name in &skills {
        SkillInstaller::remove(name, targets_override.clone())?;
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
    for name in available.keys() {
        let status = if installed.contains(name) {
            Colors::success("[installed]")
        } else {
            Colors::dim("")
        };
        println!("  - {} {}", Colors::skill(name), status);
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

pub fn sync(remove_stale: bool, targets: Option<Vec<String>>) -> Result<(), String> {
    ensure_initialized()?;

    let targets_override = parse_targets(targets)?;
    if let Some(ref t) = targets_override {
        println!(
            "  {}: agents={}, cursor={}, claude={}",
            Colors::dim("Target"),
            t.agents,
            t.cursor,
            t.claude
        );
    }

    let available = SkillLoader::load_available_skills();
    let installed = SkillLoader::load_installed_skills();

    if installed.is_empty() && !remove_stale {
        println!("  {}", Colors::dim("Nothing to sync"));
        return Ok(());
    }

    let mut count = 0;

    // Remove stale from specified targets (CLI > config)
    if remove_stale {
        for name in &installed {
            if !available.contains_key(name) {
                SkillInstaller::remove(name, targets_override.clone())?;
                println!(
                    "  {} stale: {}",
                    Colors::success("Removed"),
                    Colors::skill(name)
                );
                count += 1;
            }
        }
    }

    for name in &installed {
        if let Some(skill) = available.get(name) {
            SkillInstaller::install(skill, targets_override.clone())?;
            println!("  {}: {}", Colors::success("Synced"), Colors::skill(name));
            count += 1;
        }
    }

    if count == 0 {
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
    let result = SkillLoader::validate_skills();

    if result.valid {
        println!("  {}", Colors::success("All skills are valid!"));
    } else {
        println!(
            "  {} {} error(s):",
            Colors::error("Found"),
            result.errors.len()
        );
        for err in &result.errors {
            println!(
                "    {} {}: {}",
                Colors::error("[ERROR]"),
                Colors::skill(&err.skill),
                err.message
            );
        }
    }

    if !result.warnings.is_empty() {
        println!(
            "\n  {} ({}):",
            Colors::warning("Warnings"),
            result.warnings.len()
        );
        for w in &result.warnings {
            println!("    {}", Colors::warning(w));
        }
    }

    println!("\n{}", Colors::header("Skill Sources"));
    let sources = SkillLoader::get_skill_sources();
    if sources.is_empty() {
        println!("  {}", Colors::dim("No skill sources configured"));
    } else {
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

pub fn clear(targets: Option<Vec<String>>) -> Result<(), String> {
    ensure_initialized()?;

    let targets_override = parse_targets(targets)?;
    if let Some(ref t) = targets_override {
        println!(
            "  {}: agents={}, cursor={}, claude={}",
            Colors::dim("Target"),
            t.agents,
            t.cursor,
            t.claude
        );
    }

    let installed = SkillLoader::load_installed_skills();
    if installed.is_empty() {
        println!("{}", Colors::dim("Nothing to clear"));
        return Ok(());
    }

    println!("{}...", Colors::header("Clearing all skills"));
    for name in &installed {
        SkillInstaller::remove(name, targets_override.clone())?;
        println!("  {}: {}", Colors::success("Removed"), Colors::skill(name));
    }
    println!("{}", Colors::success("Done!"));
    Ok(())
}

pub fn config_cmd() -> Result<(), String> {
    ensure_initialized()?;

    let path = config::global_config_path();
    println!("{}\n", Colors::header("Smart Skills Configuration"));
    println!("{}", Colors::dim(&path.display().to_string()));

    let cfg = Config::load(&path);
    println!("  {}:", Colors::header("Skill sources"));
    for source in &cfg.skill_sources {
        let p = std::path::Path::new(&source.path);
        let exists = p.exists();
        let count = p
            .exists()
            .then(|| fs::read_dir(p).ok())
            .flatten()
            .map(|entries| {
                entries
                    .filter_map(|e| e.ok())
                    .filter(|e| e.path().is_dir())
                    .count()
            })
            .unwrap_or(0);
        let status = if exists {
            Colors::success("[ok]")
        } else {
            Colors::error("[missing]")
        };
        println!(
            "    - {} (priority: {}, {} skills) {}",
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

pub fn set_sources(paths: Vec<String>, overwrite: bool) -> Result<(), String> {
    if paths.is_empty() {
        println!(
            "{}",
            Colors::dim("Usage: smart-skills set-sources <path>... [--overwrite]")
        );
        return Ok(());
    }

    ensure_initialized()?;

    let path = config::global_config_path();
    let mut cfg = Config::load(&path);

    let sources = if overwrite { vec![] } else { cfg.skill_sources };
    let max_priority = sources.iter().map(|s| s.priority).max().unwrap_or(0);

    let mut new_sources: Vec<_> = paths
        .into_iter()
        .enumerate()
        .map(|(i, p)| SkillSource {
            path: config::resolve_path(&p).display().to_string(),
            priority: max_priority + DEFAULT_PRIORITY * (i as u8 + 1),
        })
        .collect();

    let mut all = sources;
    all.append(&mut new_sources);
    all.sort_by(|a, b| b.priority.cmp(&a.priority));
    cfg.skill_sources = all;

    fs::create_dir_all(config::global_config_dir()).map_err(|e| e.to_string())?;
    cfg.save(&path)?;

    println!("Updated skill sources:");
    for s in &cfg.skill_sources {
        println!("  - {} (priority: {})", Colors::skill(&s.path), s.priority);
    }
    Ok(())
}
