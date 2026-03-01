# Smart Skills

Agent skill management tool - manage and sync AI agent instructions for opencode, nvim, and Cursor.

## Features

- **Config-Based Skills**: Configure skill sources in JSON config
- **Per-Project Skills**: Add custom skills in configured directories
- **Global Skills**: Support for `~/.config/smart-skills/skills/`
- **Skill Validation**: Validate skill structure and content
- **Multi-Platform**: Installs skills for opencode, nvim, and Cursor

## Installation

```bash
cargo build --release
cargo install --path .
```

Or from source:

```bash
cargo build
```

## Usage

### Initialize a project

```bash
smart-skills init                                    # Use global skills (~/.config/smart-skills/skills/)
smart-skills init --skills-source ./my-skills       # Use custom skill source
smart-skills init --skills-source skills            # Use local skills/ directory
```

This creates:
- `.smart-skills/config.json` - Project config with skill sources
- `.agents/skills/` - Skills for opencode/nvim
- `.cursor/rules/` - Skills for Cursor

### Global Skills

By default, init uses skills from:
- `~/.config/smart-skills/skills/` (Linux/macOS XDG)
- `~/Library/Application Support/smart-skills/skills/` (macOS legacy)

### Add skills

```bash
smart-skills add planning
smart-skills add planning execution  # Add multiple
smart-skills add                     # Show available skills
```

### Remove skills

```bash
smart-skills remove planning
smart-skills remove                 # Show installed skills
```

### List skills

```bash
smart-skills list
```

### Sync skills

```bash
smart-skills sync
```

### Check status (with validation)

```bash
smart-skills status
```

### Config management

```bash
smart-skills config              # Show current config
smart-skills set-sources ./my-skills /global/skills  # Set skill sources
```

### Clear all skills

```bash
smart-skills clear
```

## Configuration

### Project Config (`.smart-skills/config.json`)

```json
{
  "skill_sources": [
    {
      "path": "skills",
      "priority": 10
    }
  ],
  "install_targets": {
    "agents": true,
    "cursor": true
  }
}
```

### Skill Sources

- **path**: Path to skill directory (relative or absolute)
- **priority**: Higher priority sources are checked first (10 = highest)

### Global Config

You can also set up global skills at `~/.config/smart-skills/config.json`

## Skill Structure

Skills should be in directories with `SKILL.md` files:

```
skills/
├── planning/
│   └── SKILL.md
├── code-review/
│   └── SKILL.md
└── ...
```

### Validating Skills

Run `smart-skills status` to validate:
- Empty skill files
- Missing content
- Proper formatting (## headers or bullet points)

## Priority Order

Skills are loaded in this order:

1. Project config skill sources (by priority)
2. Global skills (`~/.config/smart-skills/skills/`)

## Adding Custom Skills

1. Create a skill directory:
   ```bash
   mkdir -p skills/my-custom-skill
   ```

2. Add a `SKILL.md` file:
   ```markdown
   ## My Custom Skill

   * Follow these guidelines
   * For your team
   ```

3. Initialize or sync:
   ```bash
   smart-skills init
   # or
   smart-skills sync
   ```

## Structure

```
smart-skills/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── cli/
│   │   ├── commands.rs
│   │   ├── picker.rs
│   │   └── mod.rs
│   ├── skills/
│   │   ├── mod.rs
│   │   ├── loader.rs
│   │   └── installer.rs
│   └── config/
│       └── mod.rs
└── README.md
```
