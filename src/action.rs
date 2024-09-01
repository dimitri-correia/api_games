use std::error::Error;
use serde_json::Value;

pub enum Action {
    Move,
    Fight,
    Gathering,
    Unequip,
    Equip,
    Craft,
}

pub fn get_action_name(action: Action) -> &'static str {
    match action {
        Action::Fight => "fight",
        Action::Gathering => "gathering",
        Action::Move => "move",
        Action::Unequip => "unequip",
        Action::Equip => "equip",
        Action::Craft => "crafting",
    }
}

pub async fn extract_cooldown(body: &String) -> Result<f32, Box<dyn Error>> {
    let parsed: Value = serde_json::from_str(body).expect("Failed to parse JSON");

    // Extract the remaining_seconds field from the cooldown object
    if let Some(value) = parsed["data"]["cooldown"]["remaining_seconds"].as_f64() {
        // Convert the found value to f32
        return Ok(value as f32);
    }

    // If the float value wasn't found, return an error
    Err("Failed to extract the cooldown value".into())
}