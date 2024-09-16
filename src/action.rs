pub mod bank;
pub mod movement;

use crate::character::CharacterData;
use crate::server::responsecode::ResponseCode;
use crate::server::RequestMethod::POST;
use crate::server::Server;
use crate::utils;
use crate::utils::handle_cooldown;
use reqwest::{Error, RequestBuilder, Response};
use serde::{Deserialize, Deserializer};
use serde_json::Value;
use std::time::Duration;
use std::vec;
use tokio::time;

#[derive(Clone, Copy)]
pub enum Action {
    Move,
    Fight,
    Gathering,
    UnEquip,
    Equip,
    Craft,
    BankDeposit,
    BankDepositGold,
    BankWithdraw,
    BankWithdrawGold,
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
            Action::BankDepositGold => "bank/deposit/gold".to_string(),
            Action::BankWithdrawGold => {}
        }
    }

    fn get_retry_codes(&self) -> Vec<ResponseCode> {
        let mut codes = match self {
            Action::BankDeposit => vec![ResponseCode::TransactionInProgress461],
            _ => Vec::new(),
        };
        codes.push(ResponseCode::Cooldown499);
        codes
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
            utils::info(&*char.name, format!("Inventory {}/{}", inventory_count, response.character.inventory_max_items).as_str());
        }

        // Make the request and handle cooldown
        response = handle_request(request.try_clone().unwrap(), &*char.name, &action).await;

        // either resume if you have done enough or if the inventory is full
        if how_many.is_some() {
            how_many = Some(how_many.unwrap() - 1);
            if how_many == Some(0) {
                return response;
            }
        } else {
            inventory_count = response.character.get_inventory_count();
            if inventory_count == response.character.inventory_max_items {
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
    let mut response = try_sending_request(&request).await;

    let mut number_of_retry_sending_request: u8 = 10;
    while response.is_err() && number_of_retry_sending_request != 0 {
        number_of_retry_sending_request -= 1;
        time::sleep(Duration::from_secs(1)).await;
        response = try_sending_request(&request).await;
    }

    let mut response = response.expect("Error sending request");

    utils::info(char, format!("Calling {} resulted with status: {}", response.url(), response.status().as_u16()).as_str());

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

    info!(char, "[{}] Wait for {}: {}s", char, action, cooldown);
    time::sleep(Duration::from_secs_f32(cooldown)).await;
    handle_cooldown(char, &action.to_string(), parsed_response.cooldown).await;

    parsed_response
}

async fn try_sending_request(request: &RequestBuilder) -> Result<Response, Error> {
    request
        .try_clone()
        .expect("Error cloning")
        .send()
        .await
}

