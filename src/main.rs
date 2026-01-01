mod api;
mod commands;
mod config;

use anyhow::Result;
use clap::{Parser, Subcommand};
use commands::{issues, labels, projects, teams, users, cycles, comments, documents, search, sync, statuses, git};

#[derive(Parser)]
#[command(name = "linear")]
#[command(about = "A powerful CLI for Linear.app", long_about = None)]
#[command(version)]
struct Cli {
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

    match cli.command {
        Commands::Projects { action } => projects::handle(action).await?,
        Commands::Issues { action } => issues::handle(action).await?,
        Commands::Labels { action } => labels::handle(action).await?,
        Commands::Teams { action } => teams::handle(action).await?,
        Commands::Users { action } => users::handle(action).await?,
        Commands::Cycles { action } => cycles::handle(action).await?,
        Commands::Comments { action } => comments::handle(action).await?,
        Commands::Documents { action } => documents::handle(action).await?,
        Commands::Search { action } => search::handle(action).await?,
        Commands::Sync { action } => sync::handle(action).await?,
        Commands::Statuses { action } => statuses::handle(action).await?,
        Commands::Git { action } => git::handle(action).await?,
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
