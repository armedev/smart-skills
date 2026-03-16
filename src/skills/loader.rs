use crate::config::{self, Config, SkillSource, SKILL_FILE};
use crate::skills::{Skill, SkillSource as SkillSourceEnum};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<String>,
}

#[derive(Debug)]
pub struct ValidationError {
    pub skill: String,
    pub message: String,
}

pub struct SkillLoader;

impl SkillLoader {
    pub fn load_available_skills() -> HashMap<String, Skill> {
        let mut skills = HashMap::new();
        let config = Self::load_config();

        for source in &config.skill_sources {
            let path = std::path::PathBuf::from(&source.path);
            if path.exists() {
                Self::load_from_dir(path.as_path(), &mut skills);
            }
        }
        skills
    }

    fn load_config() -> Config {
        let path = config::global_config_path();
        if path.exists() {
            Config::load(&path)
        } else {
            Config::default()
        }
    }

    fn load_from_dir(dir: &Path, skills: &mut HashMap<String, Skill>) {
        if !dir.exists() {
            return;
        }

        for entry in WalkDir::new(dir)
            .max_depth(2)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() && path.file_name().map(|n| n == SKILL_FILE).unwrap_or(false) {
                if let Some(name) = path.parent().and_then(|p| p.file_name()) {
                    let skill_name = name.to_str().unwrap_or("");
                    if !skill_name.is_empty() && !skills.contains_key(skill_name) {
                        if let Some(skill) = Skill::from_file(
                            skill_name.to_string(),
                            path.to_path_buf(),
                            SkillSourceEnum::Project,
                        ) {
                            skills.insert(skill_name.to_string(), skill);
                        }
                    }
                }
            }
        }
    }

    pub fn load_installed_skills() -> Vec<String> {
        let dir = config::agents_skills_dir();
        if !dir.exists() {
            return vec![];
        }

        fs::read_dir(dir)
            .ok()
            .map(|entries| {
                entries
                    .filter_map(|e| e.ok())
                    .filter(|e| e.path().is_dir())
                    .filter_map(|e| e.file_name().to_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn validate_skills() -> ValidationResult {
        let skills = Self::load_available_skills();

        if skills.is_empty() {
            return ValidationResult {
                valid: false,
                errors: vec![ValidationError {
                    skill: "global".to_string(),
                    message: "No skills found in any configured source".to_string(),
                }],
                warnings: vec![],
            };
        }

        let errors: Vec<_> = skills
            .iter()
            .filter(|(_, s)| s.content.trim().is_empty())
            .map(|(name, _)| ValidationError {
                skill: name.clone(),
                message: "SKILL.md is empty".to_string(),
            })
            .collect();

        let warnings: Vec<_> = skills
            .iter()
            .filter(|(_, s)| !s.content.contains("## ") && !s.content.contains("* "))
            .map(|(name, _)| format!("Skill '{}' may not have proper formatting", name))
            .collect();

        ValidationResult {
            valid: errors.is_empty(),
            errors,
            warnings,
        }
    }

    pub fn get_skill_sources() -> Vec<SkillSource> {
        Self::load_config().skill_sources
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_installed_empty() {
        assert!(SkillLoader::load_installed_skills().is_empty());
    }

    #[test]
    fn test_load_from_dir_nonexistent() {
        let mut skills = HashMap::new();
        SkillLoader::load_from_dir(Path::new("/nonexistent"), &mut skills);
        assert!(skills.is_empty());
    }

    #[test]
    fn test_validation_structure() {
        let result = SkillLoader::validate_skills();
        assert_eq!(result.errors.is_empty(), result.valid);
    }
}
