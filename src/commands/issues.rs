use anyhow::Result;
use clap::Subcommand;
use colored::Colorize;
use serde_json::json;
use tabled::{Table, Tabled};

use crate::api::LinearClient;

#[derive(Subcommand)]
pub enum IssueCommands {
    /// List issues
    #[command(alias = "ls")]
    List {
        /// Filter by team name or ID
        #[arg(short, long)]
        team: Option<String>,
        /// Filter by state name or ID
        #[arg(short, long)]
        state: Option<String>,
        /// Filter by assignee (user ID, name, email, or "me")
        #[arg(short, long)]
        assignee: Option<String>,
        /// Include archived issues
        #[arg(long)]
        archived: bool,
        /// Maximum number of issues to return
        #[arg(short, long, default_value = "50")]
        limit: u32,
    },
    /// Get issue details
    Get {
        /// Issue ID or identifier (e.g., "LIN-123")
        id: String,
    },
    /// Create a new issue
    Create {
        /// Issue title
        title: String,
        /// Team name or ID
        #[arg(short, long)]
        team: String,
        /// Issue description (markdown)
        #[arg(short, long)]
        description: Option<String>,
        /// Priority (0=none, 1=urgent, 2=high, 3=normal, 4=low)
        #[arg(short, long)]
        priority: Option<i32>,
        /// State name or ID
        #[arg(short, long)]
        state: Option<String>,
        /// Assignee (user ID, name, email, or "me")
        #[arg(short, long)]
        assignee: Option<String>,
        /// Labels to add (can be specified multiple times)
        #[arg(short, long)]
        labels: Vec<String>,
    },
    /// Update an existing issue
    Update {
        /// Issue ID
        id: String,
        /// New title
        #[arg(short = 'T', long)]
        title: Option<String>,
        /// New description
        #[arg(short, long)]
        description: Option<String>,
        /// New priority (0=none, 1=urgent, 2=high, 3=normal, 4=low)
        #[arg(short, long)]
        priority: Option<i32>,
        /// New state name or ID
        #[arg(short, long)]
        state: Option<String>,
        /// New assignee (user ID, name, email, or "me")
        #[arg(short, long)]
        assignee: Option<String>,
    },
    /// Delete an issue
    Delete {
        /// Issue ID
        id: String,
        /// Skip confirmation
        #[arg(short, long)]
        force: bool,
    },
}

#[derive(Tabled)]
struct IssueRow {
    #[tabled(rename = "ID")]
    identifier: String,
    #[tabled(rename = "Title")]
    title: String,
    #[tabled(rename = "State")]
    state: String,
    #[tabled(rename = "Priority")]
    priority: String,
    #[tabled(rename = "Assignee")]
    assignee: String,
}

pub async fn handle(cmd: IssueCommands) -> Result<()> {
    match cmd {
        IssueCommands::List {
            team,
            state,
            assignee,
            archived,
            limit,
        } => list_issues(team, state, assignee, archived, limit).await,
        IssueCommands::Get { id } => get_issue(&id).await,
        IssueCommands::Create {
            title,
            team,
            description,
            priority,
            state,
            assignee,
            labels,
        } => create_issue(&title, &team, description, priority, state, assignee, labels).await,
        IssueCommands::Update {
            id,
            title,
            description,
            priority,
            state,
            assignee,
        } => update_issue(&id, title, description, priority, state, assignee).await,
        IssueCommands::Delete { id, force } => delete_issue(&id, force).await,
    }
}

fn priority_to_string(priority: Option<i64>) -> String {
    match priority {
        Some(0) => "-".to_string(),
        Some(1) => "Urgent".red().to_string(),
        Some(2) => "High".yellow().to_string(),
        Some(3) => "Normal".to_string(),
        Some(4) => "Low".dimmed().to_string(),
        _ => "-".to_string(),
    }
}

