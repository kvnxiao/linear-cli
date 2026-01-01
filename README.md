# linear-cli

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

A fast, powerful command-line interface for [Linear](https://linear.app) built with Rust.

## Features

- **Full API Coverage** - Projects, issues, labels, teams, users, cycles, comments, documents
- **Git Integration** - Checkout branches for issues automatically
- **Local Sync** - Sync local project folders with Linear
- **Search** - Find issues and projects instantly
- **Fast** - Native Rust binary, no runtime dependencies

## Installation

### From Source

```bash
git clone https://github.com/Finesssee/linear-cli.git
cd linear-cli
cargo build --release
```

### Add to PATH

```bash
# Linux/macOS
sudo cp target/release/linear-cli /usr/local/bin/

# Windows (PowerShell as Admin)
Copy-Item target\release\linear-cli.exe C:\Windows\System32\
```

## Quick Start

```bash
# 1. Configure your API key (get one at https://linear.app/settings/api)
linear-cli config set-key lin_api_xxxxxxxxxxxxx

# 2. List your projects
linear-cli projects list

# 3. Create an issue
linear-cli issues create "Fix login bug" --team Engineering --priority 2
```

## Commands

| Command | Alias | Description |
|---------|-------|-------------|
| `projects` | `p` | Manage projects |
| `issues` | `i` | Manage issues |
| `labels` | `l` | Manage labels |
| `teams` | `t` | List and view teams |
| `users` | `u` | List and view users |
| `cycles` | `c` | Manage sprint cycles |
| `comments` | `cm` | Manage issue comments |
| `documents` | `d` | Manage documents |
| `search` | `s` | Search issues and projects |
| `sync` | `sy` | Sync local folders with Linear |
| `statuses` | `st` | View issue statuses |
| `git` | `g` | Git branch operations |
| `config` | - | CLI configuration |

## Usage Examples

### Projects

```bash
linear-cli p list                              # List all projects
linear-cli p list --archived                   # Include archived
linear-cli p get PROJECT_ID                    # View project details
linear-cli p create "Q1 Roadmap" -t Engineering
linear-cli p update PROJECT_ID --name "New Name"
linear-cli p delete PROJECT_ID --force
linear-cli p add-labels PROJECT_ID LABEL_ID
```

### Issues

```bash
linear-cli i list                              # List issues
linear-cli i list -t Engineering -s "In Progress"
linear-cli i get LIN-123                       # View issue details
linear-cli i create "Bug fix" -t Eng -p 1      # Priority: 1=urgent, 4=low
linear-cli i update LIN-123 -s Done
linear-cli i delete LIN-123 --force
```

### Labels

```bash
linear-cli l list                              # List project labels
linear-cli l list --type issue                 # List issue labels
linear-cli l create "Feature" --color "#10B981"
linear-cli l create "Bug" --type issue --color "#EF4444"
linear-cli l delete LABEL_ID --force
```

### Git Integration

```bash
linear-cli g checkout LIN-123                  # Create/checkout branch for issue
linear-cli g branch LIN-123                    # Show branch name for issue
linear-cli g create LIN-123                    # Create branch without checkout
linear-cli g checkout LIN-123 -b custom-branch # Use custom branch name
```

### Sync Local Folders

```bash
linear-cli sy status                           # Compare local folders with Linear
linear-cli sy push -t Engineering              # Create Linear projects for local folders
linear-cli sy push -t Engineering --dry-run    # Preview without creating
```

### Search

```bash
linear-cli s issues "authentication bug"
linear-cli s projects "backend" --limit 10
```

### Other Commands

```bash
# Teams
linear-cli t list
linear-cli t get TEAM_ID

# Users
linear-cli u list
linear-cli u get me

# Cycles
linear-cli c list -t Engineering
linear-cli c current -t Engineering

# Comments
linear-cli cm list ISSUE_ID
linear-cli cm create ISSUE_ID -b "This is a comment"

# Documents
linear-cli d list
linear-cli d get DOC_ID
linear-cli d create "Doc Title" -p PROJECT_ID

# Statuses
linear-cli st list -t Engineering
linear-cli st get "In Progress" -t Engineering

# Config
linear-cli config set-key YOUR_API_KEY
linear-cli config show
```

## Configuration

Configuration is stored at:
- **Linux/macOS:** `~/.config/linear-cli/config.toml`
- **Windows:** `%APPDATA%\linear-cli\config.toml`

## Claude Code Integration

Add linear-cli to your Claude Code instructions:

```bash
mkdir -p ~/.claude && echo "Linear CLI: Use \`linear-cli\` for Linear.app operations instead of MCP tools. Commands: projects (p), issues (i), labels (l), teams (t), users (u), cycles (c), comments (cm), documents (d), search (s), sync (sy), statuses (st), git (g), config. Examples: \`linear-cli p list\`, \`linear-cli i create \"Title\" --team ENG\`, \`linear-cli l create \"Label\" --type project --color \"#FF5733\"\`, \`linear-cli g checkout LIN-123\`, \`linear-cli sy status\`" >> ~/.claude/CLAUDE.md
```

## Comparison with Official CLI

| Feature | `@linear/cli` | `linear-cli` |
|---------|---------------|--------------|
| Last updated | 2021 | 2025 |
| Create issues | ✓ | ✓ |
| Git checkout | ✓ | ✓ |
| List/manage projects | ✗ | ✓ |
| CRUD for all resources | ✗ | ✓ |
| Search | ✗ | ✓ |
| Local folder sync | ✗ | ✓ |
| Issue statuses | ✗ | ✓ |
| Documents | ✗ | ✓ |

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

[MIT](LICENSE)
