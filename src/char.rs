use crate::server::Server;
use reqwest::{Error, Response};
use serde_json::Value;

pub async fn get_char_max_items(server: &Server, char: &str) -> Result<u32, Error> {
    let infos = get_char_infos(server, char).await?;
    let char_infos: Value = serde_json::from_str(&infos.text().await?).unwrap();

    let max_items = char_infos["data"]["inventory_max_items"]
        .as_u64().unwrap() as u32;

    Ok(max_items)
}

pub async fn get_char_all_items(server: &Server, char: &str) -> Result<Vec<Value>, Error> {
    let infos = get_char_infos(server, char).await?;
    let char_infos: Value = serde_json::from_str(&infos.text().await?).unwrap();

    let inventory = char_infos["data"]["inventory"]
        .as_array().unwrap().to_vec();

    Ok(inventory)
}

async fn get_char_infos(server: &Server, char: &str) -> Result<Response, Error> {
    server
        .client
        .get(format!("https://api.artifactsmmo.com/characters/{}", char))
        .headers(server.headers.clone())
        .send()
        .await
}
