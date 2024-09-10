use crate::action::{handle_action_with_cooldown, Action, AllActionResponse};
use crate::character::CharacterData;
use crate::server::RequestMethod::GET;
use crate::server::Server;
use crate::utils;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use crate::gameinfo::items::{CraftItem, Item};

#[derive(Debug, Serialize, Deserialize)]
pub struct BankItem {
    pub code: String,
    quantity: u32,
}

async fn deposit_item(server: &Server, char: &CharacterData, item_code: &str, quantity: u32) -> AllActionResponse {
    let item_data = BankItem {
        code: item_code.to_string(),
        quantity,
    };
    handle_action_with_cooldown(server, Action::BankDeposit, char, Some(1), Some(&json!(item_data))).await
}


pub async fn deposit_all(server: &Server, char: &CharacterData) -> Option<AllActionResponse> {
    let mut updated_char = None;
    for item in char.inventory.iter().clone() {
        if item.quantity > 0 {
            utils::info(&*char.name, format!("Depositing item: {:?}", item).as_str());
            updated_char = Some(deposit_item(server, char, &item.code, item.quantity).await);
        }
    }
    utils::info(&*char.name, "Deposited all items");
    updated_char
}

pub async fn withdraw_item(server: &Server, char: &CharacterData, item: CraftItem) -> Option<AllActionResponse> {
    let response = handle_action_with_cooldown(server, Action::BankWithdraw, char, Some(1), Some(&json!(item))).await;
    Some(response)

}

#[derive(Deserialize, Debug)]
struct BankPage {
    pub data: Vec<BankItem>,
    pub pages: usize,
}

pub async fn get_all_items_in_bank(server: &Server) -> Vec<BankItem> {
    let mut page = 1;
    let mut all_data = Vec::new();

    // Collect all map data from the API
    loop {
        let mut params = HashMap::new();
        params.insert("size", "100");
        let p = page.to_string();
        params.insert("page", &*p);

        let response = server.create_request(GET, "my/bank/items".to_string(), None, Some(params))
            .send()
            .await.expect("Error sending request");

        let bank_items: BankPage = response.json().await.expect("Error parsing JSON");

        // Collect all data
        all_data.extend(bank_items.data);

        // Check if we've reached the last page
        if page == bank_items.pages {
            break;
        }

        // Move to the next page
        page += 1;
    }
    all_data
}
