# Linear CLI

A powerful command-line interface for [Linear.app](https://linear.app) built with Rust. Manage your projects, issues, labels, teams, and more directly from the terminal.

## Installation

### Prerequisites

- [Rust](https://rustup.rs/) (1.70 or later)
- A Linear API key ([get one here](https://linear.app/settings/api))

### Build from Source

```bash
git clone https://github.com/your-org/linear-cli.git
cd linear-cli
cargo build --release
```

The binary will be available at `target/release/linear-cli` (or `linear-cli.exe` on Windows).

Optionally, add it to your PATH:

```bash
# Linux/macOS
cp target/release/linear-cli ~/.local/bin/

# Windows (PowerShell)
Copy-Item target\release\linear-cli.exe C:\Users\$env:USERNAME\bin\
```

## Configuration

Before using the CLI, you must configure your Linear API key:

```bash
linear config set-key lin_api_XXXXXXXXXXXX
```

To view your current configuration:

```bash
linear config show
```

Configuration is stored in:
- **Linux/macOS**: `~/.config/linear-cli/config.toml`
- **Windows**: `%APPDATA%\linear-cli\config.toml`

## Usage Examples

### Projects

```bash
# List all projects
linear projects list
linear p ls                    # Short form

# Include archived projects
linear projects list --archived

# Get project details
linear projects get PROJECT_ID

# Create a new project
linear projects create "My Project" --team "Engineering"
linear projects create "My Project" -t "Engineering" -d "Description" -c "#FF5500"

# Update a project
linear projects update PROJECT_ID --name "New Name"
linear projects update PROJECT_ID -d "Updated description" -c "#00FF00"

# Delete a project
linear projects delete PROJECT_ID --force

# Add labels to a project
linear projects add-labels PROJECT_ID label-id-1 label-id-2
```

### Issues

```bash
# List issues
linear issues list
linear i ls                    # Short form

# Filter issues by team, project, or state
linear issues list --team "Engineering"
linear issues list --project "Q1 Roadmap"
linear issues list --state "In Progress"

# Get issue details
linear issues get ISSUE_ID

# Create a new issue
linear issues create "Fix login bug" --team "Engineering"
linear issues create "Add feature" -t "Product" -d "Detailed description" -p "High"

# Update an issue
linear issues update ISSUE_ID --state "Done"
linear issues update ISSUE_ID --assignee "john@example.com"

# Delete an issue
linear issues delete ISSUE_ID --force
```

### Labels

```bash
# List project labels (default)
linear labels list
linear l ls                    # Short form

# List issue labels
linear labels list --type issue

# Create a project label
linear labels create "Priority" --color "#FF0000"

# Create an issue label
linear labels create "Bug" --type issue --color "#DC2626"

# Create a child label (grouped under parent)
linear labels create "Critical" --parent PARENT_LABEL_ID

# Delete a label
linear labels delete LABEL_ID --force
linear labels delete LABEL_ID --type issue --force
```

### Teams

```bash
# List all teams
linear teams list
linear teams ls

# Get team details
linear teams get TEAM_ID
```

### Users

```bash
# List users in workspace
linear users list

# Get user details
linear users get USER_ID
linear users get me              # Get your own info
```

### Cycles

```bash
# List cycles for a team
linear cycles list --team "Engineering"

# Get current cycle
linear cycles list --team "Engineering" --type current

# Get previous/next cycle
linear cycles list --team "Engineering" --type previous
linear cycles list --team "Engineering" --type next
```

### Comments

```bash
# List comments on an issue
linear comments list ISSUE_ID

# Add a comment to an issue
linear comments create ISSUE_ID "This is my comment"

# Reply to a comment
linear comments create ISSUE_ID "Reply text" --parent COMMENT_ID
```

### Documents

```bash
# List documents
linear documents list

# Get document details
linear documents get DOCUMENT_ID

# Create a document
linear documents create "Document Title" --project "My Project"
linear documents create "Title" -p "Project" --content "# Markdown content"

# Update a document
linear documents update DOCUMENT_ID --title "New Title"
```

### Search

```bash
# Search issues
linear search "authentication bug"

# Search with filters
linear search "login" --team "Engineering"
```

### Sync

```bash
# Sync local data with Linear
linear sync

# Sync specific resources
linear sync --projects
linear sync --issues
```

## Command Reference

| Command | Alias | Description |
|---------|-------|-------------|
| `projects list` | `p ls` | List all projects |
| `projects get <id>` | `p get` | Get project details |
| `projects create <name>` | `p create` | Create a new project |
| `projects update <id>` | `p update` | Update a project |
| `projects delete <id>` | `p delete` | Delete a project |
| `projects add-labels <id> <labels...>` | - | Add labels to a project |
| `issues list` | `i ls` | List issues |
| `issues get <id>` | `i get` | Get issue details |
| `issues create <title>` | `i create` | Create a new issue |
| `issues update <id>` | `i update` | Update an issue |
| `issues delete <id>` | `i delete` | Delete an issue |
| `labels list` | `l ls` | List labels |
| `labels create <name>` | `l create` | Create a label |
| `labels delete <id>` | `l delete` | Delete a label |
| `teams list` | - | List teams |
| `teams get <id>` | - | Get team details |
| `users list` | - | List users |
| `users get <id>` | - | Get user details |
| `cycles list` | - | List cycles |
| `comments list <issue-id>` | - | List comments |
| `comments create <issue-id> <body>` | - | Create a comment |
| `documents list` | - | List documents |
| `documents get <id>` | - | Get document details |
| `documents create <title>` | - | Create a document |
| `documents update <id>` | - | Update a document |
| `search <query>` | - | Search issues |
| `sync` | - | Sync data with Linear |
| `config set-key <key>` | - | Set API key |
| `config show` | - | Show configuration |

## Common Options

| Option | Description |
|--------|-------------|
| `--help`, `-h` | Show help for any command |
| `--version`, `-V` | Show version information |
| `--force`, `-f` | Skip confirmation prompts (for delete operations) |
| `--archived`, `-a` | Include archived items |

## Output Format

The CLI uses formatted tables for list output and detailed views for single items. All output is designed for terminal readability with color-coded status indicators.

## License

MIT License - see [LICENSE](LICENSE) for details.

## Claude Code Integration

Add linear-cli to your Claude Code instructions with this one-liner:

```bash
mkdir -p ~/.claude && echo "Linear CLI: Use \`linear-cli\` for Linear.app operations instead of MCP tools. Commands: projects (p), issues (i), labels (l), teams (t), users (u), cycles (c), comments (cm), documents (d), search (s), sync (sy), statuses (st), git (g), config. Examples: \`linear-cli p list\`, \`linear-cli i create \"Title\" --team ENG\`, \`linear-cli l create \"Label\" --type project --color \"#FF5733\"\`, \`linear-cli g checkout LIN-123\`, \`linear-cli sy status\`" >> ~/.claude/CLAUDE.md
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.
