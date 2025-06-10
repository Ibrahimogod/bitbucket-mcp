use actix_web::{web, HttpResponse, Responder, Error};
use serde::{Deserialize, Serialize};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use crate::bitbucket::repo::BitbucketAuth;

#[derive(Deserialize)]
pub struct BitbucketPullRequestRequest {
    pub workspace: String,
    pub repo_slug: String,
    pub pr_id: Option<u32>, // If present, get a specific PR; else, list PRs
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BitbucketPullRequest {
    pub id: u32,
    pub title: String,
    pub state: String,
    pub author: Option<String>,
    // Add more fields as needed
}

#[derive(Serialize)]
pub struct BitbucketPullRequestResponse {
    pub pull_requests: Vec<BitbucketPullRequest>,
    pub error: Option<String>,
}

pub async fn bitbucket_pull_request_handler(
    req: web::Json<BitbucketPullRequestRequest>,
    data: web::Data<BitbucketAuth>,
) -> Result<impl Responder, Error> {
    let client = reqwest::Client::new();
    let url = if let Some(pr_id) = req.pr_id {
        format!(
            "https://api.bitbucket.org/2.0/repositories/{}/{}/pullrequests/{}",
            req.workspace, req.repo_slug, pr_id
        )
    } else {
        format!(
            "https://api.bitbucket.org/2.0/repositories/{}/{}/pullrequests",
            req.workspace, req.repo_slug
        )
    };
    let res = client
        .get(&url)
        .header(AUTHORIZATION, format!("Bearer {}", data.token))
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await;
    match res {
        Ok(response) => {
            if response.status().is_success() {
                let json: serde_json::Value = response.json().await.unwrap_or_default();
                let mut prs = Vec::new();
                if let Some(values) = json.get("values").and_then(|v| v.as_array()) {
                    for pr in values {
                        if let (Some(id), Some(title), Some(state)) = (
                            pr.get("id").and_then(|v| v.as_u64()),
                            pr.get("title").and_then(|v| v.as_str()),
                            pr.get("state").and_then(|v| v.as_str()),
                        ) {
                            prs.push(BitbucketPullRequest {
                                id: id as u32,
                                title: title.to_string(),
                                state: state.to_string(),
                                author: pr.get("author").and_then(|a| a.get("display_name")).and_then(|v| v.as_str()).map(|s| s.to_string()),
                            });
                        }
                    }
                } else if let (Some(id), Some(title), Some(state)) = (
                    json.get("id").and_then(|v| v.as_u64()),
                    json.get("title").and_then(|v| v.as_str()),
                    json.get("state").and_then(|v| v.as_str()),
                ) {
                    prs.push(BitbucketPullRequest {
                        id: id as u32,
                        title: title.to_string(),
                        state: state.to_string(),
                        author: json.get("author").and_then(|a| a.get("display_name")).and_then(|v| v.as_str()).map(|s| s.to_string()),
                    });
                }
                Ok(HttpResponse::Ok().json(BitbucketPullRequestResponse {
                    pull_requests: prs,
                    error: None,
                }))
            } else {
                Ok(HttpResponse::BadRequest().json(BitbucketPullRequestResponse {
                    pull_requests: vec![],
                    error: Some(format!("Bitbucket API error: {}", response.status())),
                }))
            }
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(BitbucketPullRequestResponse {
            pull_requests: vec![],
            error: Some(format!("Request error: {}", e)),
        })),
    }
}
