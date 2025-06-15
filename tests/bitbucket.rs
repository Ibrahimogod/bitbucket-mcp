#[tokio::test]
async fn test_get_repository_success() {
    let _m = mockito::mock("GET", "/2.0/repositories/ws/repo")
        .with_status(200)
        .with_body(r#"{"slug": "repo"}"#)
        .create();
    let client = make_client(&mockito::server_url());
    let result = client.get_repository("ws", "repo").await.unwrap();
    assert_eq!(result["slug"], "repo");
}

#[tokio::test]
async fn test_get_repository_error() {
    let _m = mockito::mock("GET", "/2.0/repositories/ws/repo")
        .with_status(404)
        .with_body(r#"{"error": "Not found"}"#)
        .create();
    let client = make_client(&mockito::server_url());
    let result = client.get_repository("ws", "repo").await;
    assert!(result.is_err());
}
#[tokio::test]
async fn test_get_workspace_success() {
    let _m = mockito::mock("GET", "/2.0/workspaces/ws")
        .with_status(200)
        .with_body(r#"{"slug": "ws"}"#)
        .create();
    let client = make_client(&mockito::server_url());
    let result = client.get_workspace("ws").await.unwrap();
    assert_eq!(result["slug"], "ws");
}

#[tokio::test]
async fn test_get_workspace_error() {
    let _m = mockito::mock("GET", "/2.0/workspaces/ws")
        .with_status(404)
        .with_body(r#"{"error": "Not found"}"#)
        .create();
    let client = make_client(&mockito::server_url());
    let result = client.get_workspace("ws").await;
    assert!(result.is_err());
}
#[tokio::test]
async fn test_list_pullrequests_success() {
    let _m = mockito::mock("GET", "/2.0/repositories/ws/repo/pullrequests")
        .with_status(200)
        .with_body(r#"{"values": ["pr1", "pr2"]}"#)
        .create();
    let client = make_client(&mockito::server_url());
    let result = client.list_pullrequests("ws", "repo").await.unwrap();
    assert!(result["values"].is_array());
}

#[tokio::test]
async fn test_list_pullrequests_error() {
    let _m = mockito::mock("GET", "/2.0/repositories/ws/repo/pullrequests")
        .with_status(404)
        .with_body(r#"{"error": "Not found"}"#)
        .create();
    let client = make_client(&mockito::server_url());
    let result = client.list_pullrequests("ws", "repo").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_issues_success() {
    let _m = mockito::mock("GET", "/2.0/repositories/ws/repo/issues")
        .with_status(200)
        .with_body(r#"{"values": ["issue1", "issue2"]}"#)
        .create();
    let client = make_client(&mockito::server_url());
    let result = client.list_issues("ws", "repo").await.unwrap();
    assert!(result["values"].is_array());
}

#[tokio::test]
async fn test_list_issues_error() {
    let _m = mockito::mock("GET", "/2.0/repositories/ws/repo/issues")
        .with_status(500)
        .with_body(r#"{"error": "Server error"}"#)
        .create();
    let client = make_client(&mockito::server_url());
    let result = client.list_issues("ws", "repo").await;
    assert!(result.is_err());
}
#[tokio::test]
async fn test_get_user_success() {
    let _m = mockito::mock("GET", "/2.0/user")
        .with_status(200)
        .with_body(r#"{"username": "testuser"}"#)
        .create();
    let client = make_client(&mockito::server_url());
    let result = client.get_user().await.unwrap();
    assert_eq!(result["username"], "testuser");
}

#[tokio::test]
async fn test_get_user_error() {
    let _m = mockito::mock("GET", "/2.0/user")
        .with_status(401)
        .with_body(r#"{"error": "Unauthorized"}"#)
        .create();
    let client = make_client(&mockito::server_url());
    let result = client.get_user().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_workspaces_success() {
    let _m = mockito::mock("GET", "/2.0/workspaces")
        .with_status(200)
        .with_body(r#"{"values": ["ws1", "ws2"]}"#)
        .create();
    let client = make_client(&mockito::server_url());
    let result = client.list_workspaces().await.unwrap();
    assert!(result["values"].is_array());
}

#[tokio::test]
async fn test_list_workspaces_error() {
    let _m = mockito::mock("GET", "/2.0/workspaces")
        .with_status(500)
        .with_body(r#"{"error": "Server error"}"#)
        .create();
    let client = make_client(&mockito::server_url());
    let result = client.list_workspaces().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_repositories_success() {
    let _m = mockito::mock("GET", "/2.0/repositories/ws")
        .with_status(200)
        .with_body(r#"{"values": ["repo1", "repo2"]}"#)
        .create();
    let client = make_client(&mockito::server_url());
    let result = client.list_repositories("ws").await.unwrap();
    assert!(result["values"].is_array());
}

#[tokio::test]
async fn test_list_repositories_error() {
    let _m = mockito::mock("GET", "/2.0/repositories/ws")
        .with_status(404)
        .with_body(r#"{"error": "Not found"}"#)
        .create();
    let client = make_client(&mockito::server_url());
    let result = client.list_repositories("ws").await;
    assert!(result.is_err());
}
#[tokio::test]
async fn test_list_pullrequest_tasks_success() {
    let _m = mockito::mock("GET", "/2.0/repositories/ws/repo/pullrequests/1/tasks")
        .with_status(200)
        .with_body(r#"{"values": []}"#)
        .create();
    let client = make_client(&mockito::server_url());
    let result = client.list_pullrequest_tasks("ws", "repo", "1").await.unwrap();
    assert!(result["values"].is_array());
}

#[tokio::test]
async fn test_list_pullrequest_tasks_error() {
    let _m = mockito::mock("GET", "/2.0/repositories/ws/repo/pullrequests/1/tasks")
        .with_status(404)
        .with_body(r#"{"error": "Not found"}"#)
        .create();
    let client = make_client(&mockito::server_url());
    let result = client.list_pullrequest_tasks("ws", "repo", "1").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_add_pullrequest_task_success() {
    let _m = mockito::mock("POST", "/2.0/repositories/ws/repo/pullrequests/1/tasks")
        .with_status(201)
        .with_body(r#"{"id": 456, "content": {"raw": "Do this!"}}"#)
        .create();
    let client = make_client(&mockito::server_url());
    let body = serde_json::json!({"content": {"raw": "Do this!"}});
    let result = client.add_pullrequest_task("ws", "repo", "1", body).await.unwrap();
    assert_eq!(result["id"], 456);
}

#[tokio::test]
async fn test_add_pullrequest_task_error() {
    let _m = mockito::mock("POST", "/2.0/repositories/ws/repo/pullrequests/1/tasks")
        .with_status(400)
        .with_body(r#"{"error": "Bad Request"}"#)
        .create();
    let client = make_client(&mockito::server_url());
    let body = serde_json::json!({"content": {"raw": "Do this!"}});
    let result = client.add_pullrequest_task("ws", "repo", "1", body).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_pullrequest_diffstat_success() {
    let _m = mockito::mock("GET", "/2.0/repositories/ws/repo/pullrequests/1/diffstat")
        .with_status(200)
        .with_body(r#"{"values": []}"#)
        .create();
    let client = make_client(&mockito::server_url());
    let result = client.get_pullrequest_diffstat("ws", "repo", "1").await.unwrap();
    assert!(result["values"].is_array());
}

#[tokio::test]
async fn test_get_pullrequest_diffstat_error() {
    let _m = mockito::mock("GET", "/2.0/repositories/ws/repo/pullrequests/1/diffstat")
        .with_status(404)
        .with_body(r#"{"error": "Not found"}"#)
        .create();
    let client = make_client(&mockito::server_url());
    let result = client.get_pullrequest_diffstat("ws", "repo", "1").await;
    assert!(result.is_err());
}
#[tokio::test]
async fn test_merge_pullrequest_success() {
    let _m = mockito::mock("POST", "/2.0/repositories/ws/repo/pullrequests/1/merge")
        .with_status(200)
        .with_body(r#"{"merged": true}"#)
        .create();
    let client = make_client(&mockito::server_url());
    let result = client.merge_pullrequest("ws", "repo", "1", None).await.unwrap();
    assert_eq!(result["merged"], true);
}

#[tokio::test]
async fn test_merge_pullrequest_error() {
    let _m = mockito::mock("POST", "/2.0/repositories/ws/repo/pullrequests/1/merge")
        .with_status(409)
        .with_body(r#"{"error": "Conflict"}"#)
        .create();
    let client = make_client(&mockito::server_url());
    let result = client.merge_pullrequest("ws", "repo", "1", None).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_pullrequest_comments_success() {
    let _m = mockito::mock("GET", "/2.0/repositories/ws/repo/pullrequests/1/comments")
        .with_status(200)
        .with_body(r#"{"values": []}"#)
        .create();
    let client = make_client(&mockito::server_url());
    let result = client.list_pullrequest_comments("ws", "repo", "1").await.unwrap();
    assert!(result["values"].is_array());
}

#[tokio::test]
async fn test_list_pullrequest_comments_error() {
    let _m = mockito::mock("GET", "/2.0/repositories/ws/repo/pullrequests/1/comments")
        .with_status(500)
        .with_body(r#"{"error": "Server error"}"#)
        .create();
    let client = make_client(&mockito::server_url());
    let result = client.list_pullrequest_comments("ws", "repo", "1").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_add_pullrequest_comment_success() {
    let _m = mockito::mock("POST", "/2.0/repositories/ws/repo/pullrequests/1/comments")
        .with_status(201)
        .with_body(r#"{"id": 123, "content": {"raw": "Nice!"}}"#)
        .create();
    let client = make_client(&mockito::server_url());
    let body = serde_json::json!({"content": {"raw": "Nice!"}});
    let payload = bitbucket_mcp::common::bitbucket::normalize_comment_input(body).unwrap();
    let result = client.add_pullrequest_comment("ws", "repo", "1", payload).await.unwrap();
    assert_eq!(result["id"], 123);
}

#[tokio::test]
async fn test_add_pullrequest_comment_error() {
    let _m = mockito::mock("POST", "/2.0/repositories/ws/repo/pullrequests/1/comments")
        .with_status(400)
        .with_body(r#"{"error": "Bad Request"}"#)
        .create();
    let client = make_client(&mockito::server_url());
    let body = serde_json::json!({"content": {"raw": "Nice!"}});
    let payload = bitbucket_mcp::common::bitbucket::normalize_comment_input(body).unwrap();
    let result = client.add_pullrequest_comment("ws", "repo", "1", payload).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_pullrequest_activity_success() {
    let _m = mockito::mock("GET", "/2.0/repositories/ws/repo/pullrequests/1/activity")
        .with_status(200)
        .with_body(r#"{"values": []}"#)
        .create();
    let client = make_client(&mockito::server_url());
    let result = client.list_pullrequest_activity("ws", "repo", "1").await.unwrap();
    assert!(result["values"].is_array());
}

#[tokio::test]
async fn test_list_pullrequest_activity_error() {
    let _m = mockito::mock("GET", "/2.0/repositories/ws/repo/pullrequests/1/activity")
        .with_status(404)
        .with_body(r#"{"error": "Not found"}"#)
        .create();
    let client = make_client(&mockito::server_url());
    let result = client.list_pullrequest_activity("ws", "repo", "1").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_pullrequest_diff_success() {
    let _m = mockito::mock("GET", "/2.0/repositories/ws/repo/pullrequests/1/diff")
        .with_status(200)
        .with_body("diff --git a/file b/file\n...")
        .create();
    let client = make_client(&mockito::server_url());
    let result = client.get_pullrequest_diff("ws", "repo", "1").await.unwrap();
    assert!(result.contains("diff --git"));
}

#[tokio::test]
async fn test_get_pullrequest_diff_error() {
    let _m = mockito::mock("GET", "/2.0/repositories/ws/repo/pullrequests/1/diff")
        .with_status(404)
        .with_body("")
        .create();
    let client = make_client(&mockito::server_url());
    let result = client.get_pullrequest_diff("ws", "repo", "1").await;
    assert!(result.is_err() || result.unwrap().is_empty());
}
#[tokio::test]
async fn test_approve_pullrequest_success() {
    let _m = mockito::mock("POST", "/2.0/repositories/ws/repo/pullrequests/1/approve")
        .with_status(200)
        .with_body(r#"{"approved": true}"#)
        .create();
    let client = make_client(&mockito::server_url());
    let result = client.approve_pullrequest("ws", "repo", "1").await.unwrap();
    assert_eq!(result["approved"], true);
}

#[tokio::test]
async fn test_approve_pullrequest_error() {
    let _m = mockito::mock("POST", "/2.0/repositories/ws/repo/pullrequests/1/approve")
        .with_status(403)
        .with_body(r#"{"error": "Forbidden"}"#)
        .create();
    let client = make_client(&mockito::server_url());
    let result = client.approve_pullrequest("ws", "repo", "1").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_unapprove_pullrequest_success() {
    let _m = mockito::mock("DELETE", "/2.0/repositories/ws/repo/pullrequests/1/approve")
        .with_status(200)
        .with_body(r#"{"approved": false}"#)
        .create();
    let client = make_client(&mockito::server_url());
    let result = client.unapprove_pullrequest("ws", "repo", "1").await.unwrap();
    assert_eq!(result["approved"], false);
}

#[tokio::test]
async fn test_unapprove_pullrequest_error() {
    let _m = mockito::mock("DELETE", "/2.0/repositories/ws/repo/pullrequests/1/approve")
        .with_status(404)
        .with_body(r#"{"error": "Not found"}"#)
        .create();
    let client = make_client(&mockito::server_url());
    let result = client.unapprove_pullrequest("ws", "repo", "1").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_decline_pullrequest_success() {
    let _m = mockito::mock("POST", "/2.0/repositories/ws/repo/pullrequests/1/decline")
        .with_status(200)
        .with_body(r#"{"declined": true}"#)
        .create();
    let client = make_client(&mockito::server_url());
    let result = client.decline_pullrequest("ws", "repo", "1").await.unwrap();
    assert_eq!(result["declined"], true);
}

#[tokio::test]
async fn test_decline_pullrequest_error() {
    let _m = mockito::mock("POST", "/2.0/repositories/ws/repo/pullrequests/1/decline")
        .with_status(400)
        .with_body(r#"{"error": "Bad Request"}"#)
        .create();
    let client = make_client(&mockito::server_url());
    let result = client.decline_pullrequest("ws", "repo", "1").await;
    assert!(result.is_err());
}

use bitbucket_mcp::common::bitbucket::BitbucketClient;
use mockito::mock;
use serde_json::json;

#[tokio::test]
async fn test_delete_repository_success() {
    let _m = mockito::mock("DELETE", "/2.0/repositories/ws/repo")
        .with_status(204)
        .with_body("")
        .create();
    let client = make_client(&mockito::server_url());
    let result = client.delete_repository("ws", "repo").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_repository_error() {
    let _m = mockito::mock("DELETE", "/2.0/repositories/ws/repo")
        .with_status(404)
        .with_body(r#"{"error": "Not found"}"#)
        .create();
    let client = make_client(&mockito::server_url());
    let result = client.delete_repository("ws", "repo").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_branch_success() {
    let _m = mockito::mock("DELETE", "/2.0/repositories/ws/repo/refs/branches/feature-1")
        .with_status(204)
        .with_body("")
        .create();
    let client = make_client(&mockito::server_url());
    let result = client.delete_branch("ws", "repo", "feature-1").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_branch_error() {
    let _m = mockito::mock("DELETE", "/2.0/repositories/ws/repo/refs/branches/feature-1")
        .with_status(404)
        .with_body(r#"{"error": "Not found"}"#)
        .create();
    let client = make_client(&mockito::server_url());
    let result = client.delete_branch("ws", "repo", "feature-1").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_issue_success() {
    let _m = mockito::mock("DELETE", "/2.0/repositories/ws/repo/issues/123")
        .with_status(204)
        .with_body("")
        .create();
    let client = make_client(&mockito::server_url());
    let result = client.delete_issue("ws", "repo", "123").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_issue_error() {
    let _m = mockito::mock("DELETE", "/2.0/repositories/ws/repo/issues/123")
        .with_status(404)
        .with_body(r#"{"error": "Not found"}"#)
        .create();
    let client = make_client(&mockito::server_url());
    let result = client.delete_issue("ws", "repo", "123").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_project_success() {
    let _m = mockito::mock("DELETE", "/2.0/workspaces/ws/projects/PROJ")
        .with_status(204)
        .with_body("")
        .create();
    let client = make_client(&mockito::server_url());
    let result = client.delete_project("ws", "PROJ").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_project_error() {
    let _m = mockito::mock("DELETE", "/2.0/workspaces/ws/projects/PROJ")
        .with_status(404)
        .with_body(r#"{"error": "Not found"}"#)
        .create();
    let client = make_client(&mockito::server_url());
    let result = client.delete_project("ws", "PROJ").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_snippet_success() {
    let _m = mockito::mock("DELETE", "/2.0/snippets/ws/abc123")
        .with_status(204)
        .with_body("")
        .create();
    let client = make_client(&mockito::server_url());
    let result = client.delete_snippet("ws", "abc123").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_snippet_error() {
    let _m = mockito::mock("DELETE", "/2.0/snippets/ws/abc123")
        .with_status(404)
        .with_body(r#"{"error": "Not found"}"#)
        .create();
    let client = make_client(&mockito::server_url());
    let result = client.delete_snippet("ws", "abc123").await;
    assert!(result.is_err());
}

fn make_client(base_url: &str) -> BitbucketClient {
    let base_url = if base_url.ends_with("/2.0") {
        base_url.to_string()
    } else {
        format!("{}/2.0", base_url.trim_end_matches('/'))
    };
    BitbucketClient {
        api_username: "user".to_string(),
        app_password: "pass".to_string(),
        client: reqwest::Client::new(),
        base_url,
    }
}

#[tokio::test]
async fn test_create_pullrequest_success() {
    let _m = mock("POST", "/2.0/repositories/ws/repo/pullrequests")
        .with_status(201)
        .with_body(r#"{"id": 1, "title": "Test PR"}"#)
        .create();
    let url = &mockito::server_url();
    let client = make_client(url);
    let body = json!({"title": "Test PR"});
    let result = client.create_pullrequest("ws", "repo", body).await.unwrap();
    assert_eq!(result["id"], 1);
}

#[tokio::test]
async fn test_get_pullrequest_not_found() {
    let _m = mock("GET", "/2.0/repositories/ws/repo/pullrequests/42")
        .with_status(404)
        .with_body(r#"{"error": {"message": "Not found"}}"#)
        .create();
    let url = &mockito::server_url();
    let client = make_client(url);
    let result = client.get_pullrequest("ws", "repo", "42").await;
    assert!(result.is_err());
}


// --- Scaffold for all BitbucketClient methods ---

#[tokio::test]
async fn test_update_pullrequest_success() {
    let _m = mockito::mock("PUT", "/2.0/repositories/ws/repo/pullrequests/1")
        .with_status(200)
        .with_body(r#"{"id": 1, "title": "Updated PR"}"#)
        .create();
    let client = make_client(&mockito::server_url());
    let body = serde_json::json!({"title": "Updated PR"});
    let result = client.update_pullrequest("ws", "repo", "1", body).await.unwrap();
    assert_eq!(result["id"], 1);
}

#[tokio::test]
async fn test_update_pullrequest_error() {
    let _m = mockito::mock("PUT", "/2.0/repositories/ws/repo/pullrequests/1")
        .with_status(400)
        .with_body(r#"{"error": "Bad Request"}"#)
        .create();
    let client = make_client(&mockito::server_url());
    let body = serde_json::json!({"title": "Updated PR"});
    let result = client.update_pullrequest("ws", "repo", "1", body).await;
    assert!(result.is_err());
}

// Repeat this pattern for all other methods, e.g. approve_pullrequest, unapprove_pullrequest, decline_pullrequest, merge_pullrequest, etc.
// For brevity, only a few are shown here. You can copy and adapt these for each method signature.
