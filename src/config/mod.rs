use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

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
        fs::write(path, content).map_err(|e| e.to_string())?;
        Ok(())
    }
}

#[allow(dead_code)]
pub fn global_config_path() -> PathBuf {
    global_config_dir().join("config.json")
}

pub fn global_config_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".config")
        .join("smart-skills")
}

pub fn global_skills_dir() -> PathBuf {
    global_config_dir().join("skills")
}

#[allow(dead_code)]
pub fn project_skills_dir() -> PathBuf {
    PathBuf::from("skills")
}

pub fn project_config_dir() -> PathBuf {
    PathBuf::from(".smart-skills")
}

pub fn project_config_path() -> PathBuf {
    project_config_dir().join("config.json")
}

pub fn agents_dir() -> PathBuf {
    PathBuf::from(".agents")
}

pub fn agents_skills_dir() -> PathBuf {
    agents_dir().join("skills")
}

pub fn cursor_rules_dir() -> PathBuf {
    PathBuf::from(".cursor").join("rules")
}

pub fn claude_rules_dir() -> PathBuf {
    PathBuf::from(".claude").join("rules")
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
                priority: 10,
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
        assert_eq!(loaded.skill_sources[0].priority, 10);
        assert!(!loaded.install_targets.cursor);
    }

    #[test]
    fn test_config_load_nonexistent() {
        let config_path = PathBuf::from("/nonexistent/path/config.json");
        let config = Config::load(&config_path);
        // Should return default config
        assert!(config.skill_sources.is_empty());
    }

    #[test]
    fn test_path_helpers() {
        // Just make sure these don't panic
        let _ = global_config_path();
        let _ = global_config_dir();
        let _ = global_skills_dir();
        let _ = project_config_path();
        let _ = agents_skills_dir();
        let _ = cursor_rules_dir();
        let _ = claude_rules_dir();
    }
}
