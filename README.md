# Smart Skills

Agent skill management tool - manage and sync AI agent instructions for opencode, nvim, Cursor, and Claude Code.

## Features

- **Config-Based Skills**: Configure skill sources in JSON config
- **Per-Project Skills**: Add custom skills in configured directories
- **Global Config Support**: Use `~/.config/smart-skills/config.json` as template for new projects
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
smart-skills init
```

<details>
<summary><strong>How init works (click to expand)</strong></summary>

The `init` command follows this logic:

#### 1. No arguments provided

| Scenario | Behavior |
|----------|----------|
| Global config exists | Copies config to project, resolves source paths |
| Global config missing but global skills exist | Uses global skills dir as source, prompts for targets (interactive) or defaults to `agents` (non-interactive) |
| No global config, no global skills | Leaves sources empty, prompts for targets (interactive) or defaults to `agents` (non-interactive) |

#### 2. With arguments

| Argument | Behavior |
|----------|----------|
| `--skills-source <path>` | Override source, use global config targets (or default to `agents`) |
| `--targets a,c` | Use global config source, override targets |
| Both provided | Use CLI values for both |

#### Path Resolution

When global config has relative paths (e.g., `"path": "skills"`), they are resolved:
1. First check: relative to current project directory
2. Second check: relative to global config directory (`~/.config/smart-skills/`)

</details>

### Quick Examples

```bash
# Default initialization (uses global config if available)
smart-skills init

# Use custom skill source
smart-skills init --skills-source ./my-skills

# Use local skills/ directory
smart-skills init --skills-source skills

# Specify targets explicitly
smart-skills init --targets agents,cursor
smart-skills init --targets claude
```

**Targets:**
- `agents` - opencode/nvim (`.agents/skills/`)
- `cursor` - Cursor IDE (`.cursor/rules/`)
- `claude` - Claude Code (`.claude/rules/`)

---

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

---

## Global Config

<details>
<summary><strong>What is global config? (click to expand)</strong></summary>

Global config allows you to define a standard skill configuration that new projects can inherit.

### Location
```
~/.config/smart-skills/config.json
```

### Example
```json
{
  "skill_sources": [
    {
      "path": "~/my-skills",
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

### Benefits

1. **One-time setup**: Configure once, use in all projects
2. **Team consistency**: Share the config file via dotfiles
3. **Path resolution**: Relative paths are automatically resolved when initializing new projects

### How it works

- When you run `smart-skills init` without arguments, it copies global config to your project
- Relative paths in global config are resolved to absolute paths
- You can override either source or targets via CLI arguments

### Setup Global Config

```bash
# Create global config directory
mkdir -p ~/.config/smart-skills

# Create config with your preferred settings
cat > ~/.config/smart-skills/config.json << 'EOF'
{
  "skill_sources": [
    {
      "path": "~/my-skills",
      "priority": 10
    }
  ],
  "install_targets": {
    "agents": true,
    "cursor": true,
    "claude": false
  }
}
EOF

# Now any new project will use this config
cd ~/my-new-project
smart-skills init
```

</details>

---

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

---

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

---

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

---

## Structure

```
smart-skills/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── cli/
│   │   ├── commands.rs
│   │   ├── init.rs
│   │   ├── colors.rs
│   │   └── mod.rs
│   ├── skills/
│   │   ├── mod.rs
│   │   ├── loader.rs
│   │   └── installer.rs
│   └── config/
│       └── mod.rs
├── tests/
│   ├── init_tests.rs
│   └── ...
└── README.md
```
