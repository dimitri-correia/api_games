use crate::action::equipment::SlotType;
use crate::action::fight::AttackStats;
use crate::gameinfo::items::Item;
use crate::gameinfo::map::Position;
use crate::server::creation::RequestMethod::GET;
use crate::server::creation::Server;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct CharacterData {
    pub name: String,
    // skin: String,
    pub level: u32,
    xp: u32,
    max_xp: u32,
    achievements_points: u32,
    pub gold: u32,
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
    res_fire: i32,
    res_earth: i32,
    res_water: i32,
    res_air: i32,
    x: i32,
    y: i32,
    cooldown: u32,
    cooldown_expiration: String,
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
    pub inventory: Vec<Item>,
}

impl CharacterData {
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
            dmg_fire: Some(self.dmg_fire),
            dmg_earth: Some(self.dmg_earth),
            dmg_water: Some(self.dmg_water),
            dmg_air: Some(self.dmg_air),
        }
    }
}

#[derive(Deserialize)]
struct CharacterResponse {
    data: CharacterData,
}

#[derive(Debug, Deserialize)]
struct AllCharactersResponse {
    data: Vec<CharacterData>,
}

pub async fn get_char_infos(server: &Server, character: &str) -> CharacterData {
    server
        .create_request(GET, format!("characters/{}", character), None, None)
        .send()
        .await
        .expect("Error sending request")
        .json::<CharacterResponse>()
        .await
        .expect("Error parsing JSON")
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
        .expect("Error parsing JSON")
        .data
}

impl CharacterData {
    pub fn get_inventory_count(&self) -> u32 {
        self.inventory.iter().map(|item| item.quantity).sum()
    }

    pub fn get_current_position(&self) -> Position {
        Position {
            x: self.x,
            y: self.y,
        }
    }

    pub fn get_equipment(&self, slot_type: &SlotType) -> String {
        match slot_type {
            SlotType::Weapon => self.weapon_slot.clone(),
            SlotType::Shield => self.shield_slot.clone(),
            SlotType::Helmet => self.helmet_slot.clone(),
            SlotType::BodyArmor => self.body_armor_slot.clone(),
            SlotType::LegArmor => self.leg_armor_slot.clone(),
            SlotType::Boots => self.boots_slot.clone(),
            SlotType::Ring1 => self.ring1_slot.clone(),
            SlotType::Ring2 => self.ring2_slot.clone(),
            SlotType::Amulet => self.amulet_slot.clone(),
            SlotType::Artifact1 => self.artifact1_slot.clone(),
            SlotType::Artifact2 => self.artifact2_slot.clone(),
            SlotType::Artifact3 => self.artifact3_slot.clone(),
            SlotType::Consumable1 => self.consumable1_slot.clone(),
            SlotType::Consumable2 => self.consumable2_slot.clone(),
        }
    }
}
