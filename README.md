# smart-skills

CLI tool to manage AI agent skills for opencode, Cursor, and Claude Code.

Built for my own use. Open for contributions.

## Install

```bash
cargo install smart-skills
```

## Quick Start

```bash
# Create global config
smart-skills init

# Add your skill source
smart-skills set-sources ./skills

# Install skills
smart-skills add planning
smart-skills add code-review

# Sync installed skills
smart-skills sync
```

## Examples

See [`examples/`](examples/) for sample skills and config:

```
examples/
├── config.json          # Example config
└── skills/
    ├── planning/
    │   └── SKILL.md
    └── code-review/
        └── SKILL.md
```

## Commands

| Command | Description |
|---------|-------------|
| `add <skill>` | Install a skill |
| `add <skill> --targets cursor,claude` | Install to specific targets |
| `add` | List available skills |
| `remove <skill>` | Remove a skill |
| `remove <skill> --targets cursor` | Remove from specific targets |
| `list` | List available and installed |
| `sync` | Re-sync installed skills |
| `sync --targets cursor` | Sync to specific targets |
| `sync --remove-stale` | Remove stale skills |
| `clear` | Remove all installed |
| `clear --targets claude` | Clear from specific targets |
| `status` | Show status and validation |
| `config` | Show configuration |
| `init` | Create global config |
| `set-sources <path>` | Add skill sources (appends) |
| `set-sources <path> --overwrite` | Replace all sources |

## Override Targets

Use `--targets` to override global config for a single command:

```bash
smart-skills add planning --targets cursor,claude
smart-skills sync --targets agents
smart-skills clear --targets claude
```

Valid targets: `agents`, `cursor`, `claude`

## Config

Global config: `~/.config/smart-skills/config.json`

### Path Resolution

Skill source paths in config are resolved relative to the config directory (`~/.config/smart-skills/`):

```json
{
  "skill_sources": [
    { "path": "./skills", "priority": 10 },
    { "path": "../my-shared-skills", "priority": 5 }
  ]
}
```

This allows skills to live alongside your config rather than relative to pwd.

```json
{
  "skill_sources": [{ "path": "~/my-skills", "priority": 10 }],
  "install_targets": { "agents": true, "cursor": false, "claude": false }
}
```

## Targets

- `agents` → `.agents/skills/` (opencode, nvim)
- `cursor` → `.cursor/rules/` (Cursor)
- `claude` → `.claude/rules/` (Claude Code)

## Skill Format

```
skills/
├── planning/
│   └── SKILL.md
```

SKILL.md:
```markdown
---
name: planning
description: Plan before coding
---

## Planning

* Always understand the problem first
* Break down into smaller tasks
```

Frontmatter is optional. If missing, first bullet point becomes the description.

## Security

We validate **FORMAT** (file exists, not empty, has ## headers or bullets).

You validate **CONTENT** (review skills manually before adding).

Your agent can run shell commands and modify files. Don't add skills you haven't reviewed.

## No Marketplace

No automatic fetching from the internet. Your skills stay on your machine.
