use crate::config::{self, Config, InstallTargets, SKILL_FILE};
use crate::skills::Skill;
use std::fs;

pub struct SkillInstaller;

impl SkillInstaller {
    pub fn install(skill: &Skill, targets: Option<InstallTargets>) -> Result<(), String> {
        let cfg = Self::load_config();
        let targets = targets.unwrap_or(cfg.install_targets);

        if targets.agents {
            Self::install_skill(skill, &config::agents_skills_dir(), false)?;
        }
        if targets.cursor {
            Self::install_skill(skill, &config::cursor_rules_dir(), true)?;
        }
        if targets.claude {
            Self::install_skill(skill, &config::claude_rules_dir(), true)?;
        }
        Ok(())
    }

    fn install_skill(
        skill: &Skill,
        dir: &std::path::Path,
        single_file: bool,
    ) -> Result<(), String> {
        let target_dir = if single_file {
            dir.to_path_buf()
        } else {
            dir.join(&skill.name)
        };

        fs::create_dir_all(&target_dir).map_err(|e| e.to_string())?;

        let path = if single_file {
            dir.join(format!("{}.md", skill.name))
        } else {
            target_dir.join(SKILL_FILE)
        };

        let content = if skill.content.starts_with("---") {
            skill.content.clone()
        } else {
            format!(
                "---\nname: {}\ndescription: {}\n---\n\n{}",
                skill.name, skill.description, skill.content
            )
        };

        fs::write(path, content).map_err(|e| e.to_string())
    }

    fn load_config() -> Config {
        let path = config::global_config_path();
        if path.exists() {
            Config::load(&path)
        } else {
            Config::default()
        }
    }

    pub fn remove(name: &str, targets: Option<InstallTargets>) -> Result<(), String> {
        let cfg = Self::load_config();
        let targets = targets.unwrap_or(cfg.install_targets);

        if targets.agents {
            let dir = config::agents_skills_dir().join(name);
            if dir.exists() {
                fs::remove_dir_all(dir).map_err(|e| e.to_string())?;
            }
        }
        if targets.cursor {
            let path = config::cursor_rules_dir().join(format!("{}.md", name));
            let _ = fs::remove_file(path);
        }
        if targets.claude {
            let path = config::claude_rules_dir().join(format!("{}.md", name));
            let _ = fs::remove_file(path);
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub fn is_installed(name: &str) -> bool {
        config::agents_skills_dir()
            .join(name)
            .join(SKILL_FILE)
            .exists()
    }

    #[allow(dead_code)]
    pub fn remove_all_from_config() -> Result<(), String> {
        let cfg = Self::load_config();
        let targets = cfg.install_targets;

        if targets.agents {
            let dir = config::agents_skills_dir();
            if dir.exists() {
                fs::remove_dir_all(dir).map_err(|e| e.to_string())?;
            }
        }
        if targets.cursor {
            let dir = config::cursor_rules_dir();
            if dir.exists() {
                for e in fs::read_dir(dir).map_err(|e| e.to_string())?.flatten() {
                    let _ = fs::remove_file(e.path());
                }
            }
        }
        if targets.claude {
            let dir = config::claude_rules_dir();
            if dir.exists() {
                for e in fs::read_dir(dir).map_err(|e| e.to_string())?.flatten() {
                    let _ = fs::remove_file(e.path());
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_nonexistent() {
        assert!(SkillInstaller::remove("nonexistent", None).is_ok());
    }

    #[test]
    fn test_is_installed_not_installed() {
        assert!(!SkillInstaller::is_installed("nonexistent"));
    }
}
