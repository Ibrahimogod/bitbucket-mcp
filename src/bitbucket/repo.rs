use actix_web::{web, HttpResponse, Responder, Error};
use serde::{Deserialize, Serialize};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};

#[derive(Deserialize)]
pub struct BitbucketRepoRequest {
    pub workspace: String,
    pub repo_slug: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BitbucketRepo {
    pub uuid: String,
    pub name: String,
    pub full_name: String,
    pub is_private: bool,
}

#[derive(Serialize)]
pub struct BitbucketRepoResponse {
    pub repo: Option<BitbucketRepo>,
    pub error: Option<String>,
}

pub struct BitbucketAuth {
    pub token: String,
}

pub async fn bitbucket_repo_handler(
    req: web::Json<BitbucketRepoRequest>,
    data: web::Data<BitbucketAuth>,
) -> Result<impl Responder, Error> {
    let client = reqwest::Client::new();
    let url = format!(
        "https://api.bitbucket.org/2.0/repositories/{}/{}",
        req.workspace, req.repo_slug
    );
    let res = client
        .get(&url)
        .header(AUTHORIZATION, format!("Bearer {}", data.token))
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await;
    match res {
        Ok(response) => {
            if response.status().is_success() {
                let repo: BitbucketRepo = response.json().await.unwrap_or_else(|_| BitbucketRepo {
                    uuid: "".to_string(),
                    name: "".to_string(),
                    full_name: "".to_string(),
                    is_private: false,
                });
                Ok(HttpResponse::Ok().json(BitbucketRepoResponse {
                    repo: Some(repo),
                    error: None,
                }))
            } else {
                Ok(HttpResponse::BadRequest().json(BitbucketRepoResponse {
                    repo: None,
                    error: Some(format!("Bitbucket API error: {}", response.status())),
                }))
            }
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(BitbucketRepoResponse {
            repo: None,
            error: Some(format!("Request error: {}", e)),
        })),
    }
}

// List repos
#[derive(Deserialize)]
pub struct BitbucketListReposRequest {
    pub workspace: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BitbucketRepoSummary {
    pub uuid: String,
    pub name: String,
    pub full_name: String,
    pub is_private: bool,
}

#[derive(Serialize)]
pub struct BitbucketListReposResponse {
    pub repos: Vec<BitbucketRepoSummary>,
    pub error: Option<String>,
}

pub async fn bitbucket_list_repos_handler(
    req: web::Json<BitbucketListReposRequest>,
    data: web::Data<BitbucketAuth>,
) -> Result<impl Responder, Error> {
    let client = reqwest::Client::new();
    let url = format!(
        "https://api.bitbucket.org/2.0/repositories/{}",
        req.workspace
    );
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
                let mut repos = Vec::new();
                if let Some(values) = json.get("values").and_then(|v| v.as_array()) {
                    for repo in values {
                        if let (Some(uuid), Some(name), Some(full_name), Some(is_private)) = (
                            repo.get("uuid").and_then(|v| v.as_str()),
                            repo.get("name").and_then(|v| v.as_str()),
                            repo.get("full_name").and_then(|v| v.as_str()),
                            repo.get("is_private").and_then(|v| v.as_bool()),
                        ) {
                            repos.push(BitbucketRepoSummary {
                                uuid: uuid.to_string(),
                                name: name.to_string(),
                                full_name: full_name.to_string(),
                                is_private,
                            });
                        }
                    }
                }
                Ok(HttpResponse::Ok().json(BitbucketListReposResponse {
                    repos,
                    error: None,
                }))
            } else {
                Ok(HttpResponse::BadRequest().json(BitbucketListReposResponse {
                    repos: vec![],
                    error: Some(format!("Bitbucket API error: {}", response.status())),
                }))
            }
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(BitbucketListReposResponse {
            repos: vec![],
            error: Some(format!("Request error: {}", e)),
        })),
    }
}
