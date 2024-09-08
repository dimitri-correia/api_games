use crate::character::CharacterData;
use crate::responsecode::ResponseCode;
use crate::server::RequestMethod::POST;
use crate::server::Server;
use crate::utils;
use crate::utils::handle_cooldown;
use reqwest::RequestBuilder;
use serde::{Deserialize, Deserializer};
use serde_json::Value;

#[derive(Clone, Copy)]
pub enum Action {
    Move,
    Fight,
    Gathering,
    UnEquip,
    Equip,
    Craft,
    BankDeposit,
}

impl Action {
    fn to_string(&self) -> String {
        match self {
            Action::Move => "move".to_string(),
            Action::Fight => "fight".to_string(),
            Action::Gathering => "gathering".to_string(),
            Action::UnEquip => "unequip".to_string(),
            Action::Equip => "equip".to_string(),
            Action::Craft => "crafting".to_string(),
            Action::BankDeposit => "bank/deposit".to_string(),
        }
    }

    fn get_retry_codes(&self) -> Vec<ResponseCode> {
        match self {
            Action::BankDeposit => vec![ResponseCode::TransactionInProgress461],
            _ => vec![]
        }
    }
}

pub async fn handle_action_with_cooldown(
    server: &Server,
    action: Action,
    char: &str,
    mut how_many: u32,
    json: Option<&Value>,
) -> AllActionResponse {
    let request = server
        .create_request(POST, format!("my/{}/action/{}", char, action.to_string()), json, None);

    let mut response;

    // Loop through the calls
    loop {
        utils::info(char, format!("Remaining calls of {}: {}", action.to_string(), how_many).as_str());

        // Make the request and handle cooldown
        response = handle_request(request.try_clone().unwrap(), char, &action).await;

        how_many -= 1;
        if how_many == 0 {
            return response;
        }
    }
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
    pub character: CharacterData,
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

async fn handle_request(request: RequestBuilder, char: &str, action: &Action) -> AllActionResponse {
    let mut response = request.try_clone().expect("Error cloning")
        .send().await.expect("Error sending request");

    utils::info(char, format!("Calling {} with status: {}", response.url(), response.status()).as_str());

    while action.get_retry_codes().iter().map(|x| x.get_code()).collect::<Vec<u16>>()
        .contains(&response.status().as_u16()) {
        utils::info(char, format!("Retrying action due to status: {}", response.status()).as_str());
        response = request.try_clone().expect("Error cloning")
            .send().await.expect("Error sending request");
    }

    let parsed_response = response
        .json::<ActionResponse>()
        .await
        .expect("Error parsing JSON")
        .data;

    handle_cooldown(char, &action.to_string(), parsed_response.cooldown).await;

    parsed_response
}

