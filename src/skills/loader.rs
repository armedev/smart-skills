use crate::config::{self, Config, SkillSource};
use crate::skills::{Skill, SkillSource as SkillSourceEnum};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
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

        let mut sources: Vec<(PathBuf, SkillSourceEnum)> = Vec::new();

        for source in &config.skill_sources {
            let path = PathBuf::from(&source.path);
            if path.exists() {
                sources.push((path, SkillSourceEnum::Project));
            }
        }

        let global_path = config::global_skills_dir();
        if global_path.exists() {
            sources.push((global_path, SkillSourceEnum::Global));
        }

        let project_path = config::project_skills_dir();
        if project_path.exists() && !config.skill_sources.iter().any(|s| s.path == "skills") {
            sources.push((project_path, SkillSourceEnum::Project));
        }

        for (path, source_enum) in sources {
            Self::load_skills_from_dir(path.as_path(), source_enum, &mut skills);
        }

        skills
    }

    fn load_config() -> Config {
        let project_config = config::project_config_path();
        if project_config.exists() {
            return Config::load(&project_config);
        }

        let global_config = config::global_config_path();
        if global_config.exists() {
            return Config::load(&global_config);
        }

        Config::default()
    }

    fn load_skills_from_dir(
        dir: &Path,
        source: SkillSourceEnum,
        skills: &mut HashMap<String, Skill>,
    ) {
        if !dir.exists() {
            return;
        }

        for entry in WalkDir::new(dir)
            .max_depth(2)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() && path.file_name().map(|n| n == "SKILL.md").unwrap_or(false) {
                if let Some(name) = path.parent().and_then(|p| p.file_name()) {
                    let skill_name = name.to_str().unwrap_or("").to_string();
                    if !skill_name.is_empty() && !skills.contains_key(&skill_name) {
                        if let Some(skill) =
                            Skill::from_file(skill_name.clone(), path.to_path_buf(), source.clone())
                        {
                            skills.insert(skill_name, skill);
                        }
                    }
                }
            }
        }
    }

    pub fn load_installed_skills() -> Vec<String> {
        let mut installed = Vec::new();
        let agents_dir = config::agents_skills_dir();

        if agents_dir.exists() {
            if let Ok(entries) = fs::read_dir(&agents_dir) {
                for entry in entries.filter_map(|e| e.ok()) {
                    let path = entry.path();
                    if path.is_dir() {
                        if let Some(name) = path.file_name() {
                            installed.push(name.to_str().unwrap_or("").to_string());
                        }
                    }
                }
            }
        }

        installed
    }

    pub fn validate_skills() -> ValidationResult {
        let skills = Self::load_available_skills();
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        if skills.is_empty() {
            errors.push(ValidationError {
                skill: "global".to_string(),
                message: "No skills found in any configured source".to_string(),
            });
            return ValidationResult {
                valid: false,
                errors,
                warnings,
            };
        }

        for (name, skill) in &skills {
            if skill.content.trim().is_empty() {
                errors.push(ValidationError {
                    skill: name.clone(),
                    message: "SKILL.md is empty".to_string(),
                });
            }

            let lines: Vec<&str> = skill.content.lines().collect();
            if lines.is_empty() || (lines.len() == 1 && lines[0].trim().is_empty()) {
                warnings.push(format!("Skill '{}' has minimal content", name));
            }

            if !skill.content.contains("## ") && !skill.content.contains("* ") {
                warnings.push(format!("Skill '{}' may not have proper formatting (expected ## headers or bullet points)", name));
            }
        }

        ValidationResult {
            valid: errors.is_empty(),
            errors,
            warnings,
        }
    }

    pub fn get_skill_sources() -> Vec<SkillSource> {
        let config = Self::load_config();
        if !config.skill_sources.is_empty() {
            return config.skill_sources;
        }

        let mut default_sources = Vec::new();

        let project_path = config::project_skills_dir();
        if project_path.exists() {
            default_sources.push(SkillSource {
                path: "skills".to_string(),
                priority: 10,
            });
        }

        let global_path = config::global_skills_dir();
        if global_path.exists() {
            default_sources.push(SkillSource {
                path: global_path.to_string_lossy().to_string(),
                priority: 5,
            });
        }

        default_sources
    }
}
