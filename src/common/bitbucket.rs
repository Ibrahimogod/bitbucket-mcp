use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct BitbucketCommentContent {
    pub raw: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BitbucketCommentPayload {
    pub content: BitbucketCommentContent,
}

pub fn normalize_comment_input(body: serde_json::Value) -> Result<BitbucketCommentPayload, String> {
    if let Some(content) = body.get("content") {
        if let Some(raw) = content.get("raw") {
            return Ok(BitbucketCommentPayload {
                content: BitbucketCommentContent {
                    raw: raw.as_str().unwrap_or("").to_string(),
                },
            });
        }
    }
    if let Some(raw) = body.get("body") {
        return Ok(BitbucketCommentPayload {
            content: BitbucketCommentContent {
                raw: raw.as_str().unwrap_or("").to_string(),
            },
        });
    }
    if let Some(s) = body.as_str() {
        return Ok(BitbucketCommentPayload {
            content: BitbucketCommentContent {
                raw: s.to_string(),
            },
        });
    }
    Err("Invalid comment input format".to_string())
}
// Bitbucket MCP Tool Implementation
// This module provides MCP tools for Bitbucket Cloud REST API integration.
// Credentials are fetched from environment variables: BITBUCKET_USERNAME, BITBUCKET_APP_PASSWORD

use std::env;
use anyhow::{Result, anyhow};
use reqwest::{Client};
use rmcp::{Error as McpError, ServerHandler, model::*, schemars, tool};

#[derive(Clone)]
pub struct BitbucketClient {
    pub api_username: String,
    pub app_password: String,
    pub client: Client,
    pub base_url: String,
}

impl BitbucketClient {
    // --- Pull Requests ---
    /// Create a bitbucket pull request
    pub async fn create_pullrequest(&self, workspace: &str, repo_slug: &str, body: serde_json::Value) -> Result<serde_json::Value> {
        let url = format!("{}/repositories/{}/{}/pullrequests", self.base_url, workspace, repo_slug);
        let req = self.client.post(&url).json(&body);
        let resp = self.apply_auth(req).send().await?;
        Ok(resp.json().await?)
    }

    /// Get bitbucket pull request details
    pub async fn get_pullrequest(&self, workspace: &str, repo_slug: &str, pr_id: &str) -> Result<serde_json::Value> {
        let url = format!("{}/repositories/{}/{}/pullrequests/{}", self.base_url, workspace, repo_slug, pr_id);
        let req = self.client.get(&url);
        let resp = self.apply_auth(req).send().await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(anyhow!("Bitbucket API error: {} - {}", status, text));
        }
        Ok(resp.json().await?)
    }

    /// Update a bitbucket pull request
    pub async fn update_pullrequest(&self, workspace: &str, repo_slug: &str, pr_id: &str, body: serde_json::Value) -> Result<serde_json::Value> {
        let url = format!("{}/repositories/{}/{}/pullrequests/{}", self.base_url, workspace, repo_slug, pr_id);
        let req = self.client.put(&url).json(&body);
        let resp = self.apply_auth(req).send().await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(anyhow!("Bitbucket API error: {} - {}", status, text));
        }
        Ok(resp.json().await?)
    }

    /// Approve a bitbucket pull request
    pub async fn approve_pullrequest(&self, workspace: &str, repo_slug: &str, pr_id: &str) -> Result<serde_json::Value> {
        let url = format!("{}/repositories/{}/{}/pullrequests/{}/approve", self.base_url, workspace, repo_slug, pr_id);
        let req = self.client.post(&url);
        let resp = self.apply_auth(req).send().await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(anyhow!("Bitbucket API error: {} - {}", status, text));
        }
        Ok(resp.json().await?)
    }

    /// Unapprove a bitbucket pull request
    pub async fn unapprove_pullrequest(&self, workspace: &str, repo_slug: &str, pr_id: &str) -> Result<serde_json::Value> {
        let url = format!("{}/repositories/{}/{}/pullrequests/{}/approve", self.base_url, workspace, repo_slug, pr_id);
        let req = self.client.delete(&url);
        let resp = self.apply_auth(req).send().await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(anyhow!("Bitbucket API error: {} - {}", status, text));
        }
        Ok(resp.json().await?)
    }

    /// Decline a bitbucket pull request
    pub async fn decline_pullrequest(&self, workspace: &str, repo_slug: &str, pr_id: &str) -> Result<serde_json::Value> {
        let url = format!("{}/repositories/{}/{}/pullrequests/{}/decline", self.base_url, workspace, repo_slug, pr_id);
        let req = self.client.post(&url);
        let resp = self.apply_auth(req).send().await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(anyhow!("Bitbucket API error: {} - {}", status, text));
        }
        Ok(resp.json().await?)
    }

    /// Merge a bitbucket pull request
    pub async fn merge_pullrequest(&self, workspace: &str, repo_slug: &str, pr_id: &str, body: Option<serde_json::Value>) -> Result<serde_json::Value> {
        let url = format!("{}/repositories/{}/{}/pullrequests/{}/merge", self.base_url, workspace, repo_slug, pr_id);
        let req = if let Some(b) = body {
            self.client.post(&url).json(&b)
        } else {
            self.client.post(&url)
        };
        let resp = self.apply_auth(req).send().await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(anyhow!("Bitbucket API error: {} - {}", status, text));
        }
        Ok(resp.json().await?)
    }

    /// List bitbucket pull request comments
    pub async fn list_pullrequest_comments(&self, workspace: &str, repo_slug: &str, pr_id: &str) -> Result<serde_json::Value> {
        let url = format!("{}/repositories/{}/{}/pullrequests/{}/comments", self.base_url, workspace, repo_slug, pr_id);
        let req = self.client.get(&url);
        let resp = self.apply_auth(req).send().await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(anyhow!("Bitbucket API error: {} - {}", status, text));
        }
        Ok(resp.json().await?)
    }

    /// Add a bitbucket pull request comment
    pub async fn add_pullrequest_comment(&self, workspace: &str, repo_slug: &str, pr_id: &str, body: BitbucketCommentPayload) -> Result<serde_json::Value> {
        let url = format!("{}/repositories/{}/{}/pullrequests/{}/comments", self.base_url, workspace, repo_slug, pr_id);
        let req = self.client.post(&url).json(&body);
        let resp = self.apply_auth(req).send().await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(anyhow!("Bitbucket API error: {} - {}", status, text));
        }
        Ok(resp.json().await?)
    }

    /// List bitbucket pull request activity
    pub async fn list_pullrequest_activity(&self, workspace: &str, repo_slug: &str, pr_id: &str) -> Result<serde_json::Value> {
        let url = format!("{}/repositories/{}/{}/pullrequests/{}/activity", self.base_url, workspace, repo_slug, pr_id);
        let req = self.client.get(&url);
        let resp = self.apply_auth(req).send().await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(anyhow!("Bitbucket API error: {} - {}", status, text));
        }
        Ok(resp.json().await?)
    }

    /// Get bitbucket pull request diff
    pub async fn get_pullrequest_diff(&self, workspace: &str, repo_slug: &str, pr_id: &str) -> Result<String> {
        let url = format!("{}/repositories/{}/{}/pullrequests/{}/diff", self.base_url, workspace, repo_slug, pr_id);
        let req = self.client.get(&url);
        let resp = self.apply_auth(req).send().await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(anyhow!("Bitbucket API error: {} - {}", status, text));
        }
        Ok(resp.text().await?)
    }

    /// Get bitbucket pull request commits
    pub async fn list_pullrequest_commits(&self, workspace: &str, repo_slug: &str, pr_id: &str) -> Result<serde_json::Value> {
        let url = format!("https://api.bitbucket.org/2.0/repositories/{}/{}/pullrequests/{}/commits", workspace, repo_slug, pr_id);
        let req = self.client.get(&url);
        let resp = self.apply_auth(req).send().await?;
        Ok(resp.json().await?)
    }

    /// List bitbucket pull request tasks
    pub async fn list_pullrequest_tasks(&self, workspace: &str, repo_slug: &str, pr_id: &str) -> Result<serde_json::Value> {
        let url = format!("{}/repositories/{}/{}/pullrequests/{}/tasks", self.base_url, workspace, repo_slug, pr_id);
        let req = self.client.get(&url);
        let resp = self.apply_auth(req).send().await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(anyhow!("Bitbucket API error: {} - {}", status, text));
        }
        Ok(resp.json().await?)
    }

    /// Add a bitbucket pull request task
    pub async fn add_pullrequest_task(&self, workspace: &str, repo_slug: &str, pr_id: &str, body: serde_json::Value) -> Result<serde_json::Value> {
        let url = format!("{}/repositories/{}/{}/pullrequests/{}/tasks", self.base_url, workspace, repo_slug, pr_id);
        let req = self.client.post(&url).json(&body);
        let resp = self.apply_auth(req).send().await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(anyhow!("Bitbucket API error: {} - {}", status, text));
        }
        Ok(resp.json().await?)
    }

    /// Get bitbucket pull request diffstat
    pub async fn get_pullrequest_diffstat(&self, workspace: &str, repo_slug: &str, pr_id: &str) -> Result<serde_json::Value> {
        let url = format!("{}/repositories/{}/{}/pullrequests/{}/diffstat", self.base_url, workspace, repo_slug, pr_id);
        let req = self.client.get(&url);
        let resp = self.apply_auth(req).send().await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(anyhow!("Bitbucket API error: {} - {}", status, text));
        }
        Ok(resp.json().await?)
    }
    pub fn from_env() -> Result<Self> {
        let api_username = env::var("BITBUCKET_API_USERNAME")
            .map_err(|_| anyhow!("BITBUCKET_API_USERNAME env var not set. Please set it to your Atlassian email."))?;
        let app_password = env::var("BITBUCKET_APP_PASSWORD")
            .map_err(|_| anyhow!("BITBUCKET_APP_PASSWORD env var not set. Please set it to your Bitbucket API token."))?;
        Ok(Self {
            api_username,
            app_password,
            client: Client::new(),
            base_url: "https://api.bitbucket.org/2.0".to_string(),
        })
    }

    fn apply_auth<'a>(&self, req: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        req.basic_auth(&self.api_username, Some(&self.app_password))
    }

    pub async fn get_user(&self) -> Result<serde_json::Value> {
        let url = format!("{}/user", self.base_url);
        let req = self.client.get(&url);
        let resp = self.apply_auth(req).send().await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(anyhow!("Bitbucket API error: {} - {}", status, text));
        }
        Ok(resp.json().await?)
    }

    pub async fn list_workspaces(&self) -> Result<serde_json::Value> {
        let url = format!("{}/workspaces", self.base_url);
        let req = self.client.get(&url);
        let resp = self.apply_auth(req).send().await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(anyhow!("Bitbucket API error: {} - {}", status, text));
        }
        Ok(resp.json().await?)
    }

    pub async fn list_repositories(&self, workspace: &str) -> Result<serde_json::Value> {
        let url = format!("{}/repositories/{}", self.base_url, workspace);
        let req = self.client.get(&url);
        let resp = self.apply_auth(req).send().await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(anyhow!("Bitbucket API error: {} - {}", status, text));
        }
        Ok(resp.json().await?)
    }

    pub async fn list_pullrequests(&self, workspace: &str, repo_slug: &str) -> Result<serde_json::Value> {
        let url = format!("{}/repositories/{}/{}/pullrequests", self.base_url, workspace, repo_slug);
        let req = self.client.get(&url);
        let resp = self.apply_auth(req).send().await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(anyhow!("Bitbucket API error: {} - {}", status, text));
        }
        Ok(resp.json().await?)
    }

    pub async fn list_issues(&self, workspace: &str, repo_slug: &str) -> Result<serde_json::Value> {
        let url = format!("{}/repositories/{}/{}/issues", self.base_url, workspace, repo_slug);
        let req = self.client.get(&url);
        let resp = self.apply_auth(req).send().await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(anyhow!("Bitbucket API error: {} - {}", status, text));
        }
        Ok(resp.json().await?)
    }

    pub async fn get_workspace(&self, workspace: &str) -> Result<serde_json::Value> {
        let url = format!("{}/workspaces/{}", self.base_url, workspace);
        let req = self.client.get(&url);
        let resp = self.apply_auth(req).send().await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(anyhow!("Bitbucket API error: {} - {}", status, text));
        }
        Ok(resp.json().await?)
    }
    pub async fn get_repository(&self, workspace: &str, repo_slug: &str) -> Result<serde_json::Value> {
        let url = format!("{}/repositories/{}/{}", self.base_url, workspace, repo_slug);
        let req = self.client.get(&url);
        let resp = self.apply_auth(req).send().await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(anyhow!("Bitbucket API error: {} - {}", status, text));
        }
        Ok(resp.json().await?)
    }
    pub async fn list_branches(&self, workspace: &str, repo_slug: &str) -> Result<serde_json::Value> {
        let url = format!("https://api.bitbucket.org/2.0/repositories/{}/{}/refs/branches", workspace, repo_slug);
        let req = self.client.get(&url);
        let resp = self.apply_auth(req).send().await?;
        Ok(resp.json().await?)
    }
    pub async fn list_tags(&self, workspace: &str, repo_slug: &str) -> Result<serde_json::Value> {
        let url = format!("https://api.bitbucket.org/2.0/repositories/{}/{}/refs/tags", workspace, repo_slug);
        let req = self.client.get(&url);
        let resp = self.apply_auth(req).send().await?;
        Ok(resp.json().await?)
    }
    pub async fn list_commits(&self, workspace: &str, repo_slug: &str) -> Result<serde_json::Value> {
        let url = format!("https://api.bitbucket.org/2.0/repositories/{}/{}/commits", workspace, repo_slug);
        let req = self.client.get(&url);
        let resp = self.apply_auth(req).send().await?;
        Ok(resp.json().await?)
    }
    pub async fn list_pipelines(&self, workspace: &str, repo_slug: &str) -> Result<serde_json::Value> {
        let url = format!("https://api.bitbucket.org/2.0/repositories/{}/{}/pipelines/", workspace, repo_slug);
        let req = self.client.get(&url);
        let resp = self.apply_auth(req).send().await?;
        Ok(resp.json().await?)
    }
    pub async fn list_deployments(&self, workspace: &str, repo_slug: &str) -> Result<serde_json::Value> {
        let url = format!("https://api.bitbucket.org/2.0/repositories/{}/{}/deployments/", workspace, repo_slug);
        let req = self.client.get(&url);
        let resp = self.apply_auth(req).send().await?;
        Ok(resp.json().await?)
    }
    pub async fn list_downloads(&self, workspace: &str, repo_slug: &str) -> Result<serde_json::Value> {
        let url = format!("https://api.bitbucket.org/2.0/repositories/{}/{}/downloads", workspace, repo_slug);
        let req = self.client.get(&url);
        let resp = self.apply_auth(req).send().await?;
        Ok(resp.json().await?)
    }
    pub async fn list_webhooks(&self, workspace: &str, repo_slug: &str) -> Result<serde_json::Value> {
        let url = format!("https://api.bitbucket.org/2.0/repositories/{}/{}/hooks", workspace, repo_slug);
        let req = self.client.get(&url);
        let resp = self.apply_auth(req).send().await?;
        Ok(resp.json().await?)
    }
    pub async fn list_snippets(&self, workspace: &str) -> Result<serde_json::Value> {
        let url = format!("https://api.bitbucket.org/2.0/snippets/{}", workspace);
        let req = self.client.get(&url);
        let resp = self.apply_auth(req).send().await?;
        Ok(resp.json().await?)
    }
    pub async fn list_projects(&self, workspace: &str) -> Result<serde_json::Value> {
        let url = format!("https://api.bitbucket.org/2.0/workspaces/{}/projects", workspace);
        let req = self.client.get(&url);
        let resp = self.apply_auth(req).send().await?;
        Ok(resp.json().await?)
    }
    pub async fn list_branch_restrictions(&self, workspace: &str, repo_slug: &str) -> Result<serde_json::Value> {
        let url = format!("https://api.bitbucket.org/2.0/repositories/{}/{}/branch-restrictions", workspace, repo_slug);
        let req = self.client.get(&url);
        let resp = self.apply_auth(req).send().await?;
        Ok(resp.json().await?)
    }
    pub async fn list_commit_statuses(&self, workspace: &str, repo_slug: &str, commit: &str) -> Result<serde_json::Value> {
        let url = format!("https://api.bitbucket.org/2.0/repositories/{}/{}/commit/{}/statuses", workspace, repo_slug, commit);
        let req = self.client.get(&url);
        let resp = self.apply_auth(req).send().await?;
        Ok(resp.json().await?)
    }
    pub async fn list_users(&self, workspace: &str) -> Result<serde_json::Value> {
        let url = format!("https://api.bitbucket.org/2.0/workspaces/{}/members", workspace);
        let req = self.client.get(&url);
        let resp = self.apply_auth(req).send().await?;
        Ok(resp.json().await?)
    }

    /// Create a repository in a workspace
    pub async fn create_repository(&self, workspace: &str, repo_slug: &str, body: serde_json::Value) -> Result<serde_json::Value> {
        let url = format!("https://api.bitbucket.org/2.0/repositories/{}/{}", workspace, repo_slug);
        let req = self.client.post(&url).json(&body);
        let resp = self.apply_auth(req).send().await?;
        Ok(resp.json().await?)
    }

    /// Update a repository in a workspace
    pub async fn update_repository(&self, workspace: &str, repo_slug: &str, body: serde_json::Value) -> Result<serde_json::Value> {
        let url = format!("https://api.bitbucket.org/2.0/repositories/{}/{}", workspace, repo_slug);
        let req = self.client.put(&url).json(&body);
        let resp = self.apply_auth(req).send().await?;
        Ok(resp.json().await?)
    }

    /// Delete a repository in a workspace
    pub async fn delete_repository(&self, workspace: &str, repo_slug: &str) -> Result<serde_json::Value> {
        let url = format!("{}/repositories/{}/{}", self.base_url, workspace, repo_slug);
        let req = self.client.delete(&url);
        let resp = self.apply_auth(req).send().await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(anyhow!("Bitbucket API error: {} - {}", status, text));
        }
        if resp.status() == reqwest::StatusCode::NO_CONTENT {
            return Ok(serde_json::json!({}));
        }
        Ok(resp.json().await?)
    }
    // --- Branches ---
    /// Create a branch in a repository
    pub async fn create_branch(&self, workspace: &str, repo_slug: &str, body: serde_json::Value) -> Result<serde_json::Value> {
        let url = format!("https://api.bitbucket.org/2.0/repositories/{}/{}/refs/branches", workspace, repo_slug);
        let req = self.client.post(&url).json(&body);
        let resp = self.apply_auth(req).send().await?;
        Ok(resp.json().await?)
    }
    /// Delete a branch in a repository
    pub async fn delete_branch(&self, workspace: &str, repo_slug: &str, branch: &str) -> Result<serde_json::Value> {
        let url = format!("{}/repositories/{}/{}/refs/branches/{}", self.base_url, workspace, repo_slug, branch);
        let req = self.client.delete(&url);
        let resp = self.apply_auth(req).send().await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(anyhow!("Bitbucket API error: {} - {}", status, text));
        }
        if resp.status() == reqwest::StatusCode::NO_CONTENT {
            return Ok(serde_json::json!({}));
        }
        Ok(resp.json().await?)
    }
    // --- Branching Model ---
    pub async fn get_branching_model(&self, workspace: &str, repo_slug: &str) -> Result<serde_json::Value> {
        let url = format!("https://api.bitbucket.org/2.0/repositories/{}/{}/branching-model", workspace, repo_slug);
        let req = self.client.get(&url);
        let resp = self.apply_auth(req).send().await?;
        Ok(resp.json().await?)
    }
    pub async fn update_branching_model(&self, workspace: &str, repo_slug: &str, body: serde_json::Value) -> Result<serde_json::Value> {
        let url = format!("https://api.bitbucket.org/2.0/repositories/{}/{}/branching-model", workspace, repo_slug);
        let req = self.client.put(&url).json(&body);
        let resp = self.apply_auth(req).send().await?;
        Ok(resp.json().await?)
    }
    // --- Commit Statuses ---
    pub async fn create_commit_status(&self, workspace: &str, repo_slug: &str, commit: &str, body: serde_json::Value) -> Result<serde_json::Value> {
        let url = format!("https://api.bitbucket.org/2.0/repositories/{}/{}/commit/{}/statuses/build", workspace, repo_slug, commit);
        let req = self.client.post(&url).json(&body);
        let resp = self.apply_auth(req).send().await?;
        Ok(resp.json().await?)
    }
    // --- Commits ---
    pub async fn get_commit(&self, workspace: &str, repo_slug: &str, commit: &str) -> Result<serde_json::Value> {
        let url = format!("https://api.bitbucket.org/2.0/repositories/{}/{}/commit/{}", workspace, repo_slug, commit);
        let req = self.client.get(&url);
        let resp = self.apply_auth(req).send().await?;
        Ok(resp.json().await?)
    }
    // --- Deployments ---
    pub async fn create_deployment(&self, workspace: &str, repo_slug: &str, body: serde_json::Value) -> Result<serde_json::Value> {
        let url = format!("https://api.bitbucket.org/2.0/repositories/{}/{}/deployments/", workspace, repo_slug);
        let req = self.client.post(&url).json(&body);
        let resp = self.apply_auth(req).send().await?;
        Ok(resp.json().await?)
    }
    // --- Issue Tracker ---
    pub async fn create_issue(&self, workspace: &str, repo_slug: &str, body: serde_json::Value) -> Result<serde_json::Value> {
        let url = format!("https://api.bitbucket.org/2.0/repositories/{}/{}/issues", workspace, repo_slug);
        let req = self.client.post(&url).json(&body);
        let resp = self.apply_auth(req).send().await?;
        Ok(resp.json().await?)
    }
    pub async fn update_issue(&self, workspace: &str, repo_slug: &str, issue_id: &str, body: serde_json::Value) -> Result<serde_json::Value> {
        let url = format!("https://api.bitbucket.org/2.0/repositories/{}/{}/issues/{}", workspace, repo_slug, issue_id);
        let req = self.client.put(&url).json(&body);
        let resp = self.apply_auth(req).send().await?;
        Ok(resp.json().await?)
    }
    pub async fn delete_issue(&self, workspace: &str, repo_slug: &str, issue_id: &str) -> Result<serde_json::Value> {
        let url = format!("{}/repositories/{}/{}/issues/{}", self.base_url, workspace, repo_slug, issue_id);
        let req = self.client.delete(&url);
        let resp = self.apply_auth(req).send().await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(anyhow!("Bitbucket API error: {} - {}", status, text));
        }
        if resp.status() == reqwest::StatusCode::NO_CONTENT {
            return Ok(serde_json::json!({}));
        }
        Ok(resp.json().await?)
    }
    // --- Pipelines ---
    pub async fn trigger_pipeline(&self, workspace: &str, repo_slug: &str, body: serde_json::Value) -> Result<serde_json::Value> {
        let url = format!("https://api.bitbucket.org/2.0/repositories/{}/{}/pipelines/", workspace, repo_slug);
        let req = self.client.post(&url).json(&body);
        let resp = self.apply_auth(req).send().await?;
        Ok(resp.json().await?)
    }
    // --- Projects ---
    pub async fn create_project(&self, workspace: &str, body: serde_json::Value) -> Result<serde_json::Value> {
        let url = format!("https://api.bitbucket.org/2.0/workspaces/{}/projects", workspace);
        let req = self.client.post(&url).json(&body);
        let resp = self.apply_auth(req).send().await?;
        Ok(resp.json().await?)
    }
    pub async fn update_project(&self, workspace: &str, project_key: &str, body: serde_json::Value) -> Result<serde_json::Value> {
        let url = format!("https://api.bitbucket.org/2.0/workspaces/{}/projects/{}", workspace, project_key);
        let req = self.client.put(&url).json(&body);
        let resp = self.apply_auth(req).send().await?;
        Ok(resp.json().await?)
    }
    pub async fn delete_project(&self, workspace: &str, project_key: &str) -> Result<serde_json::Value> {
        let url = format!("{}/workspaces/{}/projects/{}", self.base_url, workspace, project_key);
        let req = self.client.delete(&url);
        let resp = self.apply_auth(req).send().await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(anyhow!("Bitbucket API error: {} - {}", status, text));
        }
        if resp.status() == reqwest::StatusCode::NO_CONTENT {
            return Ok(serde_json::json!({}));
        }
        Ok(resp.json().await?)
    }
    // --- Snippets ---
    pub async fn create_snippet(&self, workspace: &str, body: serde_json::Value) -> Result<serde_json::Value> {
        let url = format!("https://api.bitbucket.org/2.0/snippets/{}", workspace);
        let req = self.client.post(&url).json(&body);
        let resp = self.apply_auth(req).send().await?;
        Ok(resp.json().await?)
    }
    pub async fn delete_snippet(&self, workspace: &str, snippet_id: &str) -> Result<serde_json::Value> {
        let url = format!("{}/snippets/{}/{}", self.base_url, workspace, snippet_id);
        let req = self.client.delete(&url);
        let resp = self.apply_auth(req).send().await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(anyhow!("Bitbucket API error: {} - {}", status, text));
        }
        if resp.status() == reqwest::StatusCode::NO_CONTENT {
            return Ok(serde_json::json!({}));
        }
        Ok(resp.json().await?)
    }
    // --- Source ---
    pub async fn get_file_source(&self, workspace: &str, repo_slug: &str, commit: &str, path: &str) -> Result<serde_json::Value> {
        let url = format!("https://api.bitbucket.org/2.0/repositories/{}/{}/src/{}/{}", workspace, repo_slug, commit, path);
        let req = self.client.get(&url);
        let resp = self.apply_auth(req).send().await?;
        Ok(resp.json().await?)
    }
    // Add more methods for each Bitbucket REST API group here
}

