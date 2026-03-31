// Integration tests using real Bitbucket API
// These tests use real credentials but only perform READ operations
// to avoid modifying the actual Bitbucket workspace

use bitbucket_mcp::common::bitbucket::BitbucketClient;
use std::env;

fn get_integration_client() -> Option<BitbucketClient> {
    // Use provided credentials or skip tests if not available
    let api_username = env::var("BITBUCKET_API_USERNAME").ok()?;
    let api_token = env::var("BITBUCKET_API_TOKEN").ok()?;
    
    Some(BitbucketClient {
        api_username,
        api_token,
        client: reqwest::Client::new(),
        base_url: "https://api.bitbucket.org/2.0".to_string(),
    })
}

fn get_test_workspace() -> Option<String> {
    // Get workspace from environment variable, or skip test if not set
    env::var("BITBUCKET_TEST_WORKSPACE").ok()
}

#[tokio::test]
#[ignore] // Run with: cargo test --ignored
async fn integration_test_get_user() {
    let client = match get_integration_client() {
        Some(c) => c,
        None => {
            println!("Skipping integration test - credentials not set");
            return;
        }
    };
    
    let result = client.get_user().await;
    assert!(result.is_ok(), "Failed to get user: {:?}", result.err());
    
    let user = result.unwrap();
    println!("User: {}", serde_json::to_string_pretty(&user).unwrap());
    assert!(user.get("username").is_some());
}

#[tokio::test]
#[ignore]
async fn integration_test_list_workspaces() {
    let client = match get_integration_client() {
        Some(c) => c,
        None => return,
    };
    
    let result = client.list_workspaces().await;
    assert!(result.is_ok(), "Failed to list workspaces: {:?}", result.err());
    
    let workspaces = result.unwrap();
    println!("Workspaces: {}", serde_json::to_string_pretty(&workspaces).unwrap());
    assert!(workspaces.get("values").is_some());
    
    let values = workspaces["values"].as_array().unwrap();
    println!("Found {} workspaces", values.len());
}

#[tokio::test]
#[ignore]
async fn integration_test_get_workspace() {
    let client = match get_integration_client() {
        Some(c) => c,
        None => return,
    };
    
    let workspace = match get_test_workspace() {
        Some(w) => w,
        None => return,
    };
    
    let result = client.get_workspace(&workspace).await;
    assert!(result.is_ok(), "Failed to get workspace: {:?}", result.err());
    
    let workspace_data = result.unwrap();
    println!("Workspace: {}", serde_json::to_string_pretty(&workspace_data).unwrap());
    assert_eq!(workspace_data["slug"], workspace);
}

#[tokio::test]
#[ignore]
async fn integration_test_list_repositories() {
    let client = match get_integration_client() {
        Some(c) => c,
        None => return,
    };
    
    let workspace = match get_test_workspace() {
        Some(w) => w,
        None => return,
    };
    
    let result = client.list_repositories(&workspace).await;
    assert!(result.is_ok(), "Failed to list repositories: {:?}", result.err());
    
    let repos = result.unwrap();
    println!("Repositories: {}", serde_json::to_string_pretty(&repos).unwrap());
    assert!(repos.get("values").is_some());
    
    let values = repos["values"].as_array().unwrap();
    println!("Found {} repositories (after pagination)", values.len());
    
    // Verify pagination worked by checking we got all repos
    assert!(repos.get("size").is_some());
}

#[tokio::test]
#[ignore]
async fn integration_test_get_repository() {
    let client = match get_integration_client() {
        Some(c) => c,
        None => return,
    };
    
    let workspace = match get_test_workspace() {
        Some(w) => w,
        None => return,
    };
    
    // First, list repositories to get an actual repo
    let repos_result = client.list_repositories(&workspace).await;
    if let Ok(repos) = repos_result {
        if let Some(values) = repos["values"].as_array() {
            if !values.is_empty() {
                let first_repo_slug = values[0]["slug"].as_str().unwrap();
                println!("Testing with repository: {}", first_repo_slug);
                
                let result = client.get_repository(&workspace, first_repo_slug).await;
                assert!(result.is_ok(), "Failed to get repository: {:?}", result.err());
                
                let repo = result.unwrap();
                println!("Repository: {}", serde_json::to_string_pretty(&repo).unwrap());
                assert_eq!(repo["slug"], first_repo_slug);
            } else {
                println!("No repositories found in workspace");
            }
        }
    }
}

