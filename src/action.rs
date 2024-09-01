use crate::server::Server;
use serde_json::Value;
use std::error::Error;

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

pub async fn handle_action(server: &Server, action: Action, char: &str, mut how_many: i32, json: Option<&Value>) -> Result<(), Box<dyn Error>> {
    let action = get_action_name(action);
    while how_many > 0 {
        println!("[{}] Remaining calls of {}: {}", char, action, how_many);
        let mut response = server.client
            .post(format!("https://api.artifactsmmo.com/my/{}/action/{}", char, action))
            .headers(server.headers.clone());

        if let Some(json) = json {
            response = response.json(json);
        }

        let response = response
            .send()
            .await?;

        let cooldown = extract_cooldown(&response.text().await?).await?;
        println!("[{}] Wait for {}: {}s", char, action, cooldown);
        tokio::time::sleep(tokio::time::Duration::from_secs_f32(cooldown)).await;

        how_many -= 1;
    }

    Ok(())
}