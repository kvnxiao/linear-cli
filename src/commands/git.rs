use anyhow::Result;
use clap::Subcommand;
use colored::Colorize;
use serde_json::json;
use std::process::Command;

use crate::api::LinearClient;

#[derive(Subcommand)]
pub enum GitCommands {
    /// Checkout a branch for an issue (creates if doesn't exist)
    Checkout {
        /// Issue identifier (e.g., "LIN-123") or ID
        issue: String,
        /// Custom branch name (optional, uses issue's branch name by default)
        #[arg(short, long)]
        branch: Option<String>,
    },
    /// Show the branch name for an issue
    Branch {
        /// Issue identifier (e.g., "LIN-123") or ID
        issue: String,
    },
    /// Create a branch for an issue without checking out
    Create {
        /// Issue identifier (e.g., "LIN-123") or ID
        issue: String,
        /// Custom branch name (optional)
        #[arg(short, long)]
        branch: Option<String>,
    },
}

pub async fn handle(cmd: GitCommands) -> Result<()> {
    match cmd {
        GitCommands::Checkout { issue, branch } => checkout_issue(&issue, branch).await,
        GitCommands::Branch { issue } => show_branch(&issue).await,
        GitCommands::Create { issue, branch } => create_branch(&issue, branch).await,
    }
}

async fn get_issue_info(issue_id: &str) -> Result<(String, String, String)> {
    let client = LinearClient::new()?;

    let query = r#"
        query($id: String!) {
            issue(id: $id) {
                id
                identifier
                title
                branchName
            }
        }
    "#;

    let result = client.query(query, Some(json!({ "id": issue_id }))).await?;
    let issue = &result["data"]["issue"];

    if issue.is_null() {
        anyhow::bail!("Issue not found: {}", issue_id);
    }

    let identifier = issue["identifier"].as_str().unwrap_or("").to_string();
    let title = issue["title"].as_str().unwrap_or("").to_string();
    let branch_name = issue["branchName"].as_str().unwrap_or("").to_string();

    Ok((identifier, title, branch_name))
}

fn generate_branch_name(identifier: &str, title: &str) -> String {
    // Convert title to kebab-case for branch name
    let slug: String = title
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-");

    // Truncate if too long
    let slug = if slug.len() > 50 {
        slug[..50].trim_end_matches('-').to_string()
    } else {
        slug
    };

    format!("{}/{}", identifier.to_lowercase(), slug)
}

fn run_git_command(args: &[&str]) -> Result<String> {
    let output = Command::new("git")
        .args(args)
        .output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Git command failed: {}", stderr.trim());
    }
}

fn branch_exists(branch: &str) -> bool {
    run_git_command(&["rev-parse", "--verify", branch]).is_ok()
}

fn get_current_branch() -> Result<String> {
    run_git_command(&["rev-parse", "--abbrev-ref", "HEAD"])
}

async fn checkout_issue(issue_id: &str, custom_branch: Option<String>) -> Result<()> {
    let (identifier, title, linear_branch) = get_issue_info(issue_id).await?;

    let branch_name = custom_branch
        .or_else(|| if linear_branch.is_empty() { None } else { Some(linear_branch) })
        .unwrap_or_else(|| generate_branch_name(&identifier, &title));

    println!("{} {} {}", identifier.cyan(), title.dimmed(), "");

    if branch_exists(&branch_name) {
        // Branch exists, just checkout
        println!("Checking out existing branch: {}", branch_name.green());
        run_git_command(&["checkout", &branch_name])?;
    } else {
        // Create and checkout new branch
        println!("Creating and checking out branch: {}", branch_name.green());
        run_git_command(&["checkout", "-b", &branch_name])?;
    }

    let current = get_current_branch()?;
    println!("{} Now on branch: {}", "✓".green(), current);

    Ok(())
}

async fn show_branch(issue_id: &str) -> Result<()> {
    let (identifier, title, linear_branch) = get_issue_info(issue_id).await?;

    println!("{} {}", identifier.cyan().bold(), title);
    println!("{}", "-".repeat(50));

    if !linear_branch.is_empty() {
        println!("Linear branch: {}", linear_branch.green());
    }

    let generated = generate_branch_name(&identifier, &title);
    println!("Generated:     {}", generated.yellow());

    // Check if either branch exists locally
    if branch_exists(&linear_branch) {
        println!("\n{} Linear branch exists locally", "✓".green());
    } else if branch_exists(&generated) {
        println!("\n{} Generated branch exists locally", "✓".green());
    } else {
        println!("\n{} No local branch found for this issue", "!".yellow());
    }

    Ok(())
}

async fn create_branch(issue_id: &str, custom_branch: Option<String>) -> Result<()> {
    let (identifier, title, linear_branch) = get_issue_info(issue_id).await?;

    let branch_name = custom_branch
        .or_else(|| if linear_branch.is_empty() { None } else { Some(linear_branch) })
        .unwrap_or_else(|| generate_branch_name(&identifier, &title));

    println!("{} {} {}", identifier.cyan(), title.dimmed(), "");

    if branch_exists(&branch_name) {
        println!("{} Branch already exists: {}", "!".yellow(), branch_name);
        return Ok(());
    }

    // Create branch without checking out
    run_git_command(&["branch", &branch_name])?;
    println!("{} Created branch: {}", "✓".green(), branch_name);

    Ok(())
}
