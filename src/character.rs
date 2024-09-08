use crate::server::RequestMethod::GET;
use crate::server::Server;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct InventoryItem {
    pub slot: u8,
    pub code: String,
    pub quantity: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CharacterData {
    pub name: String,
    // skin: String,
    level: u32,
    xp: u32,
    max_xp: u32,
    achievements_points: u32,
    gold: u32,
    speed: u32,
    pub mining_level: u32,
    // mining_xp: u32,
    // mining_max_xp: u32,
    pub woodcutting_level: u32,
    // woodcutting_xp: u32,
    // woodcutting_max_xp: u32,
    pub fishing_level: u32,
    // fishing_xp: u32,
    // fishing_max_xp: u32,
    pub weaponcrafting_level: u32,
    // weaponcrafting_xp: u32,
    // weaponcrafting_max_xp: u32,
    pub gearcrafting_level: u32,
    // gearcrafting_xp: u32,
    // gearcrafting_max_xp: u32,
    pub jewelrycrafting_level: u32,
    // jewelrycrafting_xp: u32,
    // jewelrycrafting_max_xp: u32,
    pub cooking_level: u32,
    // cooking_xp: u32,
    // cooking_max_xp: u32,
    hp: u32,
    haste: u32,
    critical_strike: u32,
    stamina: u32,
    attack_fire: u32,
    attack_earth: u32,
    attack_water: u32,
    attack_air: u32,
    dmg_fire: u32,
    dmg_earth: u32,
    dmg_water: u32,
    dmg_air: u32,
    res_fire: u32,
    res_earth: u32,
    res_water: u32,
    res_air: u32,
    pub x: i32,
    pub y: i32,
    cooldown: u32,
    pub cooldown_expiration: String,
    weapon_slot: String,
    shield_slot: String,
    helmet_slot: String,
    body_armor_slot: String,
    leg_armor_slot: String,
    boots_slot: String,
    ring1_slot: String,
    ring2_slot: String,
    amulet_slot: String,
    artifact1_slot: String,
    artifact2_slot: String,
    artifact3_slot: String,
    consumable1_slot: String,
    consumable1_slot_quantity: u32,
    consumable2_slot: String,
    consumable2_slot_quantity: u32,
    task: String,
    task_type: String,
    task_progress: u32,
    task_total: u32,
    pub inventory_max_items: u32,
    pub inventory: Vec<InventoryItem>,
}

#[derive(Debug, Deserialize)]
struct CharacterResponse {
    data: CharacterData,
}
#[derive(Debug, Deserialize)]
struct AllCharactersResponse {
    data: Vec<CharacterData>,
}

pub async fn get_char_infos(server: &Server, character: &str) -> CharacterData {
    server.create_request(GET, format!("characters/{}", character), None, None)
        .send()
        .await.expect("Error sending request")
        .json::<CharacterResponse>()
        .await.expect("Error parsing JSON")
        .data
}

pub async fn get_all_chars_infos(server: &Server) -> Vec<CharacterData> {
    server
        .create_request(GET, "my/characters".to_string(), None, None)
        .send()
        .await
        .expect("Error sending request")
        .json::<AllCharactersResponse>()
        .await
        .expect("Error parsing JSON").data
}
