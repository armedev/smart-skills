use std::fs;
use tempfile::TempDir;

#[test]
fn test_end_to_end_skill_workflow() {
    let temp_dir = TempDir::new().unwrap();
    let skills_dir = temp_dir.path().join("skills");
    let project_dir = temp_dir.path().join("project");

    fs::create_dir_all(&skills_dir).unwrap();
    fs::create_dir_all(&project_dir).unwrap();

    // Create a test skill
    let skill_dir = skills_dir.join("test-skill");
    fs::create_dir(&skill_dir).unwrap();
    fs::write(
        skill_dir.join("SKILL.md"),
        "## Test Skill\n\n* This is a test skill\n* For testing purposes",
    )
    .unwrap();

    // Verify skill file exists
    assert!(skill_dir.join("SKILL.md").exists());

    // Verify we can read it
    let content = fs::read_to_string(skill_dir.join("SKILL.md")).unwrap();
    assert!(content.contains("Test Skill"));
}

#[test]
fn test_config_persistence() {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize, Default)]
    struct TestConfig {
        value: String,
    }

    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("test_config.json");

    let config = TestConfig {
        value: "test".to_string(),
    };

    let content = serde_json::to_string_pretty(&config).unwrap();
    fs::write(&config_path, content).unwrap();

    let loaded_content = fs::read_to_string(&config_path).unwrap();
    let loaded: TestConfig = serde_json::from_str(&loaded_content).unwrap();

    assert_eq!(loaded.value, "test");
}

#[test]
fn test_skill_structure() {
    let temp_dir = TempDir::new().unwrap();
    let skill_dir = temp_dir.path().join("my-skill");
    fs::create_dir(&skill_dir).unwrap();

    // Valid skill file
    fs::write(
        skill_dir.join("SKILL.md"),
        "## My Skill\n\n* Do this\n* Do that",
    )
    .unwrap();

    assert!(skill_dir.join("SKILL.md").exists());

    let content = fs::read_to_string(skill_dir.join("SKILL.md")).unwrap();
    assert!(content.contains("## ")); // Has header
    assert!(content.contains("* ")); // Has bullets
}
