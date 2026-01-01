use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::config;

const LINEAR_API_URL: &str = "https://api.linear.app/graphql";

pub struct LinearClient {
    client: Client,
    api_key: String,
}

impl LinearClient {
    pub fn new() -> Result<Self> {
        let api_key = config::get_api_key()?;
        Ok(Self {
            client: Client::new(),
            api_key,
        })
    }

    pub async fn query(&self, query: &str, variables: Option<Value>) -> Result<Value> {
        let body = match variables {
            Some(vars) => json!({ "query": query, "variables": vars }),
            None => json!({ "query": query }),
        };

        let response = self
            .client
            .post(LINEAR_API_URL)
            .header("Content-Type", "application/json")
            .header("Authorization", &self.api_key)
            .json(&body)
            .send()
            .await?;

        let result: Value = response.json().await?;

        if let Some(errors) = result.get("errors") {
            anyhow::bail!("GraphQL error: {}", errors);
        }

        Ok(result)
    }

    pub async fn mutate(&self, mutation: &str, variables: Option<Value>) -> Result<Value> {
        self.query(mutation, variables).await
    }
}

// Response types
#[derive(Debug, Deserialize, Serialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub summary: Option<String>,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub url: Option<String>,
    pub state: Option<String>,
    #[serde(default)]
    pub labels: Vec<ProjectLabel>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectLabel {
    pub id: String,
    pub name: String,
    pub color: Option<String>,
    pub parent: Option<ParentLabel>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ParentLabel {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Issue {
    pub id: String,
    pub identifier: String,
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<i32>,
    pub state: Option<IssueState>,
    pub url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IssueState {
    pub name: String,
    pub color: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Team {
    pub id: String,
    pub name: String,
    pub key: String,
}
