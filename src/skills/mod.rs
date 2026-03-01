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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_description_simple() {
        let content = "## Planning\n\n* Test first\n* Document everything";
        let desc = Skill::extract_description(content);
        assert_eq!(desc, "Test first");
    }

    #[test]
    fn test_extract_description_with_header() {
        let content = "# Title\n\n* First bullet\n* Second bullet";
        let desc = Skill::extract_description(content);
        assert_eq!(desc, "First bullet");
    }

    #[test]
    fn test_extract_description_empty() {
        let content = "";
        let desc = Skill::extract_description(content);
        assert_eq!(desc, "");
    }

    #[test]
    fn test_skill_source_enum_equality() {
        assert_eq!(SkillSource::Bundled, SkillSource::Bundled);
        assert_ne!(SkillSource::Bundled, SkillSource::Global);
    }
}
