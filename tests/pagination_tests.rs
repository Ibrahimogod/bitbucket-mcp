mod common;

use serde_json::json;
use common::make_client;

#[tokio::test]
async fn test_pagination_single_page() {
    let _m = mockito::mock("GET", "/2.0/repositories/ws/repo/pullrequests")
        .with_status(200)
        .with_body(r#"{"values": [{"id": 1}, {"id": 2}], "size": 2}"#)
        .create();
    
    let client = make_client(&mockito::server_url());
    let result = client.list_pullrequests("ws", "repo").await.unwrap();
    
    assert_eq!(result["values"].as_array().unwrap().len(), 2);
    assert_eq!(result["size"], 2);
}

#[tokio::test]
async fn test_pagination_multiple_pages() {
    let server_url = mockito::server_url();
    
    let _m1 = mockito::mock("GET", "/2.0/repositories/ws/repo/pullrequests")
        .with_status(200)
        .with_body(json!({
            "values": [{"id": 1}, {"id": 2}],
            "next": format!("{}/2.0/repositories/ws/repo/pullrequests?page=2", server_url)
        }).to_string())
        .create();
    
    let _m2 = mockito::mock("GET", "/2.0/repositories/ws/repo/pullrequests?page=2")
        .with_status(200)
        .with_body(json!({
            "values": [{"id": 3}, {"id": 4}],
            "next": format!("{}/2.0/repositories/ws/repo/pullrequests?page=3", server_url)
        }).to_string())
        .create();
    
    let _m3 = mockito::mock("GET", "/2.0/repositories/ws/repo/pullrequests?page=3")
        .with_status(200)
        .with_body(r#"{"values": [{"id": 5}]}"#)
        .create();
    
    let client = make_client(&server_url);
    let result = client.list_pullrequests("ws", "repo").await.unwrap();
    
    assert_eq!(result["values"].as_array().unwrap().len(), 5);
    assert_eq!(result["size"], 5);
}

#[tokio::test]
async fn test_pagination_empty_result() {
    let _m = mockito::mock("GET", "/2.0/repositories/ws/repo/pullrequests")
        .with_status(200)
        .with_body(r#"{"values": []}"#)
        .create();
    
    let client = make_client(&mockito::server_url());
    let result = client.list_pullrequests("ws", "repo").await.unwrap();
    
    assert_eq!(result["values"].as_array().unwrap().len(), 0);
    assert_eq!(result["size"], 0);
}

#[tokio::test]
async fn test_pagination_list_comments_multiple_pages() {
    let server_url = mockito::server_url();
    
    let _m1 = mockito::mock("GET", "/2.0/repositories/ws/repo/pullrequests/1/comments")
        .with_status(200)
        .with_body(json!({
            "values": [{"id": 101}, {"id": 102}],
            "next": format!("{}/2.0/repositories/ws/repo/pullrequests/1/comments?page=2", server_url)
        }).to_string())
        .create();
    
    let _m2 = mockito::mock("GET", "/2.0/repositories/ws/repo/pullrequests/1/comments?page=2")
        .with_status(200)
        .with_body(r#"{"values": [{"id": 103}]}"#)
        .create();
    
    let client = make_client(&server_url);
    let result = client.list_pullrequest_comments("ws", "repo", "1").await.unwrap();
    
    assert_eq!(result["values"].as_array().unwrap().len(), 3);
    assert_eq!(result["size"], 3);
}

#[tokio::test]
async fn test_pagination_list_repositories() {
    let server_url = mockito::server_url();
    
    let _m1 = mockito::mock("GET", "/2.0/repositories/ws")
        .with_status(200)
        .with_body(json!({
            "values": [{"slug": "repo1"}, {"slug": "repo2"}],
            "next": format!("{}/2.0/repositories/ws?page=2", server_url)
        }).to_string())
        .create();
    
    let _m2 = mockito::mock("GET", "/2.0/repositories/ws?page=2")
        .with_status(200)
        .with_body(r#"{"values": [{"slug": "repo3"}]}"#)
        .create();
    
    let client = make_client(&server_url);
    let result = client.list_repositories("ws").await.unwrap();
    
    assert_eq!(result["values"].as_array().unwrap().len(), 3);
    assert_eq!(result["size"], 3);
}

#[tokio::test]
async fn test_pagination_list_workspaces() {
    let server_url = mockito::server_url();
    
    let _m1 = mockito::mock("GET", "/2.0/workspaces")
        .with_status(200)
        .with_body(json!({
            "values": [{"slug": "ws1"}],
            "next": format!("{}/2.0/workspaces?page=2", server_url)
        }).to_string())
        .create();
    
    let _m2 = mockito::mock("GET", "/2.0/workspaces?page=2")
        .with_status(200)
        .with_body(r#"{"values": [{"slug": "ws2"}]}"#)
        .create();
    
    let client = make_client(&server_url);
    let result = client.list_workspaces().await.unwrap();
    
    assert_eq!(result["values"].as_array().unwrap().len(), 2);
    assert_eq!(result["size"], 2);
}

#[tokio::test]
async fn test_pagination_list_issues() {
    let server_url = mockito::server_url();
    
    let _m1 = mockito::mock("GET", "/2.0/repositories/ws/repo/issues")
        .with_status(200)
        .with_body(json!({
            "values": [{"id": 1}, {"id": 2}],
            "next": format!("{}/2.0/repositories/ws/repo/issues?page=2", server_url)
        }).to_string())
        .create();
    
    let _m2 = mockito::mock("GET", "/2.0/repositories/ws/repo/issues?page=2")
        .with_status(200)
        .with_body(r#"{"values": [{"id": 3}]}"#)
        .create();
    
    let client = make_client(&server_url);
    let result = client.list_issues("ws", "repo").await.unwrap();
    
    assert_eq!(result["values"].as_array().unwrap().len(), 3);
    assert_eq!(result["size"], 3);
}

#[tokio::test]
async fn test_pagination_list_branches() {
    let _m = mockito::mock("GET", "/2.0/repositories/ws/repo/refs/branches")
        .with_status(200)
        .with_body(r#"{"values": [{"name": "main"}, {"name": "develop"}]}"#)
        .create();
    
    let client = make_client(&mockito::server_url());
    let result = client.list_branches("ws", "repo").await.unwrap();
    
    assert_eq!(result["values"].as_array().unwrap().len(), 2);
    assert_eq!(result["size"], 2);
}

#[tokio::test]
async fn test_pagination_list_tags() {
    let _m = mockito::mock("GET", "/2.0/repositories/ws/repo/refs/tags")
        .with_status(200)
        .with_body(r#"{"values": [{"name": "v1.0"}, {"name": "v2.0"}]}"#)
        .create();
    
    let client = make_client(&mockito::server_url());
    let result = client.list_tags("ws", "repo").await.unwrap();
    
    assert_eq!(result["values"].as_array().unwrap().len(), 2);
    assert_eq!(result["size"], 2);
}

#[tokio::test]
async fn test_pagination_list_commits() {
    let _m = mockito::mock("GET", "/2.0/repositories/ws/repo/commits")
        .with_status(200)
        .with_body(r#"{"values": [{"hash": "abc123"}, {"hash": "def456"}]}"#)
        .create();
    
    let client = make_client(&mockito::server_url());
    let result = client.list_commits("ws", "repo").await.unwrap();
    
    assert_eq!(result["values"].as_array().unwrap().len(), 2);
    assert_eq!(result["size"], 2);
}

#[tokio::test]
async fn test_pagination_list_pipelines() {
    let _m = mockito::mock("GET", "/2.0/repositories/ws/repo/pipelines/")
        .with_status(200)
        .with_body(r#"{"values": [{"uuid": "{1}"}, {"uuid": "{2}"}]}"#)
        .create();
    
    let client = make_client(&mockito::server_url());
    let result = client.list_pipelines("ws", "repo").await.unwrap();
    
    assert_eq!(result["values"].as_array().unwrap().len(), 2);
    assert_eq!(result["size"], 2);
}

#[tokio::test]
async fn test_pagination_list_activity() {
    let server_url = mockito::server_url();
    
    let _m1 = mockito::mock("GET", "/2.0/repositories/ws/repo/pullrequests/1/activity")
        .with_status(200)
        .with_body(json!({
            "values": [{"action": "created"}],
            "next": format!("{}/2.0/repositories/ws/repo/pullrequests/1/activity?page=2", server_url)
        }).to_string())
        .create();
    
    let _m2 = mockito::mock("GET", "/2.0/repositories/ws/repo/pullrequests/1/activity?page=2")
        .with_status(200)
        .with_body(r#"{"values": [{"action": "updated"}]}"#)
        .create();
    
    let client = make_client(&server_url);
    let result = client.list_pullrequest_activity("ws", "repo", "1").await.unwrap();
    
    assert_eq!(result["values"].as_array().unwrap().len(), 2);
    assert_eq!(result["size"], 2);
}

#[tokio::test]
async fn test_pagination_error_handling() {
    let server_url = mockito::server_url();
    
    let _m1 = mockito::mock("GET", "/2.0/repositories/ws/repo/pullrequests")
        .with_status(200)
        .with_body(json!({
            "values": [{"id": 1}],
            "next": format!("{}/2.0/repositories/ws/repo/pullrequests?page=2", server_url)
        }).to_string())
        .create();
    
    let _m2 = mockito::mock("GET", "/2.0/repositories/ws/repo/pullrequests?page=2")
        .with_status(500)
        .with_body(r#"{"error": "Server error"}"#)
        .create();
    
    let client = make_client(&server_url);
    let result = client.list_pullrequests("ws", "repo").await;
    
    assert!(result.is_err());
}
