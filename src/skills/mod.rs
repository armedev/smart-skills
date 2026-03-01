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
    use tempfile::TempDir;

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
        assert_ne!(SkillSource::Global, SkillSource::Project);
        assert_ne!(SkillSource::Bundled, SkillSource::Project);
    }

    #[test]
    fn test_skill_from_file_valid() {
        let temp_dir = TempDir::new().unwrap();
        let skill_file = temp_dir.path().join("SKILL.md");
        fs::write(&skill_file, "## Test\n\n* Description").unwrap();

        let skill = Skill::from_file("test".to_string(), skill_file, SkillSource::Project);

        assert!(skill.is_some());
        let skill = skill.unwrap();
        assert_eq!(skill.name, "test");
        assert_eq!(skill.description, "Description");
        assert!(skill.content.contains("## Test"));
        assert_eq!(skill.source, SkillSource::Project);
    }

    #[test]
    fn test_skill_from_file_nonexistent() {
        let skill = Skill::from_file(
            "test".to_string(),
            PathBuf::from("/nonexistent/path/SKILL.md"),
            SkillSource::Project,
        );
        assert!(skill.is_none());
    }

    #[test]
    fn test_extract_description_no_bullets() {
        let content = "## Title\n\nJust text without bullets";
        let desc = Skill::extract_description(content);
        // Should return the first non-empty, non-header line
        assert_eq!(desc, "Just text without bullets");
    }

    #[test]
    fn test_extract_description_only_header() {
        let content = "## Title";
        let desc = Skill::extract_description(content);
        assert_eq!(desc, "");
    }

    #[test]
    fn test_extract_description_whitespace_only() {
        let content = "   \n\n   ";
        let desc = Skill::extract_description(content);
        assert_eq!(desc, "");
    }

    #[test]
    fn test_skill_struct_creation() {
        let skill = Skill {
            name: "test".to_string(),
            description: "desc".to_string(),
            content: "content".to_string(),
            source: SkillSource::Global,
        };
        assert_eq!(skill.name, "test");
        assert_eq!(skill.description, "desc");
        assert_eq!(skill.content, "content");
        assert_eq!(skill.source, SkillSource::Global);
    }
}
