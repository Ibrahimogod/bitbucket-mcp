mod common;

use bitbucket_mcp::common::bitbucket::{BitbucketCommentPayload, normalize_comment_input};
use mockito;
use serde_json::json;
use common::make_client;

#[test]
fn test_normalize_comment_simple_string() {
    let body = json!("This is a simple comment");
    let payload = normalize_comment_input(body).unwrap();
    
    assert_eq!(payload.content.raw, "This is a simple comment");
    assert!(payload.inline.is_none());
}

#[test]
fn test_normalize_comment_with_content_object() {
    let body = json!({
        "content": {
            "raw": "Comment with content object"
        }
    });
    let payload = normalize_comment_input(body).unwrap();
    
    assert_eq!(payload.content.raw, "Comment with content object");
    assert!(payload.inline.is_none());
}

#[test]
fn test_normalize_comment_with_body_field() {
    let body = json!({
        "body": "Comment with body field"
    });
    let payload = normalize_comment_input(body).unwrap();
    
    assert_eq!(payload.content.raw, "Comment with body field");
    assert!(payload.inline.is_none());
}

#[test]
fn test_normalize_comment_with_inline_single_line() {
    let body = json!({
        "body": "This needs fixing",
        "inline": {
            "path": "src/main.rs",
            "from": 42
        }
    });
    let payload = normalize_comment_input(body).unwrap();
    
    assert_eq!(payload.content.raw, "This needs fixing");
    assert!(payload.inline.is_some());
    
    let inline = payload.inline.unwrap();
    assert_eq!(inline.path, "src/main.rs");
    assert_eq!(inline.from, Some(42));
    assert_eq!(inline.to, None);
}

#[test]
fn test_normalize_comment_with_inline_multi_line() {
    let body = json!({
        "body": "Refactor this section",
        "inline": {
            "path": "src/utils.rs",
            "from": 10,
            "to": 20
        }
    });
    let payload = normalize_comment_input(body).unwrap();
    
    assert_eq!(payload.content.raw, "Refactor this section");
    assert!(payload.inline.is_some());
    
    let inline = payload.inline.unwrap();
    assert_eq!(inline.path, "src/utils.rs");
    assert_eq!(inline.from, Some(10));
    assert_eq!(inline.to, Some(20));
}

#[test]
fn test_normalize_comment_with_inline_null_from() {
    let body = json!({
        "body": "New file comment",
        "inline": {
            "path": "src/new.rs",
            "to": 15
        }
    });
    let payload = normalize_comment_input(body).unwrap();
    
    assert_eq!(payload.content.raw, "New file comment");
    assert!(payload.inline.is_some());
    
    let inline = payload.inline.unwrap();
    assert_eq!(inline.path, "src/new.rs");
    assert_eq!(inline.from, None);
    assert_eq!(inline.to, Some(15));
}

#[test]
fn test_normalize_comment_with_content_and_inline() {
    let body = json!({
        "content": {
            "raw": "Code review comment"
        },
        "inline": {
            "path": "tests/test.rs",
            "from": 5,
            "to": 8
        }
    });
    let payload = normalize_comment_input(body).unwrap();
    
    assert_eq!(payload.content.raw, "Code review comment");
    assert!(payload.inline.is_some());
    
    let inline = payload.inline.unwrap();
    assert_eq!(inline.path, "tests/test.rs");
    assert_eq!(inline.from, Some(5));
    assert_eq!(inline.to, Some(8));
}

#[test]
fn test_normalize_comment_invalid_format() {
    let body = json!({
        "invalid": "data"
    });
    let result = normalize_comment_input(body);
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid comment input format");
}

#[test]
fn test_normalize_comment_inline_without_path() {
    let body = json!({
        "body": "Comment without path",
        "inline": {
            "from": 10,
            "to": 15
        }
    });
    let payload = normalize_comment_input(body).unwrap();
    
    // Inline should be None because path is required
    assert_eq!(payload.content.raw, "Comment without path");
    assert!(payload.inline.is_none());
}

