use crate::gameinfo::map::Content;
use crate::server::creation::RequestMethod::GET;
use crate::server::creation::Server;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
pub struct Map {
    pub name: String,
    // pub skin: String,
    pub x: u32,
    pub y: u32,
    pub content: Content,
}

#[derive(Deserialize, Debug, Clone)]
pub struct EventMapItem {
    pub name: String,
    pub map: Map,
    // pub previous_skin: String,
    pub duration: u32,
    pub expiration: String,
    pub created_at: String,
}

#[derive(Deserialize, Debug)]
struct EventPage {
    pub data: Vec<EventMapItem>,
    pub pages: usize,
}

pub async fn get_all_maps_with_events(server: &Server) -> Vec<EventMapItem> {
    let mut page = 1;
    let mut all_data: Vec<EventMapItem> = Vec::new();

    // Collect all map data from the API
    loop {
        let mut params = HashMap::new();
        params.insert("size", "100");
        let p = page.to_string();
        params.insert("page", &*p);

        let response = server
            .create_request(GET, "events".to_string(), None, Some(params))
            .send()
            .await
            .expect("Error sending request");

        let map_page: EventPage = response.json().await.expect("Error parsing JSON");

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
