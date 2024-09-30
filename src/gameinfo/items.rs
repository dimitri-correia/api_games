use crate::server::creation::RequestMethod::GET;
use crate::server::creation::Server;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
pub struct Effect {
    pub name: String,
    pub value: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Item {
    pub slot: Option<u8>,
    pub code: String,
    pub quantity: u32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Craft {
    pub skill: String,
    pub level: u32,
    pub items: Vec<Item>,
    pub quantity: u32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ItemInfo {
    pub name: String,
    pub code: String,
    pub level: u32,
    pub r#type: String,
    pub subtype: String,
    pub description: String,
    pub effects: Vec<Effect>,
    pub craft: Option<Craft>,
}

#[derive(Deserialize, Debug)]
pub struct ItemPage {
    pub data: Vec<ItemInfo>,
    pub pages: usize,
}

pub async fn get_all_items(server: &Server) -> Vec<ItemInfo> {
    let mut page = 1;
    let mut all_data = Vec::new();

    // Collect all item data from the API
    loop {
        let mut params = HashMap::new();
        params.insert("size", "100");
        let p = page.to_string();
        params.insert("page", &*p);

        let response = server
            .create_request(GET, "items".to_string(), None, Some(params))
            .send()
            .await
            .expect("Error sending request");

        let item_page: ItemPage = response.json().await.expect("Error parsing JSON");

        // Collect all data
        all_data.extend(item_page.data);

        // Check if we've reached the last page
        if page == item_page.pages {
            break;
        }

        // Move to the next page
        page += 1;
    }
    all_data
}
