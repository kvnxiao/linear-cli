use anyhow::Result;
use clap::Subcommand;
use colored::Colorize;
use futures::future::join_all;
use serde_json::json;

use crate::api::LinearClient;

#[derive(Subcommand)]
pub enum BulkCommands {
    /// Update the state of multiple issues
    #[command(alias = "state")]
    UpdateState {
        /// The new state name or ID
        state: String,
        /// Comma-separated list of issue IDs (e.g., "LIN-1,LIN-2,LIN-3")
        #[arg(short, long, value_delimiter = ',')]
        issues: Vec<String>,
    },
    /// Assign multiple issues to a user
    Assign {
        /// The user to assign (user ID, name, email, or "me")
        user: String,
        /// Comma-separated list of issue IDs (e.g., "LIN-1,LIN-2,LIN-3")
        #[arg(short, long, value_delimiter = ',')]
        issues: Vec<String>,
    },
    /// Add a label to multiple issues
    Label {
        /// The label name or ID to add
        label: String,
        /// Comma-separated list of issue IDs (e.g., "LIN-1,LIN-2,LIN-3")
        #[arg(short, long, value_delimiter = ',')]
        issues: Vec<String>,
    },
    /// Unassign multiple issues
    Unassign {
        /// Comma-separated list of issue IDs (e.g., "LIN-1,LIN-2,LIN-3")
        #[arg(short, long, value_delimiter = ',')]
        issues: Vec<String>,
    },
}

/// Result of a single bulk operation
#[derive(Debug)]
struct BulkResult {
    issue_id: String,
    success: bool,
    identifier: Option<String>,
    error: Option<String>,
}

pub async fn handle(cmd: BulkCommands) -> Result<()> {
    match cmd {
        BulkCommands::UpdateState { state, issues } => {
            bulk_update_state(&state, issues).await
        }
        BulkCommands::Assign { user, issues } => {
            bulk_assign(&user, issues).await
        }
        BulkCommands::Label { label, issues } => {
            bulk_label(&label, issues).await
        }
        BulkCommands::Unassign { issues } => {
            bulk_unassign(issues).await
        }
    }
}

async fn bulk_update_state(state: &str, issues: Vec<String>) -> Result<()> {
    if issues.is_empty() {
        println!("No issues specified.");
        return Ok(());
    }

    println!(
        "{} Updating state to '{}' for {} issues...",
        ">>".cyan(),
        state,
        issues.len()
    );

    let client = LinearClient::new()?;
    let state_owned = state.to_string();

    let futures: Vec<_> = issues
        .iter()
        .map(|issue_id| {
            let client = &client;
            let state = &state_owned;
            let id = issue_id.clone();
            async move {
                update_issue_state(client, &id, state).await
            }
        })
        .collect();

    let results = join_all(futures).await;
    print_summary(&results, "state updated");

    Ok(())
}

async fn bulk_assign(user: &str, issues: Vec<String>) -> Result<()> {
    if issues.is_empty() {
        println!("No issues specified.");
        return Ok(());
    }

    println!(
        "{} Assigning {} issues to '{}'...",
        ">>".cyan(),
        issues.len(),
        user
    );

    let client = LinearClient::new()?;
    let user_owned = user.to_string();

    let futures: Vec<_> = issues
        .iter()
        .map(|issue_id| {
            let client = &client;
            let user = &user_owned;
            let id = issue_id.clone();
            async move {
                update_issue_assignee(client, &id, Some(user)).await
            }
        })
        .collect();

    let results = join_all(futures).await;
    print_summary(&results, "assigned");

    Ok(())
}

async fn bulk_label(label: &str, issues: Vec<String>) -> Result<()> {
    if issues.is_empty() {
        println!("No issues specified.");
        return Ok(());
    }

    println!(
        "{} Adding label '{}' to {} issues...",
        ">>".cyan(),
        label,
        issues.len()
    );

    let client = LinearClient::new()?;
    let label_owned = label.to_string();

    let futures: Vec<_> = issues
        .iter()
        .map(|issue_id| {
            let client = &client;
            let label = &label_owned;
            let id = issue_id.clone();
            async move {
                add_label_to_issue(client, &id, label).await
            }
        })
        .collect();

    let results = join_all(futures).await;
    print_summary(&results, "labeled");

    Ok(())
}

async fn bulk_unassign(issues: Vec<String>) -> Result<()> {
    if issues.is_empty() {
        println!("No issues specified.");
        return Ok(());
    }

    println!(
        "{} Unassigning {} issues...",
        ">>".cyan(),
        issues.len()
    );

    let client = LinearClient::new()?;

    let futures: Vec<_> = issues
        .iter()
        .map(|issue_id| {
            let client = &client;
            let id = issue_id.clone();
            async move {
                update_issue_assignee(client, &id, None).await
            }
        })
        .collect();

    let results = join_all(futures).await;
    print_summary(&results, "unassigned");

    Ok(())
}

