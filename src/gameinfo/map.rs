use crate::gameinfo::GameInfo;
use crate::server::RequestMethod::GET;
use crate::server::Server;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Deserialize, Debug)]
struct MapData {
    data: Vec<MapEntry>,
    pages: usize,
}

#[derive(Deserialize, Debug)]
struct MapEntry {
    x: i32,
    y: i32,
    content: Option<Content>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Content {
    #[serde(rename = "type")]
    content_type: String,
    code: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn distance(&self, other: &Position) -> f64 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f64).sqrt()
    }
}

#[derive(Debug, Clone)]
pub struct Map {
    pub monster: HashMap<String, Vec<Position>>,
    pub resource: HashMap<String, Vec<Position>>,
    pub workshop: HashMap<String, Vec<Position>>,
    pub bank: HashMap<String, Vec<Position>>,
    pub grand_exchange: HashMap<String, Vec<Position>>,
    pub tasks_master: HashMap<String, Vec<Position>>,
}

pub async fn generate_map(server: &Server) -> Map {
    let all_data = collect_from_api(&*server).await;

    // Filter and classify entries into respective categories
    let mut monster = HashMap::new();
    let mut resource = HashMap::new();
    let mut workshop = HashMap::new();
    let mut bank = HashMap::new();
    let mut grand_exchange = HashMap::new();
    let mut tasks_master = HashMap::new();

    for entry in all_data {
        if let Some(content) = entry.content {
            let position = Position {
                x: entry.x,
                y: entry.y,
            };

            match content.content_type.as_str() {
                "monster" => {
                    monster
                        .entry(content.code)
                        .or_insert_with(Vec::new)
                        .push(position);
                }
                "resource" => {
                    resource
                        .entry(content.code)
                        .or_insert_with(Vec::new)
                        .push(position);
                }
                "workshop" => {
                    workshop
                        .entry(content.code)
                        .or_insert_with(Vec::new)
                        .push(position);
                }
                "bank" => {
                    bank.entry(content.code)
                        .or_insert_with(Vec::new)
                        .push(position);
                }
                "grand_exchange" => {
                    grand_exchange
                        .entry(content.code)
                        .or_insert_with(Vec::new)
                        .push(position);
                }
                "tasks_master" => {
                    tasks_master
                        .entry(content.code)
                        .or_insert_with(Vec::new)
                        .push(position);
                }
                _ => {
                    unreachable!("Unknown content type: {}", content.content_type);
                }
            }
        }
    }

    // Create the Map struct with the collected data
    Map {
        monster,
        resource,
        workshop,
        bank,
        grand_exchange,
        tasks_master,
    }
}

async fn collect_from_api(game_info: &Arc<GameInfo>) -> Vec<MapEntry> {
    let mut page = 1;
    let mut all_data = Vec::new();

    // Collect all map data from the API
    loop {
        let mut params = HashMap::new();
        params.insert("size", "100");
        let p = page.to_string();
        params.insert("page", &*p);

        let response = game_info
            .server
            .create_request(GET, "maps".to_string(), None, Some(params))
            .send()
            .await
            .expect("Error sending request");

        let map_data: MapData = response.json().await.expect("Error parsing JSON");

        // Collect all data
        all_data.extend(map_data.data);

        // Check if we've reached the last page
        if page == map_data.pages {
            break;
        }

        // Move to the next page
        page += 1;
    }
    all_data
}