// MCP tool trait implementation and registration will be added here

#[derive(Clone)]
pub struct BitbucketTool;

#[tool(tool_box)]
impl BitbucketTool {
    #[tool(description = "Create a bitbucket pull request")]
    pub async fn create_pullrequest(&self, #[tool(param)] workspace: String, #[tool(param)] repo_slug: String, #[tool(param)] body: serde_json::Value) -> Result<CallToolResult, McpError> {
        let client = match super::bitbucket::BitbucketClient::from_env() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("BitbucketClient::from_env error: {e}");
                return Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        };
        match client.create_pullrequest(&workspace, &repo_slug, body).await {
            Ok(val) => Ok(CallToolResult::success(vec![Content::json(val)?])),
            Err(e) => {
                tracing::error!("create_pullrequest error: {e}");
                Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        }
    }

    #[tool(description = "Get bitbucket pull request details")]
    pub async fn get_pullrequest(&self, #[tool(param)] workspace: String, #[tool(param)] repo_slug: String, #[tool(param)] pr_id: String) -> Result<CallToolResult, McpError> {
        let client = match super::bitbucket::BitbucketClient::from_env() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("BitbucketClient::from_env error: {e}");
                return Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        };
        match client.get_pullrequest(&workspace, &repo_slug, &pr_id).await {
            Ok(val) => Ok(CallToolResult::success(vec![Content::json(val)?])),
            Err(e) => {
                tracing::error!("get_pullrequest error: {e}");
                Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        }
    }

