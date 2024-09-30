use crate::action::fight::AttackStats;
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
    hp: u32,
    attack_fire: u32,
    attack_earth: u32,
    attack_water: u32,
    attack_air: u32,
    res_fire: i32,
    res_earth: i32,
    res_water: i32,
    res_air: i32,
    pub min_gold: u32,
    pub max_gold: u32,
    pub drops: Vec<Drop>,
}

impl Monster {
    pub fn get_attack_stats(&self) -> AttackStats {
        AttackStats {
            hp: self.hp,
            attack_fire: self.attack_fire,
            attack_earth: self.attack_earth,
            attack_water: self.attack_water,
            attack_air: self.attack_air,
            res_fire: self.res_fire,
            res_earth: self.res_earth,
            res_water: self.res_water,
            res_air: self.res_air,
            dmg_fire: None,
            dmg_earth: None,
            dmg_water: None,
            dmg_air: None,
        }
    }
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

        let response = server
            .create_request(GET, "monsters".to_string(), None, Some(params))
            .send()
            .await
            .expect("Error sending request");

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