async fn list_issues(
    team: Option<String>,
    state: Option<String>,
    assignee: Option<String>,
    include_archived: bool,
    limit: u32,
) -> Result<()> {
    let client = LinearClient::new()?;

    let query = r#"
        query($team: String, $state: String, $assignee: String, $includeArchived: Boolean, $limit: Int) {
            issues(
                first: $limit,
                includeArchived: $includeArchived,
                filter: {
                    team: { name: { eqIgnoreCase: $team } },
                    state: { name: { eqIgnoreCase: $state } },
                    assignee: { name: { eqIgnoreCase: $assignee } }
                }
            ) {
                nodes {
                    id
                    identifier
                    title
                    priority
                    state { name }
                    assignee { name }
                }
            }
        }
    "#;

    let mut variables = json!({
        "includeArchived": include_archived,
        "limit": limit
    });

    if let Some(t) = team {
        variables["team"] = json!(t);
    }
    if let Some(s) = state {
        variables["state"] = json!(s);
    }
    if let Some(a) = assignee {
        variables["assignee"] = json!(a);
    }

    let result = client.query(query, Some(variables)).await?;

    let empty = vec![];
    let issues = result["data"]["issues"]["nodes"]
        .as_array()
        .unwrap_or(&empty);

    if issues.is_empty() {
        println!("No issues found.");
        return Ok(());
    }

    let rows: Vec<IssueRow> = issues
        .iter()
        .map(|issue| IssueRow {
            identifier: issue["identifier"].as_str().unwrap_or("").to_string(),
            title: {
                let t = issue["title"].as_str().unwrap_or("");
                if t.len() > 50 {
                    format!("{}...", &t[..47])
                } else {
                    t.to_string()
                }
            },
            state: issue["state"]["name"].as_str().unwrap_or("-").to_string(),
            priority: priority_to_string(issue["priority"].as_i64()),
            assignee: issue["assignee"]["name"]
                .as_str()
                .unwrap_or("-")
                .to_string(),
        })
        .collect();

    let table = Table::new(rows).to_string();
    println!("{}", table);
    println!("\n{} issues", issues.len());

    Ok(())
}

async fn get_issue(id: &str) -> Result<()> {
    let client = LinearClient::new()?;

    let query = r#"
        query($id: String!) {
            issue(id: $id) {
                id
                identifier
                title
                description
                priority
                url
                createdAt
                updatedAt
                state { name }
                team { name }
                assignee { name email }
                labels { nodes { name color } }
                project { name }
                parent { identifier title }
            }
        }
    "#;

    let result = client.query(query, Some(json!({ "id": id }))).await?;
    let issue = &result["data"]["issue"];

    if issue.is_null() {
        anyhow::bail!("Issue not found: {}", id);
    }

    let identifier = issue["identifier"].as_str().unwrap_or("");
    let title = issue["title"].as_str().unwrap_or("");
    println!("{} {}", identifier.cyan().bold(), title.bold());
    println!("{}", "-".repeat(60));

    if let Some(desc) = issue["description"].as_str() {
        if !desc.is_empty() {
            println!("\n{}", desc);
            println!();
        }
    }

    println!("State:    {}", issue["state"]["name"].as_str().unwrap_or("-"));
    println!("Priority: {}", priority_to_string(issue["priority"].as_i64()));
    println!("Team:     {}", issue["team"]["name"].as_str().unwrap_or("-"));

    if let Some(assignee) = issue["assignee"]["name"].as_str() {
        let email = issue["assignee"]["email"].as_str().unwrap_or("");
        if !email.is_empty() {
            println!("Assignee: {} ({})", assignee, email.dimmed());
        } else {
            println!("Assignee: {}", assignee);
        }
    } else {
        println!("Assignee: -");
    }

    if let Some(project) = issue["project"]["name"].as_str() {
        println!("Project:  {}", project);
    }

    if let Some(parent) = issue["parent"]["identifier"].as_str() {
        let parent_title = issue["parent"]["title"].as_str().unwrap_or("");
        println!("Parent:   {} {}", parent, parent_title.dimmed());
    }

    let labels = issue["labels"]["nodes"].as_array();
    if let Some(labels) = labels {
        if !labels.is_empty() {
            let label_names: Vec<&str> = labels
                .iter()
                .filter_map(|l| l["name"].as_str())
                .collect();
            println!("Labels:   {}", label_names.join(", "));
        }
    }

    println!("\nURL: {}", issue["url"].as_str().unwrap_or("-"));
    println!("ID:  {}", issue["id"].as_str().unwrap_or("-"));

    Ok(())
}

