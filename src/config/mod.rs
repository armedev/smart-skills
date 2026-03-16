use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

pub const SKILL_FILE: &str = "SKILL.md";
pub const DEFAULT_PRIORITY: u8 = 10;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub skill_sources: Vec<SkillSource>,
    pub install_targets: InstallTargets,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillSource {
    pub path: String,
    pub priority: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallTargets {
    pub agents: bool,
    pub cursor: bool,
    pub claude: bool,
}

impl Default for InstallTargets {
    fn default() -> Self {
        Self {
            agents: true,
            cursor: false,
            claude: false,
        }
    }
}

impl Config {
    pub fn load(path: &PathBuf) -> Self {
        if path.exists() {
            if let Ok(content) = fs::read_to_string(path) {
                if let Ok(config) = serde_json::from_str(&content) {
                    return config;
                }
            }
        }
        Self::default()
    }

    pub fn save(&self, path: &PathBuf) -> Result<(), String> {
        let content = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        fs::write(path, content).map_err(|e| e.to_string())
    }
}

pub fn global_config_path() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".config")
        .join("smart-skills")
        .join("config.json")
}

pub fn global_config_dir() -> PathBuf {
    global_config_path().parent().unwrap().to_path_buf()
}

pub fn agents_skills_dir() -> PathBuf {
    PathBuf::from(".agents").join("skills")
}

pub fn cursor_rules_dir() -> PathBuf {
    PathBuf::from(".cursor").join("rules")
}

pub fn claude_rules_dir() -> PathBuf {
    PathBuf::from(".claude").join("rules")
}

pub fn resolve_path(path: &str) -> PathBuf {
    let p = PathBuf::from(path);
    if p.is_relative() {
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(p)
    } else {
        p
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert!(config.skill_sources.is_empty());
        assert!(config.install_targets.agents);
        assert!(!config.install_targets.cursor);
        assert!(!config.install_targets.claude);
    }

    #[test]
    fn test_config_save_and_load() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.json");

        let config = Config {
            skill_sources: vec![SkillSource {
                path: "test".to_string(),
                priority: DEFAULT_PRIORITY,
            }],
            install_targets: InstallTargets {
                agents: true,
                cursor: false,
                claude: true,
            },
        };

        config.save(&config_path).unwrap();
        let loaded = Config::load(&config_path);

        assert_eq!(loaded.skill_sources.len(), 1);
        assert_eq!(loaded.skill_sources[0].path, "test");
        assert_eq!(loaded.skill_sources[0].priority, DEFAULT_PRIORITY);
        assert!(!loaded.install_targets.cursor);
    }

    #[test]
    fn test_config_load_nonexistent() {
        let config = Config::load(&PathBuf::from("/nonexistent/config.json"));
        assert!(config.skill_sources.is_empty());
    }

    #[test]
    fn test_resolve_path_relative() {
        let result = resolve_path("./skills");
        assert!(result.is_absolute());
        assert!(result.to_string_lossy().ends_with("skills"));
    }

    #[test]
    fn test_resolve_path_absolute() {
        assert_eq!(
            resolve_path("/absolute/path"),
            PathBuf::from("/absolute/path")
        );
    }
}
