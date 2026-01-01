use anyhow::Result;
use clap::Subcommand;
use colored::Colorize;
use serde_json::json;
use tabled::{Table, Tabled};

use crate::api::LinearClient;

#[derive(Subcommand)]
pub enum DocumentCommands {
    /// List all documents
    #[command(alias = "ls")]
    List {
        /// Filter by project ID
        #[arg(short, long)]
        project: Option<String>,
        /// Include archived documents
        #[arg(short, long)]
        archived: bool,
    },
    /// Get document details and content
    Get {
        /// Document ID or slug
        id: String,
    },
    /// Create a new document
    Create {
        /// Document title
        title: String,
        /// Project name or ID to associate the document with
        #[arg(short, long)]
        project: String,
        /// Document content (Markdown)
        #[arg(short, long)]
        content: Option<String>,
        /// Document icon (e.g., ":page_facing_up:")
        #[arg(short, long)]
        icon: Option<String>,
        /// Icon color (hex color code)
        #[arg(long)]
        color: Option<String>,
    },
    /// Update an existing document
    Update {
        /// Document ID
        id: String,
        /// New title
        #[arg(short, long)]
        title: Option<String>,
        /// New content (Markdown)
        #[arg(short, long)]
        content: Option<String>,
        /// New icon
        #[arg(short, long)]
        icon: Option<String>,
        /// New color (hex)
        #[arg(long)]
        color: Option<String>,
        /// New project ID
        #[arg(short, long)]
        project: Option<String>,
    },
}

#[derive(Tabled)]
struct DocumentRow {
    #[tabled(rename = "Title")]
    title: String,
    #[tabled(rename = "Project")]
    project: String,
    #[tabled(rename = "Updated")]
    updated: String,
    #[tabled(rename = "ID")]
    id: String,
}

pub async fn handle(cmd: DocumentCommands) -> Result<()> {
    match cmd {
        DocumentCommands::List { project, archived } => list_documents(project, archived).await,
        DocumentCommands::Get { id } => get_document(&id).await,
        DocumentCommands::Create { title, project, content, icon, color } => {
            create_document(&title, &project, content, icon, color).await
        }
        DocumentCommands::Update { id, title, content, icon, color, project } => {
            update_document(&id, title, content, icon, color, project).await
        }
    }
}

async fn list_documents(project_id: Option<String>, include_archived: bool) -> Result<()> {
    let client = LinearClient::new()?;

    let query = r#"
        query($projectId: String, $includeArchived: Boolean) {
            documents(first: 100, includeArchived: $includeArchived) {
                nodes {
                    id
                    title
                    updatedAt
                    project { id name }
                }
            }
        }
    "#;

    let mut variables = json!({ "includeArchived": include_archived });
    if let Some(ref pid) = project_id {
        variables["projectId"] = json!(pid);
    }

    let result = client.query(query, Some(variables)).await?;

    let empty = vec![];
    let documents = result["data"]["documents"]["nodes"]
        .as_array()
        .unwrap_or(&empty);

    // Filter by project if specified
    let filtered_docs: Vec<_> = if let Some(ref pid) = project_id {
        documents
            .iter()
            .filter(|d| {
                d["project"]["id"].as_str() == Some(pid.as_str())
                    || d["project"]["name"].as_str().map(|n| n.to_lowercase()) == Some(pid.to_lowercase())
            })
            .collect()
    } else {
        documents.iter().collect()
    };

    if filtered_docs.is_empty() {
        println!("No documents found.");
        return Ok(());
    }

    let rows: Vec<DocumentRow> = filtered_docs
        .iter()
        .map(|d| {
            let updated = d["updatedAt"]
                .as_str()
                .unwrap_or("")
                .chars()
                .take(10)
                .collect::<String>();

            DocumentRow {
                title: d["title"].as_str().unwrap_or("").to_string(),
                project: d["project"]["name"].as_str().unwrap_or("-").to_string(),
                updated,
                id: d["id"].as_str().unwrap_or("").to_string(),
            }
        })
        .collect();

    let table = Table::new(rows).to_string();
    println!("{}", table);
    println!("\n{} documents", filtered_docs.len());

    Ok(())
}

