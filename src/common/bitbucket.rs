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
}

impl BitbucketClient {
    pub fn from_env() -> Result<Self> {
        let api_username = env::var("BITBUCKET_API_USERNAME")
            .map_err(|_| anyhow!("BITBUCKET_API_USERNAME env var not set. Please set it to your Atlassian email."))?;
        let app_password = env::var("BITBUCKET_APP_PASSWORD")
            .map_err(|_| anyhow!("BITBUCKET_APP_PASSWORD env var not set. Please set it to your Bitbucket API token."))?;
        Ok(Self {
            api_username,
            app_password,
            client: Client::new(),
        })
    }

    fn apply_auth<'a>(&self, req: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        req.basic_auth(&self.api_username, Some(&self.app_password))
    }

    pub async fn get_user(&self) -> Result<serde_json::Value> {
        let url = "https://api.bitbucket.org/2.0/user";
        let req = self.client.get(url);
        let resp = self.apply_auth(req).send().await?;
        Ok(resp.json().await?)
    }

    pub async fn list_workspaces(&self) -> Result<serde_json::Value> {
        let url = "https://api.bitbucket.org/2.0/workspaces";
        let req = self.client.get(url);
        let resp = self.apply_auth(req).send().await?;
        Ok(resp.json().await?)
    }

    pub async fn list_repositories(&self, workspace: &str) -> Result<serde_json::Value> {
        let url = format!("https://api.bitbucket.org/2.0/repositories/{}", workspace);
        let req = self.client.get(&url);
        let resp = self.apply_auth(req).send().await?;
        Ok(resp.json().await?)
    }

    pub async fn list_pullrequests(&self, workspace: &str, repo_slug: &str) -> Result<serde_json::Value> {
        let url = format!("https://api.bitbucket.org/2.0/repositories/{}/{}/pullrequests", workspace, repo_slug);
        tracing::info!("Requesting pull requests from URL: {}", url);
        let req = self.client.get(&url);
        let resp = self.apply_auth(req).send().await?;
        let status = resp.status();
        let text = resp.text().await?;
        tracing::info!("Bitbucket response status: {}", status);
        tracing::info!("Bitbucket response body: {}", text);
        let json: serde_json::Value = serde_json::from_str(&text)?;
        Ok(json)
    }

    pub async fn list_issues(&self, workspace: &str, repo_slug: &str) -> Result<serde_json::Value> {
        let url = format!("https://api.bitbucket.org/2.0/repositories/{}/{}/issues", workspace, repo_slug);
        let req = self.client.get(&url);
        let resp = self.apply_auth(req).send().await?;
        Ok(resp.json().await?)
    }

    pub async fn get_workspace(&self, workspace: &str) -> Result<serde_json::Value> {
        let url = format!("https://api.bitbucket.org/2.0/workspaces/{}", workspace);
        let req = self.client.get(&url);
        let resp = self.apply_auth(req).send().await?;
        Ok(resp.json().await?)
    }
    pub async fn get_repository(&self, workspace: &str, repo_slug: &str) -> Result<serde_json::Value> {
        let url = format!("https://api.bitbucket.org/2.0/repositories/{}/{}", workspace, repo_slug);
        let req = self.client.get(&url);
        let resp = self.apply_auth(req).send().await?;
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
        let url = format!("https://api.bitbucket.org/2.0/repositories/{}/{}", workspace, repo_slug);
        let req = self.client.delete(&url);
        let resp = self.apply_auth(req).send().await?;
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
        let url = format!("https://api.bitbucket.org/2.0/repositories/{}/{}/refs/branches/{}", workspace, repo_slug, branch);
        let req = self.client.delete(&url);
        let resp = self.apply_auth(req).send().await?;
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
        let url = format!("https://api.bitbucket.org/2.0/repositories/{}/{}/issues/{}", workspace, repo_slug, issue_id);
        let req = self.client.delete(&url);
        let resp = self.apply_auth(req).send().await?;
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
        let url = format!("https://api.bitbucket.org/2.0/workspaces/{}/projects/{}", workspace, project_key);
        let req = self.client.delete(&url);
        let resp = self.apply_auth(req).send().await?;
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
        let url = format!("https://api.bitbucket.org/2.0/snippets/{}/{}", workspace, snippet_id);
        let req = self.client.delete(&url);
        let resp = self.apply_auth(req).send().await?;
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
    #[tool(description = "Get Bitbucket user info")] 
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

    #[tool(description = "List Bitbucket workspaces")]
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

    #[tool(description = "List repositories in a workspace")]
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

    #[tool(description = "List pull requests for a repository")]
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

    #[tool(description = "List issues for a repository")]
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

    #[tool(description = "Get workspace details")]
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

    #[tool(description = "Get repository details")]
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

    #[tool(description = "List branches for a repository")]
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

    #[tool(description = "List tags for a repository")]
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

    #[tool(description = "List commits for a repository")]
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

    #[tool(description = "List pipelines for a repository")]
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

    #[tool(description = "List deployments for a repository")]
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

    #[tool(description = "List downloads for a repository")]
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

    #[tool(description = "List webhooks for a repository")]
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

    #[tool(description = "List snippets for a workspace")]
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

    #[tool(description = "List projects for a workspace")]
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

    #[tool(description = "List branch restrictions for a repository")]
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

    #[tool(description = "List commit statuses for a commit")]
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

    #[tool(description = "List users in a workspace")]
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

    #[tool(description = "Create a repository in a workspace")]
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

    #[tool(description = "Update a repository in a workspace")]
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

    #[tool(description = "Delete a repository in a workspace")]
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

    #[tool(description = "Create a branch in a repository")]
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

    #[tool(description = "Delete a branch in a repository")]
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

    #[tool(description = "Get branching model")]
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

    #[tool(description = "Update branching model")]
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

    #[tool(description = "Create a commit status")]
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

    #[tool(description = "Get commit details")]
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

    #[tool(description = "Create a deployment")]
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

    #[tool(description = "Create an issue")]
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

    #[tool(description = "Update an issue")]
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

    #[tool(description = "Delete an issue")]
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

    #[tool(description = "Trigger a pipeline")]
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

    #[tool(description = "Create a project in a workspace")]
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

    #[tool(description = "Update a project in a workspace")]
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

    #[tool(description = "Delete a project in a workspace")]
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

    #[tool(description = "Create a snippet in a workspace")]
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

    #[tool(description = "Delete a snippet in a workspace")]
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

    #[tool(description = "Get file source from a repository")]
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
