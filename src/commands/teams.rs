use anyhow::Result;
use clap::Subcommand;
use colored::Colorize;
use serde_json::json;
use tabled::{Table, Tabled};

use crate::api::LinearClient;

#[derive(Subcommand)]
pub enum TeamCommands {
    /// List all teams
    #[command(alias = "ls")]
    List,
    /// Get team details
    Get {
        /// Team ID, key, or name
        id: String,
    },
}

#[derive(Tabled)]
struct TeamRow {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Key")]
    key: String,
    #[tabled(rename = "ID")]
    id: String,
}

pub async fn handle(cmd: TeamCommands) -> Result<()> {
    match cmd {
        TeamCommands::List => list_teams().await,
        TeamCommands::Get { id } => get_team(&id).await,
    }
}

async fn list_teams() -> Result<()> {
    let client = LinearClient::new()?;

    let query = r#"
        query {
            teams(first: 100) {
                nodes {
                    id
                    name
                    key
                }
            }
        }
    "#;

    let result = client.query(query, None).await?;

    let empty = vec![];
    let teams = result["data"]["teams"]["nodes"]
        .as_array()
        .unwrap_or(&empty);

    if teams.is_empty() {
        println!("No teams found.");
        return Ok(());
    }

    let rows: Vec<TeamRow> = teams
        .iter()
        .map(|t| TeamRow {
            name: t["name"].as_str().unwrap_or("").to_string(),
            key: t["key"].as_str().unwrap_or("").to_string(),
            id: t["id"].as_str().unwrap_or("").to_string(),
        })
        .collect();

    let table = Table::new(rows).to_string();
    println!("{}", table);
    println!("\n{} teams", teams.len());

    Ok(())
}

async fn get_team(id: &str) -> Result<()> {
    let client = LinearClient::new()?;

    let query = r#"
        query($id: String!) {
            team(id: $id) {
                id
                name
                key
                description
                icon
                color
                private
                timezone
                issueCount
                createdAt
                updatedAt
            }
        }
    "#;

    let result = client.query(query, Some(json!({ "id": id }))).await?;
    let team = &result["data"]["team"];

    if team.is_null() {
        anyhow::bail!("Team not found: {}", id);
    }

    println!("{}", team["name"].as_str().unwrap_or("").bold());
    println!("{}", "─".repeat(40));

    println!("Key: {}", team["key"].as_str().unwrap_or("-"));

    if let Some(desc) = team["description"].as_str() {
        if !desc.is_empty() {
            println!("Description: {}", desc);
        }
    }

    println!("Private: {}", team["private"].as_bool().unwrap_or(false));

    if let Some(timezone) = team["timezone"].as_str() {
        println!("Timezone: {}", timezone);
    }

    if let Some(issue_count) = team["issueCount"].as_i64() {
        println!("Issue Count: {}", issue_count);
    }

    if let Some(color) = team["color"].as_str() {
        println!("Color: {}", color);
    }

    if let Some(icon) = team["icon"].as_str() {
        println!("Icon: {}", icon);
    }

    println!("ID: {}", team["id"].as_str().unwrap_or("-"));

    if let Some(created_at) = team["createdAt"].as_str() {
        println!("Created: {}", created_at);
    }

    if let Some(updated_at) = team["updatedAt"].as_str() {
        println!("Updated: {}", updated_at);
    }

    Ok(())
}
