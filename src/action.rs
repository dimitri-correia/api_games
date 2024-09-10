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
    BankWithdraw,
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
            Action::BankWithdraw => "bank/withdraw".to_string(),
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
    char: &CharacterData,
    mut how_many: Option<u32>,
    json: Option<&Value>,
) -> AllActionResponse {
    let request = server
        .create_request(POST, format!("my/{}/action/{}", char.name, action.to_string()), json, None);

    let mut response;
    let mut inventory_count = char.get_inventory_count();

    // Loop through the calls
    loop {
        if let Some(how_many) = how_many {
            utils::info(&*char.name, format!("Remaining calls of {}: {}", action.to_string(), how_many).as_str());
        } else {
            utils::info(&*char.name, format!("Inventory {}/{}", inventory_count, char.inventory_max_items).as_str());
        }

        // Make the request and handle cooldown
        response = handle_request(request.try_clone().unwrap(), &*char.name, &action).await;
        // let mut updated_char = response.character.clone();
        // char = &mut updated_char;

        // either resume if you have done enough or if the inventory is full
        if how_many.is_some() {
            how_many = Some(how_many.unwrap() - 1);
            if how_many == Some(0) {
                return response;
            }
        } else {
            inventory_count = char.get_inventory_count();
            utils::info(&*char.name, format!("Inventory: {:?}", char.inventory).as_str());
            if inventory_count == char.inventory_max_items {
                return response;
            }
        }
    }
}


#[derive(Debug, Deserialize)]
struct ActionResponse {
    data: AllActionResponse,
}

#[derive(Debug, Deserialize, Clone)]
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

