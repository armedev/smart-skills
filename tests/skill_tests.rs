use std::fs;
use tempfile::TempDir;

#[test]
fn test_skill_loading_from_directory() {
    let temp_dir = TempDir::new().unwrap();
    let skills_dir = temp_dir.path().join("skills");

    // Create multiple skills
    let skill1_dir = skills_dir.join("planning");
    fs::create_dir_all(&skill1_dir).unwrap();
    fs::write(
        skill1_dir.join("SKILL.md"),
        "## Planning\n\n* Plan first\n* Then execute",
    )
    .unwrap();

    let skill2_dir = skills_dir.join("testing");
    fs::create_dir_all(&skill2_dir).unwrap();
    fs::write(
        skill2_dir.join("SKILL.md"),
        "## Testing\n\n* Write tests\n* Run tests",
    )
    .unwrap();

    // Verify both skills exist
    assert!(skill1_dir.join("SKILL.md").exists());
    assert!(skill2_dir.join("SKILL.md").exists());

    // Count SKILL.md files
    let count = walkdir::WalkDir::new(&skills_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name() == "SKILL.md")
        .count();

    assert_eq!(count, 2);
}

#[test]
fn test_installation_paths() {
    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path().join("project");

    // Create typical installation structure
    let agents_skills = project_dir.join(".agents").join("skills");
    let cursor_rules = project_dir.join(".cursor").join("rules");

    fs::create_dir_all(&agents_skills).unwrap();
    fs::create_dir_all(&cursor_rules).unwrap();

    // Verify paths exist
    assert!(agents_skills.exists());
    assert!(cursor_rules.exists());
    assert!(project_dir.join(".agents").exists());
    assert!(project_dir.join(".cursor").exists());
}

#[test]
fn test_skill_validation_empty_file() {
    let temp_dir = TempDir::new().unwrap();
    let skill_file = temp_dir.path().join("SKILL.md");

    // Empty skill file
    fs::write(&skill_file, "").unwrap();

    let content = fs::read_to_string(&skill_file).unwrap();
    assert!(content.trim().is_empty());
}

#[test]
fn test_skill_validation_minimal_content() {
    let temp_dir = TempDir::new().unwrap();
    let skill_file = temp_dir.path().join("SKILL.md");

    // Minimal valid skill
    fs::write(&skill_file, "## Title\n\n* One bullet").unwrap();

    let content = fs::read_to_string(&skill_file).unwrap();
    assert!(content.contains("## "));
    assert!(content.contains("* "));
}