#[tokio::test]
#[ignore]
async fn integration_test_list_pullrequests() {
    let client = match get_integration_client() {
        Some(c) => c,
        None => return,
    };
    
    let workspace = match get_test_workspace() {
        Some(w) => w,
        None => return,
    };
    
    // First, get a repository
    let repos_result = client.list_repositories(&workspace).await;
    if let Ok(repos) = repos_result {
        if let Some(values) = repos["values"].as_array() {
            if !values.is_empty() {
                let first_repo_slug = values[0]["slug"].as_str().unwrap();
                println!("Listing PRs for repository: {}", first_repo_slug);
                
                let result = client.list_pullrequests(&workspace, first_repo_slug).await;
                assert!(result.is_ok(), "Failed to list pull requests: {:?}", result.err());
                
                let prs = result.unwrap();
                println!("Pull Requests: {}", serde_json::to_string_pretty(&prs).unwrap());
                
                let values = prs["values"].as_array().unwrap();
                println!("Found {} pull requests (after pagination)", values.len());
            }
        }
    }
}

#[tokio::test]
#[ignore]
async fn integration_test_list_pullrequest_comments() {
    let client = match get_integration_client() {
        Some(c) => c,
        None => return,
    };
    
    let workspace = match get_test_workspace() {
        Some(w) => w,
        None => return,
    };
    
    // First, get a repository and a PR
    let repos_result = client.list_repositories(&workspace).await;
    if let Ok(repos) = repos_result {
        if let Some(repo_values) = repos["values"].as_array() {
            if !repo_values.is_empty() {
                let first_repo_slug = repo_values[0]["slug"].as_str().unwrap();
                
                let prs_result = client.list_pullrequests(&workspace, first_repo_slug).await;
                if let Ok(prs) = prs_result {
                    if let Some(pr_values) = prs["values"].as_array() {
                        if !pr_values.is_empty() {
                            let first_pr_id = pr_values[0]["id"].as_i64().unwrap();
                            println!("Listing comments for PR: {}", first_pr_id);
                            
                            let result = client.list_pullrequest_comments(
                                &workspace,
                                first_repo_slug,
                                &first_pr_id.to_string()
                            ).await;
                            
                            assert!(result.is_ok(), "Failed to list PR comments: {:?}", result.err());
                            
                            let comments = result.unwrap();
                            println!("PR Comments: {}", serde_json::to_string_pretty(&comments).unwrap());
                            
                            let values = comments["values"].as_array().unwrap();
                            println!("Found {} comments (after pagination)", values.len());
                            
                            // Verify pagination was applied
                            assert!(comments.get("size").is_some());
                        } else {
                            println!("No PRs found");
                        }
                    }
                }
            }
        }
    }
}

#[tokio::test]
#[ignore]
async fn integration_test_list_branches() {
    let client = match get_integration_client() {
        Some(c) => c,
        None => return,
    };
    
    let workspace = match get_test_workspace() {
        Some(w) => w,
        None => return,
    };
    
    let repos_result = client.list_repositories(&workspace).await;
    if let Ok(repos) = repos_result {
        if let Some(values) = repos["values"].as_array() {
            if !values.is_empty() {
                let first_repo_slug = values[0]["slug"].as_str().unwrap();
                println!("Listing branches for repository: {}", first_repo_slug);
                
                let result = client.list_branches(&workspace, first_repo_slug).await;
                assert!(result.is_ok(), "Failed to list branches: {:?}", result.err());
                
                let branches = result.unwrap();
                println!("Branches: {}", serde_json::to_string_pretty(&branches).unwrap());
                
                let values = branches["values"].as_array().unwrap();
                println!("Found {} branches (after pagination)", values.len());
            }
        }
    }
}

#[tokio::test]
#[ignore]
async fn integration_test_list_tags() {
    let client = match get_integration_client() {
        Some(c) => c,
        None => return,
    };
    
    let workspace = match get_test_workspace() {
        Some(w) => w,
        None => return,
    };
    
    let repos_result = client.list_repositories(&workspace).await;
    if let Ok(repos) = repos_result {
        if let Some(values) = repos["values"].as_array() {
            if !values.is_empty() {
                let first_repo_slug = values[0]["slug"].as_str().unwrap();
                println!("Listing tags for repository: {}", first_repo_slug);
                
                let result = client.list_tags(&workspace, first_repo_slug).await;
                assert!(result.is_ok(), "Failed to list tags: {:?}", result.err());
                
                let tags = result.unwrap();
                println!("Tags: {}", serde_json::to_string_pretty(&tags).unwrap());
                
                let values = tags["values"].as_array().unwrap();
                println!("Found {} tags (after pagination)", values.len());
            }
        }
    }
}

