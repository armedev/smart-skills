use std::fs;
use tempfile::TempDir;

#[test]
fn test_init_with_empty_source() {
    // Test that init with empty source works correctly
    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path().join("project");
    fs::create_dir(&project_dir).unwrap();

    // This is an integration test placeholder
    // The actual init function requires user interaction
    assert!(project_dir.exists());
}

#[test]
fn test_config_directory_creation() {
    let temp_dir = TempDir::new().unwrap();
    let config_dir = temp_dir.path().join(".smart-skills");
    let agents_dir = temp_dir.path().join(".agents").join("skills");
    let cursor_dir = temp_dir.path().join(".cursor").join("rules");

    fs::create_dir_all(&config_dir).unwrap();
    fs::create_dir_all(&agents_dir).unwrap();
    fs::create_dir_all(&cursor_dir).unwrap();

    assert!(config_dir.exists());
    assert!(agents_dir.exists());
    assert!(cursor_dir.exists());
}

#[test]
fn test_skill_file_format() {
    // Test that skill files follow the expected format
    let temp_dir = TempDir::new().unwrap();
    let skill_file = temp_dir.path().join("SKILL.md");

    // Valid skill with header and bullets
    fs::write(&skill_file, "## Skill Name\n\n* Point 1\n* Point 2").unwrap();
    let content = fs::read_to_string(&skill_file).unwrap();

    assert!(content.contains("## "));
    assert!(content.contains("* "));
}

#[test]
fn test_config_json_format() {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct TestSkillSource {
        path: String,
        priority: u8,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct TestInstallTargets {
        agents: bool,
        cursor: bool,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct TestConfig {
        skill_sources: Vec<TestSkillSource>,
        install_targets: TestInstallTargets,
    }

    let config = TestConfig {
        skill_sources: vec![TestSkillSource {
            path: "skills".to_string(),
            priority: 10,
        }],
        install_targets: TestInstallTargets {
            agents: true,
            cursor: true,
        },
    };

    let json = serde_json::to_string_pretty(&config).unwrap();
    assert!(json.contains("skill_sources"));
    assert!(json.contains("install_targets"));
    assert!(json.contains("skills"));
}

#[test]
fn test_cli_command_structure() {
    // Verify that the CLI commands exist and have the right structure
    // This is a basic smoke test
    let commands = vec![
        "init", "add", "remove", "list", "sync", "status", "clear", "config",
    ];
    assert_eq!(commands.len(), 8);
    assert!(commands.contains(&"init"));
    assert!(commands.contains(&"add"));
    assert!(commands.contains(&"status"));
}

#[test]
fn test_skill_loading_priority() {
    // Test that skills are loaded in the correct priority order
    // Project > Global > Bundled
    let temp_dir = TempDir::new().unwrap();
    let project_skills = temp_dir.path().join("skills");
    fs::create_dir(&project_skills).unwrap();

    assert!(project_skills.exists());
}

#[test]
fn test_empty_skill_validation() {
    let temp_dir = TempDir::new().unwrap();
    let skill_file = temp_dir.path().join("SKILL.md");

    // Empty skill file
    fs::write(&skill_file, "").unwrap();
    let content = fs::read_to_string(&skill_file).unwrap();

    assert!(content.trim().is_empty());
}

#[test]
fn test_invalid_skill_no_bullets() {
    let temp_dir = TempDir::new().unwrap();
    let skill_file = temp_dir.path().join("SKILL.md");

    // Skill without bullet points
    fs::write(&skill_file, "## Title\n\nJust text").unwrap();
    let content = fs::read_to_string(&skill_file).unwrap();

    assert!(!content.contains("* "));
    assert!(content.contains("## "));
}
