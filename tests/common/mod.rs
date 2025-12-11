// Shared test utilities
use bitbucket_mcp::common::bitbucket::BitbucketClient;
use reqwest::Client;

/// Helper to create a BitbucketClient for unit tests with custom base_url
pub fn make_client(base_url: &str) -> BitbucketClient {
    let base_url = if base_url.ends_with("/2.0") {
        base_url.to_string()
    } else {
        format!("{}/2.0", base_url)
    };
    
    BitbucketClient {
        api_username: "testuser".to_string(),
        app_password: "testpass".to_string(),
        client: Client::new(),
        base_url,
    }
}
