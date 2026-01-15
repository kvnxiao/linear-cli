mod api;
mod cache;
mod commands;
mod config;

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use commands::{
    bulk, comments, cycles, documents, git, interactive, issues, labels, notifications, projects,
    search, statuses, sync, teams, templates, time, uploads, users,
};

/// Output format for command results
#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum OutputFormat {
    /// Display results as formatted tables (default)
    #[default]
    Table,
    /// Display results as raw JSON
    Json,
}

#[derive(Parser)]
#[command(name = "linear")]
#[command(
    about = "A powerful CLI for Linear.app - manage issues, projects, and more from your terminal"
)]
#[command(version)]
#[command(after_help = r#"QUICK START:
    1. Get your API key from https://linear.app/settings/api
    2. Configure the CLI:
       linear config set-key YOUR_API_KEY
    3. List your issues:
       linear issues list
    4. Create an issue:
       linear issues create "Fix bug" --team ENG --priority 2

For more info on a command, run: linear <command> --help"#)]
struct Cli {
    /// Output format (table or json)
    #[arg(short, long, global = true, default_value = "table")]
    output: OutputFormat,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage projects - list, create, update, delete projects
    #[command(alias = "p")]
    #[command(after_help = r#"EXAMPLES:
    linear projects list                    # List all projects
    linear p list --archived                # Include archived projects
    linear p get PROJECT_ID                 # View project details
    linear p create "Q1 Roadmap" -t ENG     # Create a project"#)]
    Projects {
        #[command(subcommand)]
        action: projects::ProjectCommands,
    },
    /// Manage issues - list, create, update, assign, track issues
    #[command(alias = "i")]
    #[command(after_help = r#"EXAMPLES:
    linear issues list                      # List all issues
    linear i list -t ENG -s "In Progress"   # Filter by team and status
    linear i get LIN-123                    # View issue details
    linear i create "Bug fix" -t ENG -p 2   # Create high priority issue
    linear i update LIN-123 -s Done         # Update issue status"#)]
    Issues {
        #[command(subcommand)]
        action: issues::IssueCommands,
    },
    /// Manage labels - create and organize project/issue labels
    #[command(alias = "l")]
    #[command(after_help = r##"EXAMPLES:
    linear labels list                      # List project labels
    linear l list --type issue              # List issue labels
    linear l create "Feature" --color "#10B981"
    linear l delete LABEL_ID --force"##)]
    Labels {
        #[command(subcommand)]
        action: labels::LabelCommands,
    },
    /// Manage teams - list and view team details
    #[command(alias = "t")]
    #[command(after_help = r#"EXAMPLES:
    linear teams list                       # List all teams
    linear t get ENG                        # View team details"#)]
    Teams {
        #[command(subcommand)]
        action: teams::TeamCommands,
    },
    /// Manage users - list workspace users and view profiles
    #[command(alias = "u")]
    #[command(after_help = r#"EXAMPLES:
    linear users list                       # List all users
    linear u list --team ENG                # List team members
    linear u me                             # View your profile"#)]
    Users {
        #[command(subcommand)]
        action: users::UserCommands,
    },
    /// Manage cycles - view sprint cycles and current cycle
    #[command(alias = "c")]
    #[command(after_help = r#"EXAMPLES:
    linear cycles list -t ENG               # List team cycles
    linear c current -t ENG                 # Show current cycle"#)]
    Cycles {
        #[command(subcommand)]
        action: cycles::CycleCommands,
    },
    /// Manage comments - add and view issue comments
    #[command(alias = "cm")]
    #[command(after_help = r#"EXAMPLES:
    linear comments list ISSUE_ID           # List comments on issue
    linear cm create ISSUE_ID -b "LGTM!"    # Add a comment"#)]
    Comments {
        #[command(subcommand)]
        action: comments::CommentCommands,
    },
    /// Manage documents - create and organize documentation
    #[command(alias = "d")]
    #[command(after_help = r#"EXAMPLES:
    linear documents list                   # List all documents
    linear d get DOC_ID                     # View document
    linear d create "Design Doc" -p PROJ_ID # Create document"#)]
    Documents {
        #[command(subcommand)]
        action: documents::DocumentCommands,
    },
    /// Search across Linear - find issues and projects
    #[command(alias = "s")]
    #[command(after_help = r#"EXAMPLES:
    linear search issues "auth bug"         # Search issues
    linear s projects "backend"             # Search projects"#)]
    Search {
        #[command(subcommand)]
        action: search::SearchCommands,
    },
    /// Sync operations - compare local folders with Linear
    #[command(alias = "sy")]
    #[command(after_help = r#"EXAMPLES:
    linear sync status                      # Compare local vs Linear
    linear sy push -t ENG                   # Create projects for folders
    linear sy push -t ENG --dry-run         # Preview without creating"#)]
    Sync {
        #[command(subcommand)]
        action: sync::SyncCommands,
    },
    /// Manage issue statuses - view workflow states
    #[command(alias = "st")]
    #[command(after_help = r#"EXAMPLES:
    linear statuses list -t ENG             # List team statuses
    linear st get "In Progress" -t ENG      # View status details"#)]
    Statuses {
        #[command(subcommand)]
        action: statuses::StatusCommands,
    },
    /// Git branch operations - checkout branches, create PRs
    #[command(alias = "g")]
    #[command(after_help = r#"EXAMPLES:
    linear git checkout LIN-123             # Checkout issue branch
    linear g branch LIN-123                 # Show branch name
    linear g pr LIN-123                     # Create GitHub PR
    linear g pr LIN-123 --draft             # Create draft PR"#)]
    Git {
        #[command(subcommand)]
        action: git::GitCommands,
    },
    /// Bulk operations - update multiple issues at once
    #[command(alias = "b")]
    #[command(after_help = r#"EXAMPLES:
    linear bulk update -s Done LIN-1 LIN-2  # Update multiple issues
    linear b assign --user me LIN-1 LIN-2   # Assign multiple issues
    linear b label --add bug LIN-1 LIN-2    # Add label to issues"#)]
    Bulk {
        #[command(subcommand)]
        action: bulk::BulkCommands,
    },
    /// Manage cache - clear cached data or view status
    #[command(alias = "ca")]
    #[command(after_help = r#"EXAMPLES:
    linear cache status                     # Show cache status
    linear ca clear                         # Clear all cache
    linear ca clear --type teams            # Clear only teams cache"#)]
    Cache {
        #[command(subcommand)]
        action: commands::cache::CacheCommands,
    },
    /// Manage notifications - view and mark as read
    #[command(alias = "n")]
    #[command(after_help = r#"EXAMPLES:
    linear notifications list               # List unread notifications
    linear n count                          # Show unread count
    linear n read-all                       # Mark all as read"#)]
    Notifications {
        #[command(subcommand)]
        action: notifications::NotificationCommands,
    },
    /// Manage issue templates - create and use templates
    #[command(alias = "tpl")]
    #[command(after_help = r#"EXAMPLES:
    linear templates list                   # List all templates
    linear tpl create bug                   # Create a new template
    linear tpl show bug                     # View template details"#)]
    Templates {
        #[command(subcommand)]
        action: templates::TemplateCommands,
    },
    /// Time tracking - log and view time entries
    #[command(alias = "tm")]
    #[command(after_help = r#"EXAMPLES:
    linear time log LIN-123 2h              # Log 2 hours on issue
    linear tm list --issue LIN-123          # List time entries"#)]
    Time {
        #[command(subcommand)]
        action: time::TimeCommands,
    },
    /// Fetch uploads from Linear with authentication
    #[command(alias = "up")]
    #[command(after_help = r#"EXAMPLES:
    linear uploads fetch URL                # Output to stdout (for piping)
    linear up fetch URL -f file.png         # Save to file
    linear up fetch URL | base64            # Pipe to another tool"#)]
    Uploads {
        #[command(subcommand)]
        action: uploads::UploadCommands,
    },
    /// Interactive mode - TUI for browsing and managing issues
    #[command(alias = "int")]
    #[command(after_help = r#"EXAMPLES:
    linear interactive                      # Launch interactive mode

Use arrow keys to navigate, Enter to select, q to quit."#)]
    Interactive,
    /// Configure CLI settings - API keys and workspaces
    #[command(after_help = r#"EXAMPLES:
    linear config set-key YOUR_API_KEY      # Set API key
    linear config show                      # Show configuration
    linear config workspace-add work KEY    # Add workspace
    linear config workspace-switch work     # Switch workspace"#)]
    Config {
        #[command(subcommand)]
        action: ConfigCommands,
    },
}