async fn update_issue_state(client: &LinearClient, issue_id: &str, state: &str) -> BulkResult {
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

    let input = json!({ "stateId": state });

    match client
        .mutate(mutation, Some(json!({ "id": issue_id, "input": input })))
        .await
    {
        Ok(result) => {
            if result["data"]["issueUpdate"]["success"].as_bool() == Some(true) {
                let identifier = result["data"]["issueUpdate"]["issue"]["identifier"]
                    .as_str()
                    .map(|s| s.to_string());
                BulkResult {
                    issue_id: issue_id.to_string(),
                    success: true,
                    identifier,
                    error: None,
                }
            } else {
                BulkResult {
                    issue_id: issue_id.to_string(),
                    success: false,
                    identifier: None,
                    error: Some("Update failed".to_string()),
                }
            }
        }
        Err(e) => BulkResult {
            issue_id: issue_id.to_string(),
            success: false,
            identifier: None,
            error: Some(e.to_string()),
        },
    }
}

async fn update_issue_assignee(
    client: &LinearClient,
    issue_id: &str,
    assignee: Option<&str>,
) -> BulkResult {
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

    let input = match assignee {
        Some(user) => json!({ "assigneeId": user }),
        None => json!({ "assigneeId": null }),
    };

    match client
        .mutate(mutation, Some(json!({ "id": issue_id, "input": input })))
        .await
    {
        Ok(result) => {
            if result["data"]["issueUpdate"]["success"].as_bool() == Some(true) {
                let identifier = result["data"]["issueUpdate"]["issue"]["identifier"]
                    .as_str()
                    .map(|s| s.to_string());
                BulkResult {
                    issue_id: issue_id.to_string(),
                    success: true,
                    identifier,
                    error: None,
                }
            } else {
                BulkResult {
                    issue_id: issue_id.to_string(),
                    success: false,
                    identifier: None,
                    error: Some("Update failed".to_string()),
                }
            }
        }
        Err(e) => BulkResult {
            issue_id: issue_id.to_string(),
            success: false,
            identifier: None,
            error: Some(e.to_string()),
        },
    }
}

async fn add_label_to_issue(client: &LinearClient, issue_id: &str, label: &str) -> BulkResult {
    // First, get existing labels for the issue
    let query = r#"
        query($id: String!) {
            issue(id: $id) {
                identifier
                labels {
                    nodes {
                        id
                    }
                }
            }
        }
    "#;

    let existing_labels = match client.query(query, Some(json!({ "id": issue_id }))).await {
        Ok(result) => {
            let identifier = result["data"]["issue"]["identifier"]
                .as_str()
                .map(|s| s.to_string());

            if result["data"]["issue"].is_null() {
                return BulkResult {
                    issue_id: issue_id.to_string(),
                    success: false,
                    identifier: None,
                    error: Some("Issue not found".to_string()),
                };
            }

            let labels: Vec<String> = result["data"]["issue"]["labels"]["nodes"]
                .as_array()
                .unwrap_or(&vec![])
                .iter()
                .filter_map(|l| l["id"].as_str().map(|s| s.to_string()))
                .collect();

            (labels, identifier)
        }
        Err(e) => {
            return BulkResult {
                issue_id: issue_id.to_string(),
                success: false,
                identifier: None,
                error: Some(e.to_string()),
            };
        }
    };

    let (mut label_ids, identifier) = existing_labels;

    // Add the new label if not already present
    if !label_ids.contains(&label.to_string()) {
        label_ids.push(label.to_string());
    }

    // Update the issue with the new label list
    let mutation = r#"
        mutation($id: String!, $input: IssueUpdateInput!) {
            issueUpdate(id: $id, input: $input) {
                success
                issue {
                    identifier
                }
            }
        }
    "#;

    let input = json!({ "labelIds": label_ids });

    match client
        .mutate(mutation, Some(json!({ "id": issue_id, "input": input })))
        .await
    {
        Ok(result) => {
            if result["data"]["issueUpdate"]["success"].as_bool() == Some(true) {
                let identifier = result["data"]["issueUpdate"]["issue"]["identifier"]
                    .as_str()
                    .map(|s| s.to_string())
                    .or(identifier);
                BulkResult {
                    issue_id: issue_id.to_string(),
                    success: true,
                    identifier,
                    error: None,
                }
            } else {
                BulkResult {
                    issue_id: issue_id.to_string(),
                    success: false,
                    identifier,
                    error: Some("Update failed".to_string()),
                }
            }
        }
        Err(e) => BulkResult {
            issue_id: issue_id.to_string(),
            success: false,
            identifier,
            error: Some(e.to_string()),
        },
    }
}

fn print_summary(results: &[BulkResult], action: &str) {
    println!();

    let success_count = results.iter().filter(|r| r.success).count();
    let failure_count = results.len() - success_count;

    // Print individual results
    for result in results {
        if result.success {
            let display_id = result.identifier.as_deref().unwrap_or(&result.issue_id);
            println!("  {} {} {}", "+".green(), display_id.cyan(), action);
        } else {
            let error_msg = result.error.as_deref().unwrap_or("Unknown error");
            println!(
                "  {} {} failed: {}",
                "x".red(),
                result.issue_id.cyan(),
                error_msg.dimmed()
            );
        }
    }

    // Print summary
    println!();
    println!(
        "{} Summary: {} succeeded, {} failed",
        ">>".cyan(),
        success_count.to_string().green(),
        if failure_count > 0 {
            failure_count.to_string().red().to_string()
        } else {
            failure_count.to_string()
        }
    );
}