    #[tool(description = "Update a bitbucket pull request")]
    pub async fn update_pullrequest(&self, #[tool(param)] workspace: String, #[tool(param)] repo_slug: String, #[tool(param)] pr_id: String, #[tool(param)] body: serde_json::Value) -> Result<CallToolResult, McpError> {
        let client = match super::bitbucket::BitbucketClient::from_env() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("BitbucketClient::from_env error: {e}");
                return Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        };
        match client.update_pullrequest(&workspace, &repo_slug, &pr_id, body).await {
            Ok(val) => Ok(CallToolResult::success(vec![Content::json(val)?])),
            Err(e) => {
                tracing::error!("update_pullrequest error: {e}");
                Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        }
    }

    #[tool(description = "Approve a bitbucket pull request")]
    pub async fn approve_pullrequest(&self, #[tool(param)] workspace: String, #[tool(param)] repo_slug: String, #[tool(param)] pr_id: String) -> Result<CallToolResult, McpError> {
        let client = match super::bitbucket::BitbucketClient::from_env() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("BitbucketClient::from_env error: {e}");
                return Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        };
        match client.approve_pullrequest(&workspace, &repo_slug, &pr_id).await {
            Ok(val) => Ok(CallToolResult::success(vec![Content::json(val)?])),
            Err(e) => {
                tracing::error!("approve_pullrequest error: {e}");
                Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        }
    }

