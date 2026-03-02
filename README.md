# Smart Skills

Agent skill management tool - manage and sync AI agent instructions for opencode, nvim, Cursor, and Claude Code.

## Features

- **Config-Based Skills**: Configure skill sources in JSON config
- **Per-Project Skills**: Add custom skills in configured directories
- **Global Skills**: Support for `~/.config/smart-skills/skills/`
- **Skill Validation**: Validate skill structure and content
- **Multi-Platform**: Installs skills for opencode, nvim, Cursor, and Claude Code

## Installation

### Via Cargo (recommended)

```bash
cargo install smart-skills
```

### Via Homebrew

```bash
brew tap armedev/tap
brew install smart-skills
```

### Via Install Script

```bash
curl -sL https://raw.githubusercontent.com/armedev/smart-skills/main/install.sh | bash
```

### From Source

```bash
cargo build --release
cargo install --path .
```

---

## Usage

### Initialize a project

```bash
smart-skills init                                    # Default: agents only (non-interactive)
smart-skills init --skills-source ./my-skills       # Use custom skill source
smart-skills init --skills-source skills            # Use local skills/ directory
smart-skills init --targets agents,cursor           # Specify targets explicitly
smart-skills init --targets claude                 # Only Claude Code
```

**Targets:**
- `agents` - opencode/nvim (`.agents/skills/`)
- `cursor` - Cursor IDE (`.cursor/rules/`)
- `claude` - Claude Code (`.claude/rules/`)

**Behavior:**
- In interactive terminal: prompts you to select targets
- In non-interactive (piped): defaults to `agents` only
- With `--targets`: uses specified targets non-interactively

### Global Skills

By default, init uses skills from:
- `~/.config/smart-skills/skills/` (all platforms)

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
    "cursor": true,
    "claude": false
  }
}
```

**Install Targets:**
- `agents`: Enable/disable `.agents/skills/` installation
- `cursor`: Enable/disable `.cursor/rules/` installation
- `claude`: Enable/disable `.claude/rules/` installation

Edit this file and run `smart-skills sync` to apply changes.

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
