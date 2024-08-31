use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use serde_json::{json, Value};
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
    if true {
        let goto = &json!({
            "x": -1,
            "y": 0
        });
        handle_movement(&headers, &client, goto).await?;
    }

    // Send the request for the fight action
    if false {
        handle_fight(&headers, &client).await?;
    }

    Ok(())
}

async fn handle_fight(headers: &HeaderMap, client: &Client) -> Result<(), Box<dyn Error>> {
    let response = client
        .post("https://api.artifactsmmo.com/my/dim/action/fight")
        .headers(headers.clone())
        .send()
        .await?.text().await?;

    println!("Fight response: {}", response);

    Ok(())
}

async fn handle_movement(headers: &HeaderMap, client: &Client, goto: &Value) -> Result<(), Box<dyn Error>> {
    let response = client
        .post("https://api.artifactsmmo.com/my/dim/action/move")
        .headers(headers.clone())
        .json(goto)
        .send()
        .await?.text().await?;

    println!("Movement response: {}", response);

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