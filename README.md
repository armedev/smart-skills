# Smart Skills

Agent skill management tool - manage and sync AI agent instructions for opencode, nvim, Cursor, and Claude Code.

## Features

- **Validates skills for agent support**: Ensures proper format so your agents understand skills
- **Built with Rust**: Blazing fast - no waiting for npm
- **Zero dependencies**: Standalone binary - no Node.js, no npm chain
- **Open source**: Inspect every line yourself
- **Your skills, your machine**: No marketplace, no third-party auto-sync
- **Per-project + global config**: Team defaults via global, project overrides locally
- **Simple sync**: `smart-skills sync` to update skills

## Installation

### No npm/Node.js required!

smart-skills is a standalone binary - no dependencies.

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

## Security

**We validate FORMAT. You validate CONTENT.**

### What we do:
- Check skills have proper format (## headers, bullet points)
- Verify SKILL.md is not empty
- Parse frontmatter for agent compatibility

### What YOU must do:
- **Manually review every skill before adding**
- **Use a raw text editor** - never trust rendered markdown
- Check for hidden comments, escape sequences, malicious code

### Why this matters:
Your AI agent has permissions to:
- Run shell commands
- Access/modify files
- Read environment variables (API keys!)

A malicious skill = instant breach.

**Never add a skill you haven't reviewed yourself.**

## Manual Review Required

**Don't trust rendered markdown. Use a real editor.**

```bash
# Find a skill you want
git clone https://github.com/awesome/skills.git

# REVIEW IT FIRST - use raw editor
vim skills/my-skill/SKILL.md

# Only if safe, copy to your source
cp -r skills/my-skill ./my-skills/

# Now add it
smart-skills add my-skill
```

Renderers can hide:
- HTML comments `<!-- malicious -->`
- Escape sequences
- Code that looks safe but isn't

**Your agent runs this. Verify it yourself.**

## Why not skills.sh?

- npm supply chain attacks
- Marketplace = trusting strangers
- Their "add" command fetches from internet = security risk
- We don't do automatic third-party

## Quick Start

```bash
# Install (no npm needed!)
cargo install smart-skills
# or: brew install smart-skills

# Initialize a project
smart-skills init

# Add skills
smart-skills add planning

# Sync to update
smart-skills sync
```

**For teams:**
1. Put skills in a shared location
2. Run `init` with your team's skill source
3. Team members run `sync` to get latest

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
| Global config exists | Uses global config as template, resolves source paths to absolute |
| Global config missing but global skills exist | Uses global skills dir as source, prompts for targets (interactive) or defaults to `agents` (non-interactive) |
| No global config, no global skills | Leaves sources empty, prompts for targets (interactive) or defaults to `agents` (non-interactive) |

#### 2. With arguments

| Argument | Behavior |
|----------|----------|
| `--skills-source <path>` | Use this skill source directory |
| `--targets a,c` | Use these targets |
| Both provided | Use both specified values |

#### Path Resolution

Skill source paths are resolved relative to current project directory.

</details>

### Quick Examples

```bash
# Default initialization
smart-skills init

# Use custom skill source
smart-skills init --skills-source ./my-skills

# Specify targets explicitly
smart-skills init --targets agents,cursor
```

**Targets:**
- `agents` - opencode/nvim (`.agents/skills/`)
- `cursor` - Cursor IDE (`.cursor/rules/`)
- `claude` - Claude Code (`.claude/rules/`)

---

### Add skills

> Requires: `smart-skills init` to have been run first

```bash
smart-skills add planning
smart-skills add planning execution  # Add multiple
smart-skills add                     # Show available skills (requires init)
```

### Remove skills

> Requires: `smart-skills init` to have been run first

```bash
smart-skills remove planning
smart-skills remove                 # Show installed skills (requires init)
```

### List skills

> Requires: `smart-skills init` to have been run first

```bash
smart-skills list
```

### Sync skills

> Requires: `smart-skills init` to have been run first

```bash
smart-skills sync
```

### Check status

> Requires: `smart-skills init` to have been run first

```bash
smart-skills status
```

### Config management

> Requires: `smart-skills init` to have been run first

```bash
smart-skills config              # Show current config
smart-skills set-sources ./my-skills  # Set skill sources
```

### Clear all skills

> Requires: `smart-skills init` to have been run first

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
    "cursor": false,
    "claude": false
  }
}
```

### Benefits

1. **One-time setup**: Configure once, use in all projects
2. **Team consistency**: Share the config file via dotfiles
3. **Path resolution**: Relative paths are automatically resolved when initializing new projects

### How it works

- When you run `smart-skills init` without arguments, it uses global config as a template for your project
- Relative paths in global config are resolved to absolute paths (relative to global config directory)
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
    "cursor": false,
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
    "cursor": false,
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

### SKILL.md Format

```markdown
---
name: planning
description: Plan before coding
---

## Planning

* Always understand the problem first
* Break down into smaller tasks
* Consider edge cases
```

The frontmatter is optional but helps agents understand skills better.

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

3. Add it:
   ```bash
   smart-skills add my-custom-skill
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