#[derive(Subcommand)]
enum ConfigCommands {
    /// Set API key
    #[command(after_help = r#"EXAMPLE:
    linear config set-key lin_api_xxxxxxxxxxxxx"#)]
    SetKey {
        /// Your Linear API key
        key: String,
    },
    /// Show current configuration
    Show,
    /// Add a new workspace
    #[command(alias = "add")]
    #[command(after_help = r#"EXAMPLE:
    linear config workspace-add personal lin_api_xxxxxxxxxxxxx"#)]
    WorkspaceAdd {
        /// Workspace name
        name: String,
        /// API key for this workspace
        api_key: String,
    },
    /// List all workspaces
    #[command(alias = "list")]
    WorkspaceList,
    /// Switch to a different workspace
    #[command(alias = "use")]
    #[command(after_help = r#"EXAMPLE:
    linear config workspace-switch personal"#)]
    WorkspaceSwitch {
        /// Workspace name to switch to
        name: String,
    },
    /// Show current workspace
    #[command(alias = "current")]
    WorkspaceCurrent,
    /// Remove a workspace
    #[command(alias = "rm")]
    WorkspaceRemove {
        /// Workspace name to remove
        name: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let output = cli.output;

    match cli.command {
        Commands::Projects { action } => projects::handle(action, output).await?,
        Commands::Issues { action } => issues::handle(action, output).await?,
        Commands::Labels { action } => labels::handle(action, output).await?,
        Commands::Teams { action } => teams::handle(action, output).await?,
        Commands::Users { action } => users::handle(action).await?,
        Commands::Cycles { action } => cycles::handle(action).await?,
        Commands::Comments { action } => comments::handle(action, output).await?,
        Commands::Documents { action } => documents::handle(action).await?,
        Commands::Search { action } => search::handle(action).await?,
        Commands::Sync { action } => sync::handle(action).await?,
        Commands::Statuses { action } => statuses::handle(action).await?,
        Commands::Git { action } => git::handle(action).await?,
        Commands::Bulk { action } => bulk::handle(action).await?,
        Commands::Cache { action } => commands::cache::handle(action).await?,
        Commands::Notifications { action } => notifications::handle(action).await?,
        Commands::Templates { action } => templates::handle(action).await?,
        Commands::Time { action } => time::handle(action).await?,
        Commands::Uploads { action } => uploads::handle(action).await?,
        Commands::Interactive => interactive::run().await?,
        Commands::Config { action } => match action {
            ConfigCommands::SetKey { key } => {
                config::set_api_key(&key)?;
                println!("API key saved successfully!");
            }
            ConfigCommands::Show => {
                config::show_config()?;
            }
            ConfigCommands::WorkspaceAdd { name, api_key } => {
                config::workspace_add(&name, &api_key)?;
            }
            ConfigCommands::WorkspaceList => {
                config::workspace_list()?;
            }
            ConfigCommands::WorkspaceSwitch { name } => {
                config::workspace_switch(&name)?;
            }
            ConfigCommands::WorkspaceCurrent => {
                config::workspace_current()?;
            }
            ConfigCommands::WorkspaceRemove { name } => {
                config::workspace_remove(&name)?;
            }
        },
    }

    Ok(())
}
