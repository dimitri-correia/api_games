use crate::action::{handle_action_with_cooldown, Action, AllActionResponse};
use crate::character::CharacterData;
use crate::server::Server;
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


pub async fn deposit_all(server: &Server, char: &CharacterData) {
    for item in char.inventory.iter().clone() {
        if item.quantity > 0 {
            println!("Depositing item: {:?}", item);
            deposit_item(server, &char.name, &item.code, item.quantity).await;
        }
    }
    println!("Depositing all items");
}