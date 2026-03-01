use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub name: String,
    pub description: String,
    pub content: String,
    pub source: SkillSource,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SkillSource {
    Bundled,
    Global,
    Project,
}

impl Skill {
    pub fn from_file(name: String, path: PathBuf, source: SkillSource) -> Option<Self> {
        let content = fs::read_to_string(&path).ok()?;
        let description = Self::extract_description(&content);

        Some(Skill {
            name,
            description,
            content,
            source,
        })
    }

    fn extract_description(content: &str) -> String {
        content
            .lines()
            .filter(|line| !line.trim().starts_with('#') && !line.trim().is_empty())
            .take(1)
            .map(|line| line.trim().trim_start_matches("* ").to_string())
            .next()
            .unwrap_or_default()
    }
}

pub mod installer;
pub mod loader;

#[allow(unused_imports)]
pub use loader::{SkillLoader, ValidationError, ValidationResult};