    #[tool(description = "Unapprove a bitbucket pull request")]
    pub async fn unapprove_pullrequest(&self, #[tool(param)] workspace: String, #[tool(param)] repo_slug: String, #[tool(param)] pr_id: String) -> Result<CallToolResult, McpError> {
        let client = match super::bitbucket::BitbucketClient::from_env() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("BitbucketClient::from_env error: {e}");
                return Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        };
        match client.unapprove_pullrequest(&workspace, &repo_slug, &pr_id).await {
            Ok(val) => Ok(CallToolResult::success(vec![Content::json(val)?])),
            Err(e) => {
                tracing::error!("unapprove_pullrequest error: {e}");
                Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        }
    }

    #[tool(description = "Decline a bitbucket pull request")]
    pub async fn decline_pullrequest(&self, #[tool(param)] workspace: String, #[tool(param)] repo_slug: String, #[tool(param)] pr_id: String) -> Result<CallToolResult, McpError> {
        let client = match super::bitbucket::BitbucketClient::from_env() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("BitbucketClient::from_env error: {e}");
                return Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        };
        match client.decline_pullrequest(&workspace, &repo_slug, &pr_id).await {
            Ok(val) => Ok(CallToolResult::success(vec![Content::json(val)?])),
            Err(e) => {
                tracing::error!("decline_pullrequest error: {e}");
                Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        }
    }

