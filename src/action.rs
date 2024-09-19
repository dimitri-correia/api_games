pub mod bank;
pub mod movement;
pub mod equipment;

use crate::action::bank::ErrorBank;
use crate::character::CharacterData;
use crate::gameinfo::GameInfo;
use crate::server::creation::RequestMethod::POST;
use crate::server::responsecode::ResponseCode;
use log::info;
use reqwest::{Error, RequestBuilder, Response};
use serde::{Deserialize, Deserializer};
use serde_json::Value;
use std::sync::Arc;
use std::time::Duration;
use std::vec;
use tokio::time;

pub enum ErrorAction {
    ErrorParsingResponse,
    ErrorSendingRequest,
}

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

    fn get_retry_codes(&self) -> Vec<u16> {
        let mut codes = match self {
            Action::BankDeposit => vec![ResponseCode::TransactionInProgress461],
            _ => Vec::new(),
        };
        codes.push(ResponseCode::Cooldown499);
        codes.iter().map(|x| x.get_code()).collect::<Vec<u16>>()
    }
}

pub async fn handle_action_with_cooldown(
    game_info: Arc<GameInfo>,
    action: Action,
    mut char: &mut CharacterData,
    mut how_many: Option<u32>,
    json: Option<&Value>,
) -> Result<(), ErrorAction> {
    let request = game_info.server
        .create_request(
            POST,
            format!("my/{}/action/{}", char.name, action.to_string()),
            json,
            None,
        );

    // Loop through the calls
    loop {
        if let Some(how_many) = how_many {
            info!("Remaining calls of {}: {}", action.to_string(), how_many);
        } else {
            info!("Inventory {}/{}", char.get_inventory_count(), char.inventory_max_items);
        }

        // Make the request and handle cooldown
        handle_request(
            request.try_clone().unwrap(),
            char,
            &action,
        ).await?;

        // Check if we need to stop
        if how_many.is_some() {
            how_many = Some(how_many.unwrap() - 1);
            if how_many == Some(0) {
                return Ok(());
            }
        } else {
            if char.get_inventory_count() == char.inventory_max_items {
                return Ok(());
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

async fn handle_request(
    request: RequestBuilder,
    mut char: &mut CharacterData,
    action: &Action,
) -> Result<(), ErrorAction> {
    let mut response = send_request_with_exponential_backoff(&request).await?;

    info!("Calling {} resulted with status: {}", response.url(), response.status().as_u16());

    while action.get_retry_codes().contains(&response.status().as_u16()) {
        info!("Retrying action due to status: {}", response.status().as_u16());
        response = send_request_with_exponential_backoff(&request).await?;
    }

    let mut parsed_response = response
        .json::<ActionResponse>()
        .await
        .map_err(|_| ErrorAction::ErrorParsingResponse)?
        .data;

    let cooldown = parsed_response.cooldown;
    info!("Wait for {}: {}s", action, cooldown);
    time::sleep(Duration::from_secs_f32(cooldown)).await;

    char = &mut parsed_response.character;

    Ok(())
}

async fn send_request_with_exponential_backoff(request: &RequestBuilder) -> Result<Response, ErrorAction> {
    let mut response = try_sending_request(&request).await;

    let mut number_of_retry_sending_request: u8 = 10;
    let mut backoff_duration = Duration::from_secs(1);
    while response.is_err() && number_of_retry_sending_request != 0 {
        number_of_retry_sending_request -= 1;
        time::sleep(backoff_duration).await;
        backoff_duration *= 2; // Exponential backoff
        response = try_sending_request(&request).await;
    }
    response.map_err(|_| ErrorAction::ErrorSendingRequest)
}

async fn try_sending_request(request: &RequestBuilder) -> Result<Response, Error> {
    request
        .try_clone()
        .expect("Error cloning")
        .send()
        .await
}

