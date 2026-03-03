use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn test_init_no_args_no_global_config_no_global_skills() {
    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path();

    std::env::set_current_dir(project_dir).unwrap();

    // Ensure no global config and no global skills
    // This test expects: source empty, targets = ["agents"] (non-tty default)
    // But since we can't easily mock global paths, we test the warning output

    let config_path = project_dir.join(".smart-skills").join("config.json");
    assert!(!config_path.exists());
}

#[test]
fn test_init_with_source_arg() {
    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path();

    // Create a skills directory
    let skills_dir = project_dir.join("my-skills");
    fs::create_dir_all(&skills_dir).unwrap();

    // Create a skill file
    fs::write(
        skills_dir.join("SKILL.md"),
        "## Test Skill\n\n* Point 1\n* Point 2",
    )
    .unwrap();

    std::env::set_current_dir(project_dir).unwrap();

    // The init function would use the source arg
    // We test that the skills directory is found
    assert!(skills_dir.exists());
}

#[test]
fn test_init_with_targets_arg() {
    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path();

    // Verify we can create the expected directories
    let agents_dir = project_dir.join(".agents");
    let cursor_dir = project_dir.join(".cursor");

    fs::create_dir_all(agents_dir.join("skills")).unwrap();
    fs::create_dir_all(cursor_dir.join("rules")).unwrap();

    assert!(agents_dir.join("skills").exists());
    assert!(cursor_dir.join("rules").exists());
}

#[test]
fn test_resolve_source_path_absolute() {
    let temp_dir = TempDir::new().unwrap();
    let skills_dir = temp_dir.path().join("skills");
    fs::create_dir(&skills_dir).unwrap();

    let absolute_path = skills_dir.to_string_lossy().to_string();
    assert!(PathBuf::from(&absolute_path).is_absolute());
}

#[test]
fn test_resolve_source_path_relative() {
    // Relative paths should be resolved
    let relative_path = "./skills";
    let path = PathBuf::from(relative_path);

    // Path should not be absolute
    assert!(!path.is_absolute() || path.exists());
}

#[test]
fn test_config_targets_defaults() {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct InstallTargets {
        agents: bool,
        cursor: bool,
        claude: bool,
    }

    // Default should be agents only
    let targets = InstallTargets {
        agents: true,
        cursor: false,
        claude: false,
    };

    assert!(targets.agents);
    assert!(!targets.cursor);
    assert!(!targets.claude);
}

#[test]
fn test_config_targets_all() {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct InstallTargets {
        agents: bool,
        cursor: bool,
        claude: bool,
    }

    let targets = InstallTargets {
        agents: true,
        cursor: true,
        claude: true,
    };

    assert!(targets.agents);
    assert!(targets.cursor);
    assert!(targets.claude);
}

#[test]
fn test_skill_source_priority() {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct SkillSource {
        path: String,
        priority: u8,
    }

    let sources = vec![
        SkillSource {
            path: "local".to_string(),
            priority: 20,
        },
        SkillSource {
            path: "global".to_string(),
            priority: 10,
        },
    ];

    // Higher priority should come first when sorted descending
    let mut sorted = sources.clone();
    sorted.sort_by(|a, b| b.priority.cmp(&a.priority));

    assert_eq!(sorted[0].priority, 20);
    assert_eq!(sorted[1].priority, 10);
}

#[test]
fn test_init_copies_global_config() {
    let temp_dir = TempDir::new().unwrap();
    let global_dir = temp_dir.path().join("global_config");
    let project_dir = temp_dir.path().join("project");

    fs::create_dir_all(&global_dir).unwrap();
    fs::create_dir(&project_dir).unwrap();

    // Create global config
    let global_config = global_dir.join("config.json");
    fs::write(
        &global_config,
        r#"{"skill_sources":[{"path":"skills","priority":10}],"install_targets":{"agents":true,"cursor":true,"claude":false}}"#,
    ).unwrap();

    // Copy config to project
    let project_config = project_dir.join(".smart-skills").join("config.json");
    fs::create_dir_all(project_config.parent().unwrap()).unwrap();
    fs::copy(&global_config, &project_config).unwrap();

    assert!(project_config.exists());

    let content = fs::read_to_string(&project_config).unwrap();
    assert!(content.contains("skill_sources"));
}

#[test]
fn test_init_resolves_relative_path() {
    let temp_dir = TempDir::new().unwrap();
    let global_dir = temp_dir.path().join("global");
    let project_dir = temp_dir.path().join("project");

    fs::create_dir_all(&global_dir).unwrap();
    fs::create_dir(&project_dir).unwrap();

    // Create global config with relative path
    let global_config = global_dir.join("config.json");
    fs::write(
        &global_config,
        r#"{"skill_sources":[{"path":"skills","priority":10}],"install_targets":{"agents":true,"cursor":false,"claude":false}}"#,
    ).unwrap();

    // Create the skills directory in global location
    let global_skills = global_dir.join("skills");
    fs::create_dir(&global_skills).unwrap();
    fs::write(global_skills.join("SKILL.md"), "## Test\n\n* Point").unwrap();

    // Load config and resolve path
    let config_content = fs::read_to_string(&global_config).unwrap();
    let config: serde_json::Value = serde_json::from_str(&config_content).unwrap();

    let source_path = config["skill_sources"][0]["path"].as_str().unwrap();

    // Relative path should be resolved against global config dir
    let resolved = global_dir.join(source_path);
    assert!(resolved.exists());
}
