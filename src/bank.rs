use crate::action::{handle_action_with_cooldown, Action, AllActionResponse};
use crate::character::CharacterData;
use crate::server::Server;
use crate::utils;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
struct DepositItem {
    code: String,
    quantity: u32,
}

async fn deposit_item(server: &Server, char: &str, item_code: &str, quantity: u32) -> AllActionResponse {
    let item_data = DepositItem {
        code: item_code.to_string(),
        quantity,
    };
    handle_action_with_cooldown(server, Action::BankDeposit, char, 1, Some(&json!(item_data))).await
}


pub async fn deposit_all(server: &Server, char: &CharacterData) -> Option<AllActionResponse> {
    let mut updated_char = None;
    for item in char.inventory.iter().clone() {
        if item.quantity > 0 {
            utils::info(&*char.name, format!("Depositing item: {:?}", item).as_str());
            updated_char = Some(deposit_item(server, &char.name, &item.code, item.quantity).await);
        }
    }
    utils::info(&*char.name, "Deposited all items");
    updated_char
}