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
}

impl Default for InstallTargets {
    fn default() -> Self {
        Self {
            agents: true,
            cursor: true,
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

pub fn global_config_path() -> PathBuf {
    global_config_dir().join("config.json")
}

pub fn global_config_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("smart-skills")
}

pub fn global_skills_dir() -> PathBuf {
    global_config_dir().join("skills")
}

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
