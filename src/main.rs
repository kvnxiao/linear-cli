mod api;
mod commands;
mod config;

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use commands::{issues, labels, projects, teams, users, cycles, comments, documents, search, sync, statuses, git, bulk, interactive};

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
#[command(about = "A powerful CLI for Linear.app", long_about = None)]
#[command(version)]
struct Cli {
    /// Output format (table or json)
    #[arg(short, long, global = true, default_value = "table")]
    output: OutputFormat,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage projects
    #[command(alias = "p")]
    Projects {
        #[command(subcommand)]
        action: projects::ProjectCommands,
    },
    /// Manage issues
    #[command(alias = "i")]
    Issues {
        #[command(subcommand)]
        action: issues::IssueCommands,
    },
    /// Manage labels
    #[command(alias = "l")]
    Labels {
        #[command(subcommand)]
        action: labels::LabelCommands,
    },
    /// Manage teams
    #[command(alias = "t")]
    Teams {
        #[command(subcommand)]
        action: teams::TeamCommands,
    },
    /// Manage users
    #[command(alias = "u")]
    Users {
        #[command(subcommand)]
        action: users::UserCommands,
    },
    /// Manage cycles
    #[command(alias = "c")]
    Cycles {
        #[command(subcommand)]
        action: cycles::CycleCommands,
    },
    /// Manage comments
    #[command(alias = "cm")]
    Comments {
        #[command(subcommand)]
        action: comments::CommentCommands,
    },
    /// Manage documents
    #[command(alias = "d")]
    Documents {
        #[command(subcommand)]
        action: documents::DocumentCommands,
    },
    /// Search across Linear
    #[command(alias = "s")]
    Search {
        #[command(subcommand)]
        action: search::SearchCommands,
    },
    /// Sync operations
    #[command(alias = "sy")]
    Sync {
        #[command(subcommand)]
        action: sync::SyncCommands,
    },
    /// Manage issue statuses
    #[command(alias = "st")]
    Statuses {
        #[command(subcommand)]
        action: statuses::StatusCommands,
    },
    /// Git branch operations for issues
    #[command(alias = "g")]
    Git {
        #[command(subcommand)]
        action: git::GitCommands,
    },
    /// Bulk operations on multiple issues
    #[command(alias = "b")]
    Bulk {
        #[command(subcommand)]
        action: bulk::BulkCommands,
    },
    /// Interactive mode for issue management
    #[command(alias = "int")]
    Interactive,
    /// Configure CLI settings
    Config {
        #[command(subcommand)]
        action: ConfigCommands,
    },
}

#[derive(Subcommand)]
enum ConfigCommands {
    /// Set API key
    SetKey {
        /// Your Linear API key
        key: String,
    },
    /// Show current configuration
    Show,
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
        Commands::Comments { action } => comments::handle(action).await?,
        Commands::Documents { action } => documents::handle(action).await?,
        Commands::Search { action } => search::handle(action).await?,
        Commands::Sync { action } => sync::handle(action).await?,
        Commands::Statuses { action } => statuses::handle(action).await?,
        Commands::Git { action } => git::handle(action).await?,
        Commands::Bulk { action } => bulk::handle(action).await?,
        Commands::Interactive => interactive::run().await?,
        Commands::Config { action } => match action {
            ConfigCommands::SetKey { key } => {
                config::set_api_key(&key)?;
                println!("API key saved successfully!");
            }
            ConfigCommands::Show => {
                config::show_config()?;
            }
        },
    }

    Ok(())
}
