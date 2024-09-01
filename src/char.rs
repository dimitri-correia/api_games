use crate::server::Server;
use reqwest::{Error, Response};

pub async fn get_char_max_items(server: &Server, char: &str) -> Result<u32, Error> {
    let infos = get_char_infos(server, char).await?;
    let char_infos = serde_json::from_str(&infos.text().await?).expect("Failed to parse JSON");
    char_infos["data"]["inventory_max_items"]["remaining"].as_u32().unwrap()
}

async fn get_char_infos(server: &Server, char: &str) -> Result<Response, Error> {
    server
        .client
        .get(format!("https://api.artifactsmmo.com/characters/{}", char))
        .headers(server.headers.clone())
        .send()
        .await
}