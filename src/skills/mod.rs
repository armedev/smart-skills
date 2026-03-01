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

#[derive(Debug, Deserialize)]
struct SkillFrontmatter {
    name: Option<String>,
    description: Option<String>,
}

impl Skill {
    pub fn from_file(name: String, path: PathBuf, source: SkillSource) -> Option<Self> {
        let content = fs::read_to_string(&path).ok()?;
        let (description, has_valid_frontmatter) = Self::extract_description(&content, &name);

        // If valid frontmatter exists, validate name matches directory name
        if has_valid_frontmatter {
            if let Some(ref frontmatter) = Self::parse_frontmatter(&content) {
                if let Some(ref fm_name) = frontmatter.name {
                    if fm_name != &name {
                        eprintln!("Warning: Skill '{}' has frontmatter name '{}' which doesn't match directory name",
                            name, fm_name);
                    }
                }
            }
        }

        Some(Skill {
            name,
            description,
            content,
            source,
        })
    }

    fn parse_frontmatter(content: &str) -> Option<SkillFrontmatter> {
        if !content.starts_with("---") {
            return None;
        }

        let end_marker = content.find("\n---")?;
        let frontmatter_yaml = &content[4..end_marker]; // Skip "---\n"

        serde_yaml::from_str(frontmatter_yaml).ok()
    }

    fn extract_description(content: &str, skill_name: &str) -> (String, bool) {
        // Try to parse frontmatter using YAML
        if let Some(frontmatter) = Self::parse_frontmatter(content) {
            let has_name = frontmatter.name.is_some();
            let has_desc = frontmatter.description.is_some();

            if has_name && has_desc {
                // Both present, use frontmatter description
                return (frontmatter.description.unwrap(), true);
            } else if has_name || has_desc {
                // Partial frontmatter - warn and fall back
                let missing = if !has_name { "name" } else { "description" };
                eprintln!("Warning: Skill '{}' has incomplete frontmatter (missing '{}'), using auto-generated values",
                    skill_name, missing);
            }
        }

        // Fallback to extracting from content body (skip frontmatter if present)
        let content_without_frontmatter = if content.starts_with("---") {
            if let Some(end_marker) = content.find("\n---") {
                &content[end_marker + 4..] // Skip past frontmatter
            } else {
                content
            }
        } else {
            content
        };

        let desc = content_without_frontmatter
            .lines()
            .filter(|line| {
                let trimmed = line.trim();
                !trimmed.is_empty() && !trimmed.starts_with("#") && !trimmed.starts_with("---")
            })
            .take(1)
            .map(|line| line.trim().trim_start_matches("* ").to_string())
            .next()
            .unwrap_or_default();
        (desc, false)
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
        let (desc, has_fm) = Skill::extract_description(content, "test");
        assert_eq!(desc, "Test first");
        assert!(!has_fm);
    }

    #[test]
    fn test_extract_description_with_header() {
        let content = "# Title\n\n* First bullet\n* Second bullet";
        let (desc, has_fm) = Skill::extract_description(content, "test");
        assert_eq!(desc, "First bullet");
        assert!(!has_fm);
    }

    #[test]
    fn test_extract_description_empty() {
        let content = "";
        let (desc, has_fm) = Skill::extract_description(content, "test");
        assert_eq!(desc, "");
        assert!(!has_fm);
    }

    #[test]
    fn test_extract_description_from_frontmatter() {
        let content =
            "---\nname: planning\ndescription: Frontmatter description\n---\n\n## Planning";
        let (desc, has_fm) = Skill::extract_description(content, "planning");
        assert_eq!(desc, "Frontmatter description");
        assert!(has_fm);
    }

    #[test]
    fn test_extract_description_incomplete_frontmatter_missing_name() {
        let content = "---\ndescription: Only description\n---\n\n## Test";
        let (_desc, has_fm) = Skill::extract_description(content, "test");
        // Should fall back to auto-extraction since name is missing
        assert!(!has_fm);
    }

    #[test]
    fn test_extract_description_incomplete_frontmatter_missing_description() {
        let content = "---\nname: test\n---\n\n## Test";
        let (_desc, has_fm) = Skill::extract_description(content, "test");
        // Should fall back to auto-extraction since description is missing
        assert!(!has_fm);
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
        let (desc, has_fm) = Skill::extract_description(content, "test");
        // Should return the first non-empty, non-header line
        assert_eq!(desc, "Just text without bullets");
        assert!(!has_fm);
    }

    #[test]
    fn test_extract_description_only_header() {
        let content = "## Title";
        let (desc, has_fm) = Skill::extract_description(content, "test");
        assert_eq!(desc, "");
        assert!(!has_fm);
    }

    #[test]
    fn test_extract_description_whitespace_only() {
        let content = "   \n\n   ";
        let (desc, has_fm) = Skill::extract_description(content, "test");
        assert_eq!(desc, "");
        assert!(!has_fm);
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

    #[test]
    fn test_yaml_frontmatter_with_special_chars() {
        let content = "---\nname: test-skill\ndescription: \"Description with: special chars\"\n---\n\n## Test";
        let (desc, has_fm) = Skill::extract_description(content, "test-skill");
        assert_eq!(desc, "Description with: special chars");
        assert!(has_fm);
    }

    #[test]
    fn test_yaml_frontmatter_multiline() {
        let content =
            "---\nname: test-skill\ndescription: |\n  Multi-line\n  description\n---\n\n## Test";
        let (desc, has_fm) = Skill::extract_description(content, "test-skill");
        assert!(desc.contains("Multi-line"));
        assert!(has_fm);
    }
}
