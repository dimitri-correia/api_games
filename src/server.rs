use std::env;
use dotenv::dotenv;
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE};

pub fn create_client_and_headers() -> (Client, HeaderMap) {
    // Load the .env file
    dotenv().ok();

    // create headers
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", get_token())).unwrap(),
    );

    // return client and headers
    (Client::new(), headers)
}

fn get_token() -> String {
    env::var("token").expect("Token not found in .env file")
}