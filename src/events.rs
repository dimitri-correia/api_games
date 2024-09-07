use crate::server::RequestMethod::GET;
use crate::server::Server;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
pub struct Content {
    pub r#type: String,
    pub code: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Map {
    pub name: String,
    pub skin: String,
    pub x: u32,
    pub y: u32,
    pub content: Content,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MapItem {
    pub name: String,
    pub map: Map,
    pub previous_skin: String,
    pub duration: u32,
    pub expiration: String,
    pub created_at: String,
}

#[derive(Deserialize, Debug)]
pub struct MapPage {
    pub data: Vec<MapItem>,
    pub pages: usize,
}

async fn get_all_maps(server: &Server) -> Vec<MapItem> {
    let mut page = 1;
    let mut all_data = Vec::new();

    // Collect all map data from the API
    loop {
        let mut params = HashMap::new();
        params.insert("size", "100");
        let p = page.to_string();
        params.insert("page", &*p);

        let response = server.create_request(GET, "maps".to_string(), None, Some(params))
            .send()
            .await.expect("Error sending request");

        let map_page: MapPage = response.json().await.expect("Error parsing JSON");

        // Collect all data
        all_data.extend(map_page.data);

        // Check if we've reached the last page
        if page == map_page.pages {
            break;
        }

        // Move to the next page
        page += 1;
    }
    all_data
}
