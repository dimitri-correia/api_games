use crate::character::CharacterData;
use crate::server::Server;
use crate::utils::handle_cooldown;
use reqwest::RequestBuilder;
use serde::{Deserialize, Deserializer};
use serde_json::Value;
use crate::server::RequestMethod::POST;

pub enum Action {
    Move,
    Fight,
    Gathering,
    Unequip,
    Equip,
    Craft,
    BankDeposit,
}

fn get_action_name(action: Action) -> &'static str {
    match action {
        Action::Fight => "fight",
        Action::Gathering => "gathering",
        Action::Move => "move",
        Action::Unequip => "unequip",
        Action::Equip => "equip",
        Action::Craft => "crafting",
        Action::BankDeposit => "bank/deposit",
    }
}

pub async fn handle_action_with_cooldown(
    server: &Server,
    action: Action,
    char: &str,
    mut how_many: u32,
    json: Option<&Value>,
) -> AllActionResponse {
    let action_name = get_action_name(action);
    let request = server.create_request(POST, format!("my/{}/action/{}", char, action_name), json, None);

    // Loop through the calls, stopping before the last one to handle it separately
    while how_many > 1 {
        println!("[{}] Remaining calls of {}: {}", char, action_name, how_many);

        // Make the request and handle cooldown
        let response = send_request(request.try_clone().unwrap()).await;
        handle_cooldown(char, &action_name, response.cooldown).await;

        how_many -= 1;
    }

    // Last call, return the response from the final request
    let final_response = send_request(request).await;
    handle_cooldown(char, &action_name, final_response.cooldown).await;

    final_response
}


#[derive(Debug, Deserialize)]
struct ActionResponse {
    data: AllActionResponse,
}

#[derive(Debug, Deserialize)]
pub struct AllActionResponse {
    // to get directly the cooldown remaining
    #[serde(deserialize_with = "deserialize_cooldown")]
    cooldown: f32,
    character_data: CharacterData,
}

fn deserialize_cooldown<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Cooldown {
        remaining_seconds: f32,
    }
    Ok(Cooldown::deserialize(deserializer)?.remaining_seconds)
}

async fn send_request(request: RequestBuilder) -> AllActionResponse {
    request
        .send()
        .await.expect("Error sending request")
        .json::<AllActionResponse>()
        .await.expect("Error parsing JSON")
}

