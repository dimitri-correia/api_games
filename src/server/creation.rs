use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Client, RequestBuilder};
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use secrecy::{ExposeSecret, Secret};

#[derive(Clone, Debug)]
pub struct Server {
    client: Client,
    headers: HeaderMap,
}

#[derive(Debug)]
pub enum RequestMethod {
    GET,
    POST,
}

pub fn create_server() -> Server {
    // Load the .env file
    dotenv().ok();

    // create headers
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    // todo use secret
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", get_token())).unwrap(),
    );

    // return client and headers
    Server {
        client: Client::new(),
        headers,
    }
}

impl Server {
    pub fn create_request(
        &self,
        request_method: RequestMethod,
        link: String,
        json: Option<&Value>,
        query: Option<HashMap<&str, &str>>,
    ) -> RequestBuilder
    {
        let url = format!("https://api.artifactsmmo.com/{}", link);

        let mut request =
            match request_method {
                RequestMethod::GET => self.client.get(url),
                RequestMethod::POST => self.client.post(url),
            };

        request = request.headers(self.headers.clone());

        if let Some(json) = json {
            request = request.json(json);
        }

        if let Some(query) = query {
            request = request.query(&query);
        }

        request
    }
}

fn get_token() -> String {
    env::var("token").expect("Token not found in .env file")
}