async fn create_issue(
    title: &str,
    team: &str,
    description: Option<String>,
    priority: Option<i32>,
    state: Option<String>,
    assignee: Option<String>,
    labels: Vec<String>,
) -> Result<()> {
    let client = LinearClient::new()?;

    let mut input = json!({
        "title": title,
        "teamId": team
    });

    if let Some(desc) = description {
        input["description"] = json!(desc);
    }
    if let Some(p) = priority {
        input["priority"] = json!(p);
    }
    if let Some(s) = state {
        input["stateId"] = json!(s);
    }
    if let Some(a) = assignee {
        input["assigneeId"] = json!(a);
    }
    if !labels.is_empty() {
        input["labelIds"] = json!(labels);
    }

    let mutation = r#"
        mutation($input: IssueCreateInput!) {
            issueCreate(input: $input) {
                success
                issue {
                    id
                    identifier
                    title
                    url
                }
            }
        }
    "#;

    let result = client.mutate(mutation, Some(json!({ "input": input }))).await?;

    if result["data"]["issueCreate"]["success"].as_bool() == Some(true) {
        let issue = &result["data"]["issueCreate"]["issue"];
        let identifier = issue["identifier"].as_str().unwrap_or("");
        let issue_title = issue["title"].as_str().unwrap_or("");
        println!(
            "{} Created issue: {} {}",
            "+".green(),
            identifier.cyan(),
            issue_title
        );
        println!("  ID:  {}", issue["id"].as_str().unwrap_or(""));
        println!("  URL: {}", issue["url"].as_str().unwrap_or(""));
    } else {
        anyhow::bail!("Failed to create issue");
    }

    Ok(())
}

async fn update_issue(
    id: &str,
    title: Option<String>,
    description: Option<String>,
    priority: Option<i32>,
    state: Option<String>,
    assignee: Option<String>,
) -> Result<()> {
    let client = LinearClient::new()?;

    let mut input = json!({});

    if let Some(t) = title {
        input["title"] = json!(t);
    }
    if let Some(d) = description {
        input["description"] = json!(d);
    }
    if let Some(p) = priority {
        input["priority"] = json!(p);
    }
    if let Some(s) = state {
        input["stateId"] = json!(s);
    }
    if let Some(a) = assignee {
        input["assigneeId"] = json!(a);
    }

    if input.as_object().map(|o| o.is_empty()).unwrap_or(true) {
        println!("No updates specified.");
        return Ok(());
    }

    let mutation = r#"
        mutation($id: String!, $input: IssueUpdateInput!) {
            issueUpdate(id: $id, input: $input) {
                success
                issue {
                    identifier
                    title
                }
            }
        }
    "#;

    let result = client
        .mutate(mutation, Some(json!({ "id": id, "input": input })))
        .await?;

    if result["data"]["issueUpdate"]["success"].as_bool() == Some(true) {
        let issue = &result["data"]["issueUpdate"]["issue"];
        println!(
            "{} Updated issue: {} {}",
            "+".green(),
            issue["identifier"].as_str().unwrap_or(""),
            issue["title"].as_str().unwrap_or("")
        );
    } else {
        anyhow::bail!("Failed to update issue");
    }

    Ok(())
}

async fn delete_issue(id: &str, force: bool) -> Result<()> {
    if !force {
        println!("Are you sure you want to delete issue {}?", id);
        println!("This action cannot be undone. Use --force to skip this prompt.");
        return Ok(());
    }

    let client = LinearClient::new()?;

    let mutation = r#"
        mutation($id: String!) {
            issueDelete(id: $id) {
                success
            }
        }
    "#;

    let result = client.mutate(mutation, Some(json!({ "id": id }))).await?;

    if result["data"]["issueDelete"]["success"].as_bool() == Some(true) {
        println!("{} Issue deleted", "+".green());
    } else {
        anyhow::bail!("Failed to delete issue");
    }

    Ok(())
}
