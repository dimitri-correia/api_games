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
    if false {
        let goto = &json!({
            "x": -1,
            "y": 0
        });
        handle_movement(&headers, &client, goto).await?;
    }

    // Send the request for the fight action
    if false {
        handle_action(&headers, &client, Action::Fight, 1).await?;
    }

    // Send the request for the gathering action
    if true {
        handle_action(&headers, &client, Action::Gathering, 10).await?;
    }

    Ok(())
}

enum Action {
    Fight,
    Gathering,
}

async fn handle_action(headers: &HeaderMap, client: &Client, action: Action, mut how_many: i32) -> Result<(), Box<dyn Error>> {
    let action = match action {
        Action::Fight => "fight",
        Action::Gathering => "gathering",
    };

    while how_many > 0 {
        println!("Remaining calls: {}", how_many);
        let response = client
            .post(format!("https://api.artifactsmmo.com/my/dim/action/{}", action))
            .headers(headers.clone())
            .send()
            .await?;

        let cooldown = extract_cooldown(&response.text().await?).await?;
        println!("Wait: {}s", cooldown);
        tokio::time::sleep(tokio::time::Duration::from_secs_f32(cooldown)).await;

        how_many -= 1;
    }

    Ok(())
}

async fn extract_cooldown(body: &String) -> Result<f32, Box<dyn Error>> {
    let parsed: Value = serde_json::from_str(body).expect("Failed to parse JSON");

    // Extract the remaining_seconds field from the cooldown object
    if let Some(value) = parsed["data"]["cooldown"]["remaining_seconds"].as_f64() {
        // Convert the found value to f32
        return Ok(value as f32);
    }

    // If the float value wasn't found, return an error
    Err("Failed to extract the cooldown value".into())
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