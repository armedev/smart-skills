use crate::config::{self, Config};
use crate::skills::Skill;
use std::fs;

#[allow(dead_code)]
pub struct SkillInstaller;

#[allow(dead_code)]
impl SkillInstaller {
    pub fn install(skill: &Skill) -> Result<(), String> {
        let cfg = Self::load_config();

        if cfg.install_targets.agents {
            Self::install_to_agents(skill)?;
        }
        if cfg.install_targets.cursor {
            Self::install_to_cursor(skill)?;
        }
        Ok(())
    }

    fn load_config() -> Config {
        let project_config = config::project_config_path();
        if project_config.exists() {
            return Config::load(&project_config);
        }
        Config::default()
    }

    fn install_to_agents(skill: &Skill) -> Result<(), String> {
        let target_dir = config::agents_skills_dir().join(&skill.name);
        fs::create_dir_all(&target_dir).map_err(|e| e.to_string())?;

        let target_path = target_dir.join("SKILL.md");
        fs::write(&target_path, &skill.content).map_err(|e| e.to_string())?;

        Ok(())
    }

    fn install_to_cursor(skill: &Skill) -> Result<(), String> {
        let target_dir = config::cursor_rules_dir();
        fs::create_dir_all(&target_dir).map_err(|e| e.to_string())?;

        let target_path = target_dir.join(format!("{}.md", skill.name));

        let content = if Self::has_frontmatter(&skill.content) {
            skill.content.clone()
        } else {
            format!(
                "---\ndescription: {}\n---\n\n{}",
                skill.description, skill.content
            )
        };

        fs::write(&target_path, content).map_err(|e| e.to_string())?;

        Ok(())
    }

    fn has_frontmatter(content: &str) -> bool {
        content.starts_with("---")
    }

    pub fn remove(skill_name: &str) -> Result<(), String> {
        let cfg = Self::load_config();

        if cfg.install_targets.agents {
            Self::remove_from_agents(skill_name)?;
        }
        if cfg.install_targets.cursor {
            Self::remove_from_cursor(skill_name)?;
        }
        Ok(())
    }

    fn remove_from_agents(skill_name: &str) -> Result<(), String> {
        let target_dir = config::agents_skills_dir().join(skill_name);
        if target_dir.exists() {
            fs::remove_dir_all(&target_dir).map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    fn remove_from_cursor(skill_name: &str) -> Result<(), String> {
        let target_path = config::cursor_rules_dir().join(format!("{}.md", skill_name));
        if target_path.exists() {
            fs::remove_file(&target_path).map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    pub fn is_installed(skill_name: &str) -> bool {
        config::agents_skills_dir()
            .join(skill_name)
            .join("SKILL.md")
            .exists()
    }

    pub fn sync_all(skills: &[Skill]) -> Result<(), String> {
        for skill in skills {
            Self::install(skill)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_frontmatter_true() {
        let content = "---\ndescription: test\n---\n\n## Content";
        assert!(SkillInstaller::has_frontmatter(content));
    }

    #[test]
    fn test_has_frontmatter_false() {
        let content = "## Content\n\n* Bullet point";
        assert!(!SkillInstaller::has_frontmatter(content));
    }

    #[test]
    fn test_has_frontmatter_empty() {
        assert!(!SkillInstaller::has_frontmatter(""));
    }

    #[test]
    fn test_is_installed_not_installed() {
        // Should return false when skill is not installed
        assert!(!SkillInstaller::is_installed("nonexistent-skill"));
    }

    #[test]
    fn test_remove_nonexistent_skill() {
        // Should not fail when removing non-existent skill
        let result = SkillInstaller::remove("nonexistent-skill");
        assert!(result.is_ok());
    }

    #[test]
    fn test_remove_from_agents_nonexistent() {
        // Should not fail when removing from non-existent directory
        let result = SkillInstaller::remove_from_agents("test-skill");
        assert!(result.is_ok());
    }

    #[test]
    fn test_remove_from_cursor_nonexistent() {
        // Should not fail when removing from non-existent directory
        let result = SkillInstaller::remove_from_cursor("test-skill");
        assert!(result.is_ok());
    }
}