    #[tool(description = "Merge a bitbucket pull request")]
    pub async fn merge_pullrequest(&self, #[tool(param)] workspace: String, #[tool(param)] repo_slug: String, #[tool(param)] pr_id: String, #[tool(param)] body: Option<serde_json::Value>) -> Result<CallToolResult, McpError> {
        let client = match super::bitbucket::BitbucketClient::from_env() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("BitbucketClient::from_env error: {e}");
                return Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        };
        match client.merge_pullrequest(&workspace, &repo_slug, &pr_id, body).await {
            Ok(val) => Ok(CallToolResult::success(vec![Content::json(val)?])),
            Err(e) => {
                tracing::error!("merge_pullrequest error: {e}");
                Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        }
    }

    #[tool(description = "List bitbucket pull request comments")]
    pub async fn list_pullrequest_comments(&self, #[tool(param)] workspace: String, #[tool(param)] repo_slug: String, #[tool(param)] pr_id: String) -> Result<CallToolResult, McpError> {
        let client = match super::bitbucket::BitbucketClient::from_env() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("BitbucketClient::from_env error: {e}");
                return Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        };
        match client.list_pullrequest_comments(&workspace, &repo_slug, &pr_id).await {
            Ok(val) => Ok(CallToolResult::success(vec![Content::json(val)?])),
            Err(e) => {
                tracing::error!("list_pullrequest_comments error: {e}");
                Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        }
    }

    #[tool(description = "Add a bitbucket pull request comment")]
    pub async fn add_pullrequest_comment(&self, #[tool(param)] workspace: String, #[tool(param)] repo_slug: String, #[tool(param)] pr_id: String, #[tool(param)] body: serde_json::Value) -> Result<CallToolResult, McpError> {
        let payload = match normalize_comment_input(body) {
            Ok(p) => p,
            Err(e) => return Ok(CallToolResult::error(vec![Content::text(e)])),
        };
        let client = match super::bitbucket::BitbucketClient::from_env() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("BitbucketClient::from_env error: {e}");
                return Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        };
        match client.add_pullrequest_comment(&workspace, &repo_slug, &pr_id, payload).await {
            Ok(val) => Ok(CallToolResult::success(vec![Content::json(val)?])),
            Err(e) => {
                tracing::error!("add_pullrequest_comment error: {e}");
                Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        }
    }

    #[tool(description = "List bitbucket pull request activity")]
    pub async fn list_pullrequest_activity(&self, #[tool(param)] workspace: String, #[tool(param)] repo_slug: String, #[tool(param)] pr_id: String) -> Result<CallToolResult, McpError> {
        let client = match super::bitbucket::BitbucketClient::from_env() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("BitbucketClient::from_env error: {e}");
                return Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        };
        match client.list_pullrequest_activity(&workspace, &repo_slug, &pr_id).await {
            Ok(val) => Ok(CallToolResult::success(vec![Content::json(val)?])),
            Err(e) => {
                tracing::error!("list_pullrequest_activity error: {e}");
                Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        }
    }

    #[tool(description = "Get bitbucket pull request diff")]
    pub async fn get_pullrequest_diff(&self, #[tool(param)] workspace: String, #[tool(param)] repo_slug: String, #[tool(param)] pr_id: String) -> Result<CallToolResult, McpError> {
        let client = match super::bitbucket::BitbucketClient::from_env() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("BitbucketClient::from_env error: {e}");
                return Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        };
        match client.get_pullrequest_diff(&workspace, &repo_slug, &pr_id).await {
            Ok(val) => Ok(CallToolResult::success(vec![Content::text(val)])),
            Err(e) => {
                tracing::error!("get_pullrequest_diff error: {e}");
                Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        }
    }

    #[tool(description = "Get bitbucket pull request commits")]
    pub async fn list_pullrequest_commits(&self, #[tool(param)] workspace: String, #[tool(param)] repo_slug: String, #[tool(param)] pr_id: String) -> Result<CallToolResult, McpError> {
        let client = match super::bitbucket::BitbucketClient::from_env() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("BitbucketClient::from_env error: {e}");
                return Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        };
        match client.list_pullrequest_commits(&workspace, &repo_slug, &pr_id).await {
            Ok(val) => Ok(CallToolResult::success(vec![Content::json(val)?])),
            Err(e) => {
                tracing::error!("list_pullrequest_commits error: {e}");
                Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        }
    }

    #[tool(description = "List bitbucket pull request tasks")]
    pub async fn list_pullrequest_tasks(&self, #[tool(param)] workspace: String, #[tool(param)] repo_slug: String, #[tool(param)] pr_id: String) -> Result<CallToolResult, McpError> {
        let client = match super::bitbucket::BitbucketClient::from_env() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("BitbucketClient::from_env error: {e}");
                return Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        };
        match client.list_pullrequest_tasks(&workspace, &repo_slug, &pr_id).await {
            Ok(val) => Ok(CallToolResult::success(vec![Content::json(val)?])),
            Err(e) => {
                tracing::error!("list_pullrequest_tasks error: {e}");
                Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        }
    }

    #[tool(description = "Add a bitbucket pull request task")]
    pub async fn add_pullrequest_task(&self, #[tool(param)] workspace: String, #[tool(param)] repo_slug: String, #[tool(param)] pr_id: String, #[tool(param)] body: serde_json::Value) -> Result<CallToolResult, McpError> {
        let client = match super::bitbucket::BitbucketClient::from_env() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("BitbucketClient::from_env error: {e}");
                return Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        };
        match client.add_pullrequest_task(&workspace, &repo_slug, &pr_id, body).await {
            Ok(val) => Ok(CallToolResult::success(vec![Content::json(val)?])),
            Err(e) => {
                tracing::error!("add_pullrequest_task error: {e}");
                Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        }
    }

    #[tool(description = "Get bitbucket pull request diffstat")]
    pub async fn get_pullrequest_diffstat(&self, #[tool(param)] workspace: String, #[tool(param)] repo_slug: String, #[tool(param)] pr_id: String) -> Result<CallToolResult, McpError> {
        let client = match super::bitbucket::BitbucketClient::from_env() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("BitbucketClient::from_env error: {e}");
                return Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        };
        match client.get_pullrequest_diffstat(&workspace, &repo_slug, &pr_id).await {
            Ok(val) => Ok(CallToolResult::success(vec![Content::json(val)?])),
            Err(e) => {
                tracing::error!("get_pullrequest_diffstat error: {e}");
                Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        }
    }
    #[tool(description = "Get bitbucket user info")]
    pub async fn get_user(&self) -> Result<CallToolResult, McpError> {
        let client = match super::bitbucket::BitbucketClient::from_env() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("BitbucketClient::from_env error: {e}");
                return Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        };
        match client.get_user().await {
            Ok(val) => Ok(CallToolResult::success(vec![Content::json(val)?])),
            Err(e) => {
                tracing::error!("get_user error: {e}");
                Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        }
    }

    #[tool(description = "List bitbucket workspaces")]
    pub async fn list_workspaces(&self) -> Result<CallToolResult, McpError> {
        let client = match super::bitbucket::BitbucketClient::from_env() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("BitbucketClient::from_env error: {e}");
                return Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        };
        match client.list_workspaces().await {
            Ok(val) => Ok(CallToolResult::success(vec![Content::json(val)?])),
            Err(e) => {
                tracing::error!("list_workspaces error: {e}");
                Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        }
    }

