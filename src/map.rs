use crate::server::Server;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;

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

#[derive(Deserialize, Debug)]
struct Content {
    #[serde(rename = "type")]
    content_type: String,
    code: String,
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
pub struct Map {
    pub monster: HashMap<String, Vec<Position>>,
    pub resource: HashMap<String, Vec<Position>>,
    pub workshop: HashMap<String, Vec<Position>>,
    pub bank: HashMap<String, Vec<Position>>,
    pub grand_exchange: HashMap<String, Vec<Position>>,
    pub tasks_master: HashMap<String, Vec<Position>>,
}

pub async fn generate_map(server: &Server) -> Result<Map, Box<dyn Error>> {
    let mut page = 1;
    let mut all_data = Vec::new();

    // Collect all map data from the API
    loop {
        let mut params = HashMap::new();
        params.insert("size", "100");
        let p = page.to_string();
        params.insert("page", &*p);

        let response = server
            .client
            .get("https://api.artifactsmmo.com/maps")
            .query(&params)
            .headers(server.headers.clone())
            .send()
            .await?;

        let map_data: MapData = response.json().await?;

        // Collect all data
        all_data.extend(map_data.data);

        // Check if we've reached the last page
        if page == map_data.pages {
            break;
        }

        // Move to the next page
        page += 1;
    }

    // Filter and classify entries into respective categories
    let mut monster = HashMap::new();
    let mut resource = HashMap::new();
    let mut workshop = HashMap::new();
    let mut bank = HashMap::new();
    let mut grand_exchange = HashMap::new();
    let mut tasks_master = HashMap::new();

    for entry in all_data {
        if let Some(content) = entry.content {
            let position = Position { x: entry.x, y: entry.y };

            match content.content_type.as_str() {
                "monster" => {
                    monster.entry(content.code).or_insert_with(Vec::new).push(position);
                }
                "resource" => {
                    resource.entry(content.code).or_insert_with(Vec::new).push(position);
                }
                "workshop" => {
                    workshop.entry(content.code).or_insert_with(Vec::new).push(position);
                }
                "bank" => {
                    bank.entry(content.code).or_insert_with(Vec::new).push(position);
                }
                "grand_exchange" => {
                    grand_exchange.entry(content.code).or_insert_with(Vec::new).push(position);
                }
                "tasks_master" => {
                    tasks_master.entry(content.code).or_insert_with(Vec::new).push(position);
                }
                _ => {
                    unreachable!("Unknown content type: {}", content.content_type);
                }
            }
        }
    }

    // Create the Map struct with the collected data
    let map = Map {
        monster,
        resource,
        workshop,
        bank,
        grand_exchange,
        tasks_master,
    };

    Ok(map)
}