#[tokio::test]
async fn test_add_inline_comment_single_line() {
    let _m = mockito::mock("POST", "/2.0/repositories/ws/repo/pullrequests/1/comments")
        .match_body(mockito::Matcher::Json(json!({
            "content": {
                "raw": "Bug here"
            },
            "inline": {
                "path": "src/lib.rs",
                "from": 25
            }
        })))
        .with_status(201)
        .with_body(r#"{"id": 123, "content": {"raw": "Bug here"}}"#)
        .create();
    
    let client = make_client(&mockito::server_url());
    let body = json!({
        "body": "Bug here",
        "inline": {
            "path": "src/lib.rs",
            "from": 25
        }
    });
    let payload = normalize_comment_input(body).unwrap();
    let result = client.add_pullrequest_comment("ws", "repo", "1", payload).await.unwrap();
    
    assert_eq!(result["id"], 123);
}

#[tokio::test]
async fn test_add_inline_comment_multi_line() {
    let _m = mockito::mock("POST", "/2.0/repositories/ws/repo/pullrequests/1/comments")
        .match_body(mockito::Matcher::Json(json!({
            "content": {
                "raw": "Refactor needed"
            },
            "inline": {
                "path": "src/handlers.rs",
                "from": 100,
                "to": 150
            }
        })))
        .with_status(201)
        .with_body(r#"{"id": 456, "content": {"raw": "Refactor needed"}}"#)
        .create();
    
    let client = make_client(&mockito::server_url());
    let body = json!({
        "body": "Refactor needed",
        "inline": {
            "path": "src/handlers.rs",
            "from": 100,
            "to": 150
        }
    });
    let payload = normalize_comment_input(body).unwrap();
    let result = client.add_pullrequest_comment("ws", "repo", "1", payload).await.unwrap();
    
    assert_eq!(result["id"], 456);
}

#[tokio::test]
async fn test_add_general_comment_without_inline() {
    let _m = mockito::mock("POST", "/2.0/repositories/ws/repo/pullrequests/1/comments")
        .match_body(mockito::Matcher::Json(json!({
            "content": {
                "raw": "LGTM!"
            }
        })))
        .with_status(201)
        .with_body(r#"{"id": 789, "content": {"raw": "LGTM!"}}"#)
        .create();
    
    let client = make_client(&mockito::server_url());
    let body = json!({
        "body": "LGTM!"
    });
    let payload = normalize_comment_input(body).unwrap();
    let result = client.add_pullrequest_comment("ws", "repo", "1", payload).await.unwrap();
    
    assert_eq!(result["id"], 789);
}

#[tokio::test]
async fn test_add_inline_comment_error_invalid_line() {
    let _m = mockito::mock("POST", "/2.0/repositories/ws/repo/pullrequests/1/comments")
        .with_status(400)
        .with_body(r#"{"error": {"message": "Invalid line number"}}"#)
        .create();
    
    let client = make_client(&mockito::server_url());
    let body = json!({
        "body": "Comment on invalid line",
        "inline": {
            "path": "src/main.rs",
            "from": 9999
        }
    });
    let payload = normalize_comment_input(body).unwrap();
    let result = client.add_pullrequest_comment("ws", "repo", "1", payload).await;
    
    assert!(result.is_err());
}

#[test]
fn test_inline_comment_payload_serialization() {
    let payload = BitbucketCommentPayload {
        content: bitbucket_mcp::common::bitbucket::BitbucketCommentContent {
            raw: "Test comment".to_string(),
        },
        inline: Some(bitbucket_mcp::common::bitbucket::BitbucketInline {
            from: Some(10),
            to: Some(15),
            path: "src/test.rs".to_string(),
        }),
    };
    
    let json = serde_json::to_value(&payload).unwrap();
    
    assert_eq!(json["content"]["raw"], "Test comment");
    assert_eq!(json["inline"]["path"], "src/test.rs");
    assert_eq!(json["inline"]["from"], 10);
    assert_eq!(json["inline"]["to"], 15);
}

#[test]
fn test_inline_comment_payload_serialization_without_inline() {
    let payload = BitbucketCommentPayload {
        content: bitbucket_mcp::common::bitbucket::BitbucketCommentContent {
            raw: "General comment".to_string(),
        },
        inline: None,
    };
    
    let json = serde_json::to_value(&payload).unwrap();
    
    assert_eq!(json["content"]["raw"], "General comment");
    assert!(json.get("inline").is_none());
}

#[test]
fn test_inline_comment_payload_serialization_null_from() {
    let payload = BitbucketCommentPayload {
        content: bitbucket_mcp::common::bitbucket::BitbucketCommentContent {
            raw: "Added lines".to_string(),
        },
        inline: Some(bitbucket_mcp::common::bitbucket::BitbucketInline {
            from: None,
            to: Some(20),
            path: "src/new.rs".to_string(),
        }),
    };
    
    let json = serde_json::to_value(&payload).unwrap();
    
    assert_eq!(json["content"]["raw"], "Added lines");
    assert_eq!(json["inline"]["path"], "src/new.rs");
    assert!(json["inline"]["from"].is_null());
    assert_eq!(json["inline"]["to"], 20);
}

#[test]
fn test_inline_comment_payload_serialization_null_to() {
    let payload = BitbucketCommentPayload {
        content: bitbucket_mcp::common::bitbucket::BitbucketCommentContent {
            raw: "Single line".to_string(),
        },
        inline: Some(bitbucket_mcp::common::bitbucket::BitbucketInline {
            from: Some(42),
            to: None,
            path: "src/file.rs".to_string(),
        }),
    };
    
    let json = serde_json::to_value(&payload).unwrap();
    
    assert_eq!(json["content"]["raw"], "Single line");
    assert_eq!(json["inline"]["path"], "src/file.rs");
    assert_eq!(json["inline"]["from"], 42);
    assert!(json["inline"]["to"].is_null());
}