async fn get_document(id: &str) -> Result<()> {
    let client = LinearClient::new()?;

    let query = r#"
        query($id: String!) {
            document(id: $id) {
                id
                title
                content
                icon
                color
                url
                createdAt
                updatedAt
                creator { name email }
                project { id name }
            }
        }
    "#;

    let result = client.query(query, Some(json!({ "id": id }))).await?;
    let document = &result["data"]["document"];

    if document.is_null() {
        anyhow::bail!("Document not found: {}", id);
    }

    println!("{}", document["title"].as_str().unwrap_or("").bold());
    println!("{}", "─".repeat(40));

    if let Some(project_name) = document["project"]["name"].as_str() {
        println!("Project: {}", project_name);
    }

    if let Some(creator_name) = document["creator"]["name"].as_str() {
        println!("Creator: {}", creator_name);
    }

    if let Some(icon) = document["icon"].as_str() {
        println!("Icon: {}", icon);
    }

    if let Some(color) = document["color"].as_str() {
        println!("Color: {}", color);
    }

    println!("URL: {}", document["url"].as_str().unwrap_or("-"));
    println!("ID: {}", document["id"].as_str().unwrap_or("-"));

    if let Some(created) = document["createdAt"].as_str() {
        println!("Created: {}", created.chars().take(10).collect::<String>());
    }

    if let Some(updated) = document["updatedAt"].as_str() {
        println!("Updated: {}", updated.chars().take(10).collect::<String>());
    }

    // Display content
    if let Some(content) = document["content"].as_str() {
        println!("\n{}", "Content".bold());
        println!("{}", "─".repeat(40));
        println!("{}", content);
    }

    Ok(())
}

async fn create_document(
    title: &str,
    project: &str,
    content: Option<String>,
    icon: Option<String>,
    color: Option<String>,
) -> Result<()> {
    let client = LinearClient::new()?;

    let mut input = json!({
        "title": title,
        "projectId": project
    });

    if let Some(c) = content {
        input["content"] = json!(c);
    }
    if let Some(i) = icon {
        input["icon"] = json!(i);
    }
    if let Some(col) = color {
        input["color"] = json!(col);
    }

    let mutation = r#"
        mutation($input: DocumentCreateInput!) {
            documentCreate(input: $input) {
                success
                document { id title url }
            }
        }
    "#;

    let result = client.mutate(mutation, Some(json!({ "input": input }))).await?;

    if result["data"]["documentCreate"]["success"].as_bool() == Some(true) {
        let document = &result["data"]["documentCreate"]["document"];
        println!("{} Created document: {}", "+".green(), document["title"].as_str().unwrap_or(""));
        println!("  ID: {}", document["id"].as_str().unwrap_or(""));
        println!("  URL: {}", document["url"].as_str().unwrap_or(""));
    } else {
        anyhow::bail!("Failed to create document");
    }

    Ok(())
}

async fn update_document(
    id: &str,
    title: Option<String>,
    content: Option<String>,
    icon: Option<String>,
    color: Option<String>,
    project: Option<String>,
) -> Result<()> {
    let client = LinearClient::new()?;

    let mut input = json!({});
    if let Some(t) = title {
        input["title"] = json!(t);
    }
    if let Some(c) = content {
        input["content"] = json!(c);
    }
    if let Some(i) = icon {
        input["icon"] = json!(i);
    }
    if let Some(col) = color {
        input["color"] = json!(col);
    }
    if let Some(p) = project {
        input["projectId"] = json!(p);
    }

    if input.as_object().map(|o| o.is_empty()).unwrap_or(true) {
        println!("No updates specified.");
        return Ok(());
    }

    let mutation = r#"
        mutation($id: String!, $input: DocumentUpdateInput!) {
            documentUpdate(id: $id, input: $input) {
                success
                document { id title }
            }
        }
    "#;

    let result = client.mutate(mutation, Some(json!({ "id": id, "input": input }))).await?;

    if result["data"]["documentUpdate"]["success"].as_bool() == Some(true) {
        println!("{} Document updated", "+".green());
    } else {
        anyhow::bail!("Failed to update document");
    }

    Ok(())
}
