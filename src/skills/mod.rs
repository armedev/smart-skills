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
        // Try to parse YAML frontmatter
        if content.starts_with("---") {
            if let Some(end) = content.find("\n---") {
                let yaml = &content[3..end];
                if let Ok(frontmatter) = serde_yaml::from_str::<serde_yaml::Value>(yaml) {
                    if let Some(desc) = frontmatter.get("description").and_then(|v| v.as_str()) {
                        return desc.to_string();
                    }
                }
            }
        }

        // Fallback: extract first non-empty, non-header line
        content
            .lines()
            .find(|l| !l.trim().is_empty() && !l.trim().starts_with('#'))
            .map(|l| l.trim().trim_start_matches("* ").to_string())
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
    use crate::config::SKILL_FILE;
    use tempfile::TempDir;

    #[test]
    fn test_extract_description_simple() {
        let content = "## Planning\n\n* Test first\n* Document everything";
        assert_eq!(Skill::extract_description(content), "Test first");
    }

    #[test]
    fn test_extract_description_from_frontmatter() {
        let content =
            "---\nname: planning\ndescription: Frontmatter description\n---\n\n## Planning";
        assert_eq!(
            Skill::extract_description(content),
            "Frontmatter description"
        );
    }

    #[test]
    fn test_extract_description_empty() {
        assert_eq!(Skill::extract_description(""), "");
    }

    #[test]
    fn test_skill_source_enum_equality() {
        assert_eq!(SkillSource::Bundled, SkillSource::Bundled);
        assert_ne!(SkillSource::Bundled, SkillSource::Global);
    }

    #[test]
    fn test_skill_from_file_valid() {
        let temp_dir = TempDir::new().unwrap();
        let skill_file = temp_dir.path().join(SKILL_FILE);
        fs::write(&skill_file, "## Test\n\n* Description").unwrap();

        let skill = Skill::from_file("test".to_string(), skill_file, SkillSource::Project).unwrap();
        assert_eq!(skill.name, "test");
        assert_eq!(skill.description, "Description");
    }

    #[test]
    fn test_skill_from_file_nonexistent() {
        assert!(Skill::from_file(
            "test".to_string(),
            PathBuf::from("/nonexistent/SKILL.md"),
            SkillSource::Project,
        )
        .is_none());
    }
}