    #[tool(description = "List bitbucket repositories in a workspace")]
    pub async fn list_repositories(&self, #[tool(param)] workspace: String) -> Result<CallToolResult, McpError> {
        let client = match super::bitbucket::BitbucketClient::from_env() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("BitbucketClient::from_env error: {e}");
                return Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        };
        match client.list_repositories(&workspace).await {
            Ok(val) => Ok(CallToolResult::success(vec![Content::json(val)?])),
            Err(e) => {
                tracing::error!("list_repositories error: {e}");
                Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        }
    }

    #[tool(description = "List bitbucket pull requests for a repository")]
    pub async fn list_pullrequests(&self, #[tool(param)] workspace: String, #[tool(param)] repo_slug: String) -> Result<CallToolResult, McpError> {
        tracing::info!("list_pullrequests called with workspace='{}', repo_slug='{}'", workspace, repo_slug);
        let client = match super::bitbucket::BitbucketClient::from_env() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("BitbucketClient::from_env error: {e}");
                return Ok(CallToolResult::error(vec![Content::text(format!("env error: {e}"))]))
            },
        };
        let result = client.list_pullrequests(&workspace, &repo_slug).await;
        match result {
            Ok(val) => {
                tracing::info!("list_pullrequests API call succeeded");
                Ok(CallToolResult::success(vec![Content::json(val)?]))
            },
            Err(e) => {
                tracing::error!("list_pullrequests API call error: {e}");
                Ok(CallToolResult::error(vec![Content::text(format!("api error: {e}"))]))
            },
        }
    }

    #[tool(description = "List bitbucket issues for a repository")]
    pub async fn list_issues(&self, #[tool(param)] workspace: String, #[tool(param)] repo_slug: String) -> Result<CallToolResult, McpError> {
        let client = match super::bitbucket::BitbucketClient::from_env() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("BitbucketClient::from_env error: {e}");
                return Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        };
        match client.list_issues(&workspace, &repo_slug).await {
            Ok(val) => Ok(CallToolResult::success(vec![Content::json(val)?])),
            Err(e) => {
                tracing::error!("list_issues error: {e}");
                Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        }
    }

    #[tool(description = "Get bitbucket workspace details")]
    pub async fn get_workspace(&self, #[tool(param)] workspace: String) -> Result<CallToolResult, McpError> {
        let client = match super::bitbucket::BitbucketClient::from_env() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("BitbucketClient::from_env error: {e}");
                return Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        };
        match client.get_workspace(&workspace).await {
            Ok(val) => Ok(CallToolResult::success(vec![Content::json(val)?])),
            Err(e) => {
                tracing::error!("get_workspace error: {e}");
                Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        }
    }

    #[tool(description = "Get bitbucket repository details")]
    pub async fn get_repository(&self, #[tool(param)] workspace: String, #[tool(param)] repo_slug: String) -> Result<CallToolResult, McpError> {
        let client = match super::bitbucket::BitbucketClient::from_env() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("BitbucketClient::from_env error: {e}");
                return Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        };
        match client.get_repository(&workspace, &repo_slug).await {
            Ok(val) => Ok(CallToolResult::success(vec![Content::json(val)?])),
            Err(e) => {
                tracing::error!("get_repository error: {e}");
                Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        }
    }

    #[tool(description = "List bitbucket branches for a repository")]
    pub async fn list_branches(&self, #[tool(param)] workspace: String, #[tool(param)] repo_slug: String) -> Result<CallToolResult, McpError> {
        let client = match super::bitbucket::BitbucketClient::from_env() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("BitbucketClient::from_env error: {e}");
                return Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        };
        match client.list_branches(&workspace, &repo_slug).await {
            Ok(val) => Ok(CallToolResult::success(vec![Content::json(val)?])),
            Err(e) => {
                tracing::error!("list_branches error: {e}");
                Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        }
    }

    #[tool(description = "List bitbucket tags for a repository")]
    pub async fn list_tags(&self, #[tool(param)] workspace: String, #[tool(param)] repo_slug: String) -> Result<CallToolResult, McpError> {
        let client = match super::bitbucket::BitbucketClient::from_env() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("BitbucketClient::from_env error: {e}");
                return Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        };
        match client.list_tags(&workspace, &repo_slug).await {
            Ok(val) => Ok(CallToolResult::success(vec![Content::json(val)?])),
            Err(e) => {
                tracing::error!("list_tags error: {e}");
                Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        }
    }

    #[tool(description = "List bitbucket commits for a repository")]
    pub async fn list_commits(&self, #[tool(param)] workspace: String, #[tool(param)] repo_slug: String) -> Result<CallToolResult, McpError> {
        let client = match super::bitbucket::BitbucketClient::from_env() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("BitbucketClient::from_env error: {e}");
                return Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        };
        match client.list_commits(&workspace, &repo_slug).await {
            Ok(val) => Ok(CallToolResult::success(vec![Content::json(val)?])),
            Err(e) => {
                tracing::error!("list_commits error: {e}");
                Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        }
    }

    #[tool(description = "List bitbucket pipelines for a repository")]
    pub async fn list_pipelines(&self, #[tool(param)] workspace: String, #[tool(param)] repo_slug: String) -> Result<CallToolResult, McpError> {
        let client = match super::bitbucket::BitbucketClient::from_env() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("BitbucketClient::from_env error: {e}");
                return Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        };
        match client.list_pipelines(&workspace, &repo_slug).await {
            Ok(val) => Ok(CallToolResult::success(vec![Content::json(val)?])),
            Err(e) => {
                tracing::error!("list_pipelines error: {e}");
                Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        }
    }

    #[tool(description = "List bitbucket deployments for a repository")]
    pub async fn list_deployments(&self, #[tool(param)] workspace: String, #[tool(param)] repo_slug: String) -> Result<CallToolResult, McpError> {
        let client = match super::bitbucket::BitbucketClient::from_env() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("BitbucketClient::from_env error: {e}");
                return Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        };
        match client.list_deployments(&workspace, &repo_slug).await {
            Ok(val) => Ok(CallToolResult::success(vec![Content::json(val)?])),
            Err(e) => {
                tracing::error!("list_deployments error: {e}");
                Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        }
    }

    #[tool(description = "List bitbucket downloads for a repository")]
    pub async fn list_downloads(&self, #[tool(param)] workspace: String, #[tool(param)] repo_slug: String) -> Result<CallToolResult, McpError> {
        let client = match super::bitbucket::BitbucketClient::from_env() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("BitbucketClient::from_env error: {e}");
                return Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        };
        match client.list_downloads(&workspace, &repo_slug).await {
            Ok(val) => Ok(CallToolResult::success(vec![Content::json(val)?])),
            Err(e) => {
                tracing::error!("list_downloads error: {e}");
                Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        }
    }

    #[tool(description = "List bitbucket webhooks for a repository")]
    pub async fn list_webhooks(&self, #[tool(param)] workspace: String, #[tool(param)] repo_slug: String) -> Result<CallToolResult, McpError> {
        let client = match super::bitbucket::BitbucketClient::from_env() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("BitbucketClient::from_env error: {e}");
                return Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        };
        match client.list_webhooks(&workspace, &repo_slug).await {
            Ok(val) => Ok(CallToolResult::success(vec![Content::json(val)?])),
            Err(e) => {
                tracing::error!("list_webhooks error: {e}");
                Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        }
    }

    #[tool(description = "List bitbucket snippets for a workspace")]
    pub async fn list_snippets(&self, #[tool(param)] workspace: String) -> Result<CallToolResult, McpError> {
        let client = match super::bitbucket::BitbucketClient::from_env() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("BitbucketClient::from_env error: {e}");
                return Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        };
        match client.list_snippets(&workspace).await {
            Ok(val) => Ok(CallToolResult::success(vec![Content::json(val)?])),
            Err(e) => {
                tracing::error!("list_snippets error: {e}");
                Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        }
    }

    #[tool(description = "List bitbucket projects for a workspace")]
    pub async fn list_projects(&self, #[tool(param)] workspace: String) -> Result<CallToolResult, McpError> {
        let client = match super::bitbucket::BitbucketClient::from_env() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("BitbucketClient::from_env error: {e}");
                return Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        };
        match client.list_projects(&workspace).await {
            Ok(val) => Ok(CallToolResult::success(vec![Content::json(val)?])),
            Err(e) => {
                tracing::error!("list_projects error: {e}");
                Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        }
    }

    #[tool(description = "List bitbucket branch restrictions for a repository")]
    pub async fn list_branch_restrictions(&self, #[tool(param)] workspace: String, #[tool(param)] repo_slug: String) -> Result<CallToolResult, McpError> {
        let client = match super::bitbucket::BitbucketClient::from_env() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("BitbucketClient::from_env error: {e}");
                return Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        };
        match client.list_branch_restrictions(&workspace, &repo_slug).await {
            Ok(val) => Ok(CallToolResult::success(vec![Content::json(val)?])),
            Err(e) => {
                tracing::error!("list_branch_restrictions error: {e}");
                Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        }
    }

    #[tool(description = "List bitbucket commit statuses for a commit")]
    pub async fn list_commit_statuses(&self, #[tool(param)] workspace: String, #[tool(param)] repo_slug: String, #[tool(param)] commit: String) -> Result<CallToolResult, McpError> {
        let client = match super::bitbucket::BitbucketClient::from_env() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("BitbucketClient::from_env error: {e}");
                return Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        };
        match client.list_commit_statuses(&workspace, &repo_slug, &commit).await {
            Ok(val) => Ok(CallToolResult::success(vec![Content::json(val)?])),
            Err(e) => {
                tracing::error!("list_commit_statuses error: {e}");
                Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        }
    }

    #[tool(description = "List bitbucket users in a workspace")]
    pub async fn list_users(&self, #[tool(param)] workspace: String) -> Result<CallToolResult, McpError> {
        let client = match super::bitbucket::BitbucketClient::from_env() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("BitbucketClient::from_env error: {e}");
                return Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        };
        match client.list_users(&workspace).await {
            Ok(val) => Ok(CallToolResult::success(vec![Content::json(val)?])),
            Err(e) => {
                tracing::error!("list_users error: {e}");
                Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        }
    }

    #[tool(description = "Create a bitbucket repository in a workspace")]
    pub async fn create_repository(&self, #[tool(param)] workspace: String, #[tool(param)] repo_slug: String, #[tool(param)] body: serde_json::Value) -> Result<CallToolResult, McpError> {
        let client = match super::bitbucket::BitbucketClient::from_env() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("BitbucketClient::from_env error: {e}");
                return Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        };
        match client.create_repository(&workspace, &repo_slug, body).await {
            Ok(val) => Ok(CallToolResult::success(vec![Content::json(val)?])),
            Err(e) => {
                tracing::error!("create_repository error: {e}");
                Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        }
    }

    #[tool(description = "Update a bitbucket repository in a workspace")]
    pub async fn update_repository(&self, #[tool(param)] workspace: String, #[tool(param)] repo_slug: String, #[tool(param)] body: serde_json::Value) -> Result<CallToolResult, McpError> {
        let client = match super::bitbucket::BitbucketClient::from_env() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("BitbucketClient::from_env error: {e}");
                return Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        };
        match client.update_repository(&workspace, &repo_slug, body).await {
            Ok(val) => Ok(CallToolResult::success(vec![Content::json(val)?])),
            Err(e) => {
                tracing::error!("update_repository error: {e}");
                Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        }
    }

    #[tool(description = "Delete a bitbucket repository in a workspace")]
    pub async fn delete_repository(&self, #[tool(param)] workspace: String, #[tool(param)] repo_slug: String) -> Result<CallToolResult, McpError> {
        let client = match super::bitbucket::BitbucketClient::from_env() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("BitbucketClient::from_env error: {e}");
                return Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        };
        match client.delete_repository(&workspace, &repo_slug).await {
            Ok(val) => Ok(CallToolResult::success(vec![Content::json(val)?])),
            Err(e) => {
                tracing::error!("delete_repository error: {e}");
                Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        }
    }

    #[tool(description = "Create a bitbucket branch in a repository")]
    pub async fn create_branch(&self, #[tool(param)] workspace: String, #[tool(param)] repo_slug: String, #[tool(param)] body: serde_json::Value) -> Result<CallToolResult, McpError> {
        let client = match super::bitbucket::BitbucketClient::from_env() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("BitbucketClient::from_env error: {e}");
                return Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        };
        match client.create_branch(&workspace, &repo_slug, body).await {
            Ok(val) => Ok(CallToolResult::success(vec![Content::json(val)?])),
            Err(e) => {
                tracing::error!("create_branch error: {e}");
                Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        }
    }

    #[tool(description = "Delete a bitbucket branch in a repository")]
    pub async fn delete_branch(&self, #[tool(param)] workspace: String, #[tool(param)] repo_slug: String, #[tool(param)] branch: String) -> Result<CallToolResult, McpError> {
        let client = match super::bitbucket::BitbucketClient::from_env() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("BitbucketClient::from_env error: {e}");
                return Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        };
        match client.delete_branch(&workspace, &repo_slug, &branch).await {
            Ok(val) => Ok(CallToolResult::success(vec![Content::json(val)?])),
            Err(e) => {
                tracing::error!("delete_branch error: {e}");
                Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        }
    }

    #[tool(description = "Get bitbucket branching model")]
    pub async fn get_branching_model(&self, #[tool(param)] workspace: String, #[tool(param)] repo_slug: String) -> Result<CallToolResult, McpError> {
        let client = match super::bitbucket::BitbucketClient::from_env() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("BitbucketClient::from_env error: {e}");
                return Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        };
        match client.get_branching_model(&workspace, &repo_slug).await {
            Ok(val) => Ok(CallToolResult::success(vec![Content::json(val)?])),
            Err(e) => {
                tracing::error!("get_branching_model error: {e}");
                Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        }
    }

    #[tool(description = "Update bitbucket branching model")]
    pub async fn update_branching_model(&self, #[tool(param)] workspace: String, #[tool(param)] repo_slug: String, #[tool(param)] body: serde_json::Value) -> Result<CallToolResult, McpError> {
        let client = match super::bitbucket::BitbucketClient::from_env() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("BitbucketClient::from_env error: {e}");
                return Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        };
        match client.update_branching_model(&workspace, &repo_slug, body).await {
            Ok(val) => Ok(CallToolResult::success(vec![Content::json(val)?])),
            Err(e) => {
                tracing::error!("update_branching_model error: {e}");
                Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        }
    }

    #[tool(description = "Create a bitbucket commit status")]
    pub async fn create_commit_status(&self, #[tool(param)] workspace: String, #[tool(param)] repo_slug: String, #[tool(param)] commit: String, #[tool(param)] body: serde_json::Value) -> Result<CallToolResult, McpError> {
        let client = match super::bitbucket::BitbucketClient::from_env() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("BitbucketClient::from_env error: {e}");
                return Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        };
        match client.create_commit_status(&workspace, &repo_slug, &commit, body).await {
            Ok(val) => Ok(CallToolResult::success(vec![Content::json(val)?])),
            Err(e) => {
                tracing::error!("create_commit_status error: {e}");
                Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        }
    }

    #[tool(description = "Get bitbucket commit details")]
    pub async fn get_commit(&self, #[tool(param)] workspace: String, #[tool(param)] repo_slug: String, #[tool(param)] commit: String) -> Result<CallToolResult, McpError> {
        let client = match super::bitbucket::BitbucketClient::from_env() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("BitbucketClient::from_env error: {e}");
                return Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        };
        match client.get_commit(&workspace, &repo_slug, &commit).await {
            Ok(val) => Ok(CallToolResult::success(vec![Content::json(val)?])),
            Err(e) => {
                tracing::error!("get_commit error: {e}");
                Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        }
    }

    #[tool(description = "Create a bitbucket deployment")]
    pub async fn create_deployment(&self, #[tool(param)] workspace: String, #[tool(param)] repo_slug: String, #[tool(param)] body: serde_json::Value) -> Result<CallToolResult, McpError> {
        let client = match super::bitbucket::BitbucketClient::from_env() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("BitbucketClient::from_env error: {e}");
                return Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        };
        match client.create_deployment(&workspace, &repo_slug, body).await {
            Ok(val) => Ok(CallToolResult::success(vec![Content::json(val)?])),
            Err(e) => {
                tracing::error!("create_deployment error: {e}");
                Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        }
    }

    #[tool(description = "Create a bitbucket issue")]
    pub async fn create_issue(&self, #[tool(param)] workspace: String, #[tool(param)] repo_slug: String, #[tool(param)] body: serde_json::Value) -> Result<CallToolResult, McpError> {
        let client = match super::bitbucket::BitbucketClient::from_env() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("BitbucketClient::from_env error: {e}");
                return Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        };
        match client.create_issue(&workspace, &repo_slug, body).await {
            Ok(val) => Ok(CallToolResult::success(vec![Content::json(val)?])),
            Err(e) => {
                tracing::error!("create_issue error: {e}");
                Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        }
    }

    #[tool(description = "Update a bitbucket issue")]
    pub async fn update_issue(&self, #[tool(param)] workspace: String, #[tool(param)] repo_slug: String, #[tool(param)] issue_id: String, #[tool(param)] body: serde_json::Value) -> Result<CallToolResult, McpError> {
        let client = match super::bitbucket::BitbucketClient::from_env() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("BitbucketClient::from_env error: {e}");
                return Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        };
        match client.update_issue(&workspace, &repo_slug, &issue_id, body).await {
            Ok(val) => Ok(CallToolResult::success(vec![Content::json(val)?])),
            Err(e) => {
                tracing::error!("update_issue error: {e}");
                Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        }
    }

    #[tool(description = "Delete a bitbucket issue")]
    pub async fn delete_issue(&self, #[tool(param)] workspace: String, #[tool(param)] repo_slug: String, #[tool(param)] issue_id: String) -> Result<CallToolResult, McpError> {
        let client = match super::bitbucket::BitbucketClient::from_env() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("BitbucketClient::from_env error: {e}");
                return Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        };
        match client.delete_issue(&workspace, &repo_slug, &issue_id).await {
            Ok(val) => Ok(CallToolResult::success(vec![Content::json(val)?])),
            Err(e) => {
                tracing::error!("delete_issue error: {e}");
                Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        }
    }

    #[tool(description = "Trigger a bitbucket pipeline")]
    pub async fn trigger_pipeline(&self, #[tool(param)] workspace: String, #[tool(param)] repo_slug: String, #[tool(param)] body: serde_json::Value) -> Result<CallToolResult, McpError> {
        let client = match super::bitbucket::BitbucketClient::from_env() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("BitbucketClient::from_env error: {e}");
                return Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        };
        match client.trigger_pipeline(&workspace, &repo_slug, body).await {
            Ok(val) => Ok(CallToolResult::success(vec![Content::json(val)?])),
            Err(e) => {
                tracing::error!("trigger_pipeline error: {e}");
                Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        }
    }

    #[tool(description = "Create a bitbucket project in a workspace")]
    pub async fn create_project(&self, #[tool(param)] workspace: String, #[tool(param)] body: serde_json::Value) -> Result<CallToolResult, McpError> {
        let client = match super::bitbucket::BitbucketClient::from_env() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("BitbucketClient::from_env error: {e}");
                return Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        };
        match client.create_project(&workspace, body).await {
            Ok(val) => Ok(CallToolResult::success(vec![Content::json(val)?])),
            Err(e) => {
                tracing::error!("create_project error: {e}");
                Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        }
    }

    #[tool(description = "Update a bitbucket project in a workspace")]
    pub async fn update_project(&self, #[tool(param)] workspace: String, #[tool(param)] project_key: String, #[tool(param)] body: serde_json::Value) -> Result<CallToolResult, McpError> {
        let client = match super::bitbucket::BitbucketClient::from_env() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("BitbucketClient::from_env error: {e}");
                return Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        };
        match client.update_project(&workspace, &project_key, body).await {
            Ok(val) => Ok(CallToolResult::success(vec![Content::json(val)?])),
            Err(e) => {
                tracing::error!("update_project error: {e}");
                Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        }
    }

    #[tool(description = "Delete a bitbucket project in a workspace")]
    pub async fn delete_project(&self, #[tool(param)] workspace: String, #[tool(param)] project_key: String) -> Result<CallToolResult, McpError> {
        let client = match super::bitbucket::BitbucketClient::from_env() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("BitbucketClient::from_env error: {e}");
                return Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        };
        match client.delete_project(&workspace, &project_key).await {
            Ok(val) => Ok(CallToolResult::success(vec![Content::json(val)?])),
            Err(e) => {
                tracing::error!("delete_project error: {e}");
                Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        }
    }

    #[tool(description = "Create a bitbucket snippet in a workspace")]
    pub async fn create_snippet(&self, #[tool(param)] workspace: String, #[tool(param)] body: serde_json::Value) -> Result<CallToolResult, McpError> {
        let client = match super::bitbucket::BitbucketClient::from_env() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("BitbucketClient::from_env error: {e}");
                return Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        };
        match client.create_snippet(&workspace, body).await {
            Ok(val) => Ok(CallToolResult::success(vec![Content::json(val)?])),
            Err(e) => {
                tracing::error!("create_snippet error: {e}");
                Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        }
    }

    #[tool(description = "Delete a bitbucket snippet in a workspace")]
    pub async fn delete_snippet(&self, #[tool(param)] workspace: String, #[tool(param)] snippet_id: String) -> Result<CallToolResult, McpError> {
        let client = match super::bitbucket::BitbucketClient::from_env() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("BitbucketClient::from_env error: {e}");
                return Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        };
        match client.delete_snippet(&workspace, &snippet_id).await {
            Ok(val) => Ok(CallToolResult::success(vec![Content::json(val)?])),
            Err(e) => {
                tracing::error!("delete_snippet error: {e}");
                Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        }
    }

    #[tool(description = "Get bitbucket file source from a repository")]
    pub async fn get_file_source(&self, #[tool(param)] workspace: String, #[tool(param)] repo_slug: String, #[tool(param)] commit: String, #[tool(param)] path: String) -> Result<CallToolResult, McpError> {
        let client = match super::bitbucket::BitbucketClient::from_env() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("BitbucketClient::from_env error: {e}");
                return Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        };
        match client.get_file_source(&workspace, &repo_slug, &commit, &path).await {
            Ok(val) => Ok(CallToolResult::success(vec![Content::json(val)?])),
            Err(e) => {
                tracing::error!("get_file_source error: {e}");
                Ok(CallToolResult::error(vec![Content::text(e.to_string())]))
            },
        }
    }
}

#[tool(tool_box)]
impl ServerHandler for BitbucketTool {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("Bitbucket MCP tool: interact with Bitbucket Cloud REST API. Set BITBUCKET_USERNAME and BITBUCKET_APP_PASSWORD env vars.".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}
