use crate::action::{handle_action_with_cooldown, Action};
use crate::character::CharacterData;
use crate::gameinfo::items::Item;
use crate::gameinfo::GameInfo;
use crate::movement;
use crate::server::creation::RequestMethod::GET;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::hint::black_box;
use std::sync::Arc;
use tracing::{error, info};

#[derive(Debug, Serialize, Deserialize)]
pub struct Bank {
    pub info: BankInfo,
    pub content: Vec<Item>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BankInfo {
    slots: u32,
    expansions: u32,
    pub next_expansion_cost: u32,
    pub gold: u32,
}

impl Bank {
    pub async fn new(game_info: &Arc<GameInfo>) -> Bank {
        let info = get_bank_details(game_info).await;
        let content = get_all_items_in_bank(game_info).await;

        Bank {
            info,
            content,
        }
    }

    async fn can_store_item(&self, item_code: &str) -> bool {
        let is_already_in_bank = self.content
            .iter()
            .find(|bank_item| bank_item.code == item_code);

        if is_already_in_bank.is_some() {
            return true;
        }

        let free_slots = self.get_free_slots().await;

        free_slots > 0
    }

    async fn get_free_slots(&self) -> usize {
        let total_slots = (self.info.slots + self.info.expansions) as usize;
        let used_slots = self.content.len();

        total_slots - used_slots
    }

    async fn deposit_gold(
        &self,
        game_info: &Arc<GameInfo>,
        mut char: &mut CharacterData,
        quantity: Option<u32>,
    ) {
        let quantity =
            if quantity.is_some() && char.gold >= quantity.unwrap() {
                quantity.unwrap()
            } else {
                char.gold
            };

        if quantity == 0 {
            info!("No gold to deposit");
            return;
        }

        movement::move_to(game_info, &mut char, movement::Place::Bank).await;

        handle_action_with_cooldown(
            game_info,
            Action::BankDepositGold,
            &mut char,
            Some(1),
            Some(&json!(quantity)),
        ).await;
    }

    async fn deposit_item(
        &self,
        game_info: &Arc<GameInfo>,
        mut char: &mut CharacterData,
        item_code: &str,
        quantity: Option<u32>,
    ) {
        if !self.can_store_item(item_code) {
            error!("Cannot store item {} in bank", item_code);
            return;
        }

        let items_in_inventory = char.inventory
            .iter()
            .find(|item| item.code == item_code);

        if items_in_inventory.is_none() {
            error!("Item {} not found in inventory", item_code);
            return;
        }

        let quantity =
            if quantity.is_some() && items_in_inventory.unwrap().quantity >= quantity.unwrap() {
                quantity.unwrap()
            } else {
                items_in_inventory.unwrap().quantity
            };

        if quantity == 0 {
            info!("No item {} to deposit", item_code);
            return;
        }

        movement::move_to(game_info, &mut char, movement::Place::Bank).await;

        let item = Item {
            code: item_code.to_string(),
            quantity,
        };
        handle_action_with_cooldown(
            game_info,
            Action::BankDeposit,
            &mut char,
            Some(1),
            Some(&json!(item)),
        ).await;
    }

    pub async fn deposit_all_items_and_gold(
        &self,
        game_info: &Arc<GameInfo>,
        mut char: &mut CharacterData,
    ) {
        self.deposit_gold(game_info, &mut char, None).await;

        if char.get_inventory_count() == 0 {
            info!("No items to deposit");
            return;
        }

        movement::move_to(game_info, &mut char, movement::Place::Bank).await;

        for item in char.inventory.iter().clone() {
            if item.quantity > 0 {
                info!("Depositing item: {:?}", item);
                self.deposit_item(game_info, &mut char, &item.code, None).await;
            }
        }

        info!("Deposited all items");
    }

    pub async fn withdraw_item(
        &self,
        game_info: &Arc<GameInfo>,
        mut char: &mut CharacterData,
        item_code: &str,
        qtt: u32,
    ) {
        let item_is_in_bank = self.content
            .iter()
            .find(|bank_item| bank_item.code == item_code);

        if item_is_in_bank.is_none() {
            error!("Item {} not found in bank", item_code);
            return;
        }

        if item_is_in_bank.unwrap().quantity < qtt {
            error!("Not enough item {} in bank", item_code);
            return;
        }

        movement::move_to(game_info, &mut char, movement::Place::Bank).await;

        let item = Some(&json!(Item {
            code: item_code.to_string(),
            quantity: qtt,
        }));

        handle_action_with_cooldown(
            game_info,
            Action::BankWithdraw,
            &mut char,
            Some(1),
            item,
        ).await;
    }

    pub async fn withdraw_gold(
        &self,
        game_info: &Arc<GameInfo>,
        mut char: &mut CharacterData,
        quantity: u32,
    ) {
        if self.info.gold < quantity {
            error!("Not enough gold in bank");
            return;
        }

        movement::move_to(game_info, &mut char, movement::Place::Bank).await;

        handle_action_with_cooldown(
            game_info,
            Action::BankWithdrawGold,
            &mut char,
            Some(1),
            Some(&json!(quantity)),
        ).await;
    }
}


#[derive(Deserialize, Debug)]
struct BankPage {
    pub data: Vec<Item>,
    pub pages: usize,
}

async fn get_all_items_in_bank(game_info: &Arc<GameInfo>) -> Vec<Item> {
    let mut page = 1;
    let mut all_data = Vec::new();

    // Collect all map data from the API
    loop {
        let mut params = HashMap::new();
        params.insert("size", "100");
        let p = page.to_string();
        params.insert("page", &*p);

        let response = game_info.server
            .create_request(GET, "my/bank/items".to_string(), None, Some(params))
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

#[derive(Debug, Deserialize)]
struct BankInfoResponse {
    data: BankInfo,
}

async fn get_bank_details(game_info: &Arc<GameInfo>) -> BankInfo {
    game_info.server
        .create_request(GET, "my/bank".to_string(), None, None)
        .send()
        .await
        .expect("Error sending request")
        .json::<BankInfoResponse>()
        .await
        .expect("Error parsing JSON")
        .data
}
