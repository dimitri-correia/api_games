use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde_json::json;
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load the .env file
    dotenv().ok();

    // Create headers
    let headers = create_headers();

    // Create the client
    let client = reqwest::Client::new();

    // Send the request for the movement action
    let response = client
        .post("https://api.artifactsmmo.com/my/dim/action/move")
        .headers(headers.clone())
        .json(&json!({
            "x": 0,
            "y": 1
        }))
        .send()
        .await?;

    // Send the request for the fight action
    let response = client
        .post("https://api.artifactsmmo.com/my/dim/action/fight")
        .headers(headers)
        .send()
        .await?;

    // Handle the response
    let body = response.text().await?;
    println!("{}", body);

    Ok(())
}

fn create_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", get_token())).unwrap(),
    );
    headers
}

fn get_token() -> String {
    env::var("token").expect("Token not found in .env file")
}