#[tokio::test]
#[ignore]
async fn integration_test_list_commits() {
    let client = match get_integration_client() {
        Some(c) => c,
        None => return,
    };
    
    let workspace = match get_test_workspace() {
        Some(w) => w,
        None => return,
    };
    
    let repos_result = client.list_repositories(&workspace).await;
    if let Ok(repos) = repos_result {
        if let Some(values) = repos["values"].as_array() {
            if !values.is_empty() {
                let first_repo_slug = values[0]["slug"].as_str().unwrap();
                println!("Listing commits for repository: {}", first_repo_slug);
                
                let result = client.list_commits(&workspace, first_repo_slug).await;
                assert!(result.is_ok(), "Failed to list commits: {:?}", result.err());
                
                let commits = result.unwrap();
                println!("Commits count: {}", commits["values"].as_array().unwrap().len());
                
                let values = commits["values"].as_array().unwrap();
                println!("Found {} commits (after pagination)", values.len());
                
                // Verify we got commits
                if !values.is_empty() {
                    assert!(values[0].get("hash").is_some());
                }
            }
        }
    }
}

#[tokio::test]
#[ignore]
async fn integration_test_list_issues() {
    let client = match get_integration_client() {
        Some(c) => c,
        None => return,
    };
    
    let workspace = match get_test_workspace() {
        Some(w) => w,
        None => return,
    };
    
    let repos_result = client.list_repositories(&workspace).await;
    if let Ok(repos) = repos_result {
        if let Some(values) = repos["values"].as_array() {
            if !values.is_empty() {
                let first_repo_slug = values[0]["slug"].as_str().unwrap();
                println!("Listing issues for repository: {}", first_repo_slug);
                
                let result = client.list_issues(&workspace, first_repo_slug).await;
                assert!(result.is_ok(), "Failed to list issues: {:?}", result.err());
                
                let issues = result.unwrap();
                println!("Issues: {}", serde_json::to_string_pretty(&issues).unwrap());
                
                let values = issues["values"].as_array().unwrap();
                println!("Found {} issues (after pagination)", values.len());
            }
        }
    }
}

#[tokio::test]
#[ignore]
async fn integration_test_pagination_verification() {
    let client = match get_integration_client() {
        Some(c) => c,
        None => return,
    };
    
    let workspace = match get_test_workspace() {
        Some(w) => w,
        None => return,
    };
    
    println!("\n=== Testing Pagination Implementation ===\n");
    
    // Test that list methods return consistent format
    let workspaces = client.list_workspaces().await.unwrap();
    assert!(workspaces.get("values").is_some(), "Workspaces should have values array");
    assert!(workspaces.get("size").is_some(), "Workspaces should have size field");
    println!("✓ Workspaces pagination format correct");
    
    let repos = client.list_repositories(&workspace).await.unwrap();
    assert!(repos.get("values").is_some(), "Repositories should have values array");
    assert!(repos.get("size").is_some(), "Repositories should have size field");
    println!("✓ Repositories pagination format correct");
    
    if let Some(repo_values) = repos["values"].as_array() {
        if !repo_values.is_empty() {
            let first_repo_slug = repo_values[0]["slug"].as_str().unwrap();
            
            let prs = client.list_pullrequests(&workspace, first_repo_slug).await.unwrap();
            assert!(prs.get("values").is_some(), "PRs should have values array");
            assert!(prs.get("size").is_some(), "PRs should have size field");
            println!("✓ Pull Requests pagination format correct");
            
            let branches = client.list_branches(&workspace, first_repo_slug).await.unwrap();
            assert!(branches.get("values").is_some(), "Branches should have values array");
            assert!(branches.get("size").is_some(), "Branches should have size field");
            println!("✓ Branches pagination format correct");
        }
    }
    
    println!("\n=== All pagination tests passed! ===\n");
}

#[tokio::test]
#[ignore]
async fn integration_test_list_users() {
    let client = match get_integration_client() {
        Some(c) => c,
        None => return,
    };
    
    let workspace = match get_test_workspace() {
        Some(w) => w,
        None => return,
    };
    
    let result = client.list_users(&workspace).await;
    assert!(result.is_ok(), "Failed to list users: {:?}", result.err());
    
    let users = result.unwrap();
    println!("Users: {}", serde_json::to_string_pretty(&users).unwrap());
    
    let values = users["values"].as_array().unwrap();
    println!("Found {} users (after pagination)", values.len());
}

#[tokio::test]
#[ignore]
async fn integration_test_list_projects() {
    let client = match get_integration_client() {
        Some(c) => c,
        None => return,
    };
    
    let workspace = match get_test_workspace() {
        Some(w) => w,
        None => return,
    };
    
    let result = client.list_projects(&workspace).await;
    assert!(result.is_ok(), "Failed to list projects: {:?}", result.err());
    
    let projects = result.unwrap();
    println!("Projects: {}", serde_json::to_string_pretty(&projects).unwrap());
    
    let values = projects["values"].as_array().unwrap();
    println!("Found {} projects (after pagination)", values.len());
}
