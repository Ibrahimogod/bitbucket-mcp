use actix_web::{web, HttpResponse, Responder, Error};
use actix_web::http::StatusCode;
use serde::Deserialize;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use crate::bitbucket::repo::BitbucketAuth;

#[derive(Deserialize)]
pub struct BitbucketProxyRequest {
    pub method: String, // GET, POST, PUT, DELETE, etc.
    pub path: String,   // e.g. "/2.0/repositories/{workspace}/{repo_slug}/pullrequests"
    pub query: Option<serde_json::Value>, // Optional query params as JSON object
    pub body: Option<serde_json::Value>,  // Optional body as JSON object
}

pub async fn bitbucket_proxy_handler(
    req: web::Json<BitbucketProxyRequest>,
    data: web::Data<BitbucketAuth>,
) -> Result<impl Responder, Error> {
    let client = reqwest::Client::new();
    let mut url = format!("https://api.bitbucket.org{}", req.path);
    if let Some(query) = &req.query {
        let query_string = serde_urlencoded::to_string(query).unwrap_or_default();
        if !query_string.is_empty() {
            url = format!("{}?{}", url, query_string);
        }
    }
    let mut request_builder = match req.method.to_uppercase().as_str() {
        "GET" => client.get(&url),
        "POST" => client.post(&url),
        "PUT" => client.put(&url),
        "DELETE" => client.delete(&url),
        "PATCH" => client.patch(&url),
        _ => return Ok(HttpResponse::BadRequest().body("Unsupported HTTP method")),
    };
    request_builder = request_builder
        .header(AUTHORIZATION, format!("Bearer {}", data.token))
        .header(CONTENT_TYPE, "application/json");
    if let Some(body) = &req.body {
        request_builder = request_builder.json(body);
    }
    let res = request_builder.send().await;
    match res {
        Ok(response) => {
            let status = StatusCode::from_u16(response.status().as_u16()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
            let text = response.text().await.unwrap_or_default();
            Ok(HttpResponse::build(status).body(text))
        }
        Err(e) => Ok(HttpResponse::InternalServerError().body(format!("Request error: {}", e))),
    }
}
