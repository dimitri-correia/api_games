use crate::server::RequestMethod::GET;
use crate::server::Server;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
pub struct Drop {
    pub code: String,
    pub rate: f32,
    pub min_quantity: u32,
    pub max_quantity: u32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Monster {
    pub name: String,
    pub code: String,
    pub level: u32,
    pub hp: u32,
    pub attack_fire: u32,
    pub attack_earth: u32,
    pub attack_water: u32,
    pub attack_air: u32,
    pub res_fire: u32,
    pub res_earth: u32,
    pub res_water: u32,
    pub res_air: u32,
    pub min_gold: u32,
    pub max_gold: u32,
    pub drops: Vec<Drop>,
}

#[derive(Deserialize, Debug)]
pub struct MonsterPage {
    pub data: Vec<Monster>,
    pub pages: usize,
}


pub async fn get_all_monsters(server: &Server) -> Vec<Monster> {
    let mut page = 1;
    let mut all_data = Vec::new();

    // Collect all map data from the API
    loop {
        let mut params = HashMap::new();
        params.insert("size", "100");
        let p = page.to_string();
        params.insert("page", &*p);

        let response = server.create_request(GET, "monsters".to_string(), None, Some(params))
            .send()
            .await.expect("Error sending request");

        let monster_page: MonsterPage = response.json().await.expect("Error parsing JSON");

        // Collect all data
        all_data.extend(monster_page.data);

        // Check if we've reached the last page
        if page == monster_page.pages {
            break;
        }

        // Move to the next page
        page += 1;
    }
    all_data
}
