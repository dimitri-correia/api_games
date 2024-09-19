use crate::action::{handle_action_with_cooldown, Action};
use crate::action::{movement, ErrorAction};
use crate::character::CharacterData;
use crate::gameinfo::items::Item;
use crate::gameinfo::GameInfo;
use crate::server::creation::RequestMethod::GET;
use crate::server::request::send_request_with_exponential_backoff;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::hint::black_box;
use std::sync::Arc;
use tracing::{error, info};

pub enum ErrorBank {
    NotEnoughGoldInBank,
    NotEnoughPlaceInBank,
    NotInInventory,
    NotEnoughItemInBank,
    ItemNotFoundInBank,
    ErrorAction(ErrorAction),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bank {
    pub info: BankInfo,
    pub content: Vec<Item>,
    pub game_info: Arc<GameInfo>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BankInfo {
    slots: u32,
    expansions: u32,
    pub next_expansion_cost: u32,
    pub gold: u32,
}

impl Bank {
    pub async fn new(game_info: &Arc<GameInfo>) -> Result<Bank, ErrorBank> {
        let info = get_bank_details(game_info).await?;
        let content = get_all_items_in_bank(game_info).await?;

        let bank = Bank {
            info,
            content,
            game_info: Arc::clone(game_info),
        };

        Ok(bank)
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
        mut char: &mut CharacterData,
        quantity: Option<u32>,
    ) -> Result<(), ErrorBank> {
        let quantity =
            if quantity.is_some() && char.gold >= quantity.unwrap() {
                quantity.unwrap()
            } else {
                char.gold
            };

        if quantity == 0 {
            info!("No gold to deposit");
            return Ok(());
        } else {
            info!("Depositing gold: {}", quantity);
        }

        movement::move_to(&self.game_info, &mut char, movement::Place::Bank).await;

        handle_action_with_cooldown(
            &self.game_info,
            Action::BankDepositGold,
            &mut char,
            Some(1),
            Some(&json!(quantity)),
        ).await.map_err(|e| ErrorBank::ErrorAction(e))
    }

    async fn deposit_item(
        &self,
        mut char: &mut CharacterData,
        item_code: &str,
        quantity: Option<u32>,
    ) -> Result<(), ErrorBank> {
        if !self.can_store_item(item_code) {
            error!("Cannot store item {} in bank", item_code);
            return Err(ErrorBank::NotEnoughPlaceInBank);
        }

        let items_in_inventory = char.inventory
            .iter()
            .find(|item| item.code == item_code);

        if items_in_inventory.is_none() {
            error!("Item {} not found in inventory", item_code);
            return Err(ErrorBank::NotInInventory);
        }

        let quantity =
            if quantity.is_some() && items_in_inventory.unwrap().quantity >= quantity.unwrap() {
                quantity.unwrap()
            } else {
                items_in_inventory.unwrap().quantity
            };

        if quantity == 0 {
            info!("No item {} to deposit", item_code);
            return Ok(());
        }

        movement::move_to(&self.game_info, &mut char, movement::Place::Bank).await;

        let item = Item {
            slot: None,
            code: item_code.to_string(),
            quantity,
        };
        handle_action_with_cooldown(
            &self.game_info,
            Action::BankDeposit,
            &mut char,
            Some(1),
            Some(&json!(item)),
        ).await.map_err(|e| ErrorBank::ErrorAction(e))
    }

    pub async fn deposit_all_items_and_gold(
        &self,
        mut char: &mut CharacterData,
    ) -> Result<(), ErrorBank> {
        self.deposit_gold(&mut char, None).await?;

        if char.get_inventory_count() == 0 {
            info!("No items to deposit");
            return Ok(());
        }

        movement::move_to(&self.game_info, &mut char, movement::Place::Bank).await;

        for item in char.inventory.iter().clone() {
            if item.quantity > 0 {
                info!("Depositing item: {:?}", item);
                self.deposit_item(&mut char, &item.code, None).await?;
            }
        }

        info!("Deposited all items");
    }

    pub async fn withdraw_item(
        &self,
        mut char: &mut CharacterData,
        item_code: &str,
        qtt: u32,
    ) -> Result<(), ErrorBank> {
        let item_is_in_bank = self.content
            .iter()
            .find(|bank_item| bank_item.code == item_code);

        if item_is_in_bank.is_none() {
            error!("Item {} not found in bank", item_code);
            return Err(ErrorBank::ItemNotFoundInBank);
        }

        if item_is_in_bank.unwrap().quantity < qtt {
            error!("Not enough item {} in bank", item_code);
            return Err(ErrorBank::NotEnoughItemInBank);
        }

        movement::move_to(&self.game_info, &mut char, movement::Place::Bank).await;

        let item = Some(&json!(Item {
            slot: None,
            code: item_code.to_string(),
            quantity: qtt,
        }));

        handle_action_with_cooldown(
            &self.game_info,
            Action::BankWithdraw,
            &mut char,
            Some(1),
            item,
        ).await.map_err(|e| ErrorBank::ErrorAction(e))
    }

    pub async fn withdraw_gold(
        &self,
        mut char: &mut CharacterData,
        quantity: u32,
    ) -> Result<(), ErrorBank> {
        if self.info.gold < quantity {
            error!("Not enough gold in bank");
            return Err(ErrorBank::NotEnoughGoldInBank);
        }

        movement::move_to(&self.game_info, &mut char, movement::Place::Bank).await;

        handle_action_with_cooldown(
            &self.game_info,
            Action::BankWithdrawGold,
            &mut char,
            Some(1),
            Some(&json!(quantity)),
        ).await.map_err(|e| ErrorBank::ErrorAction(e))
    }
}


#[derive(Deserialize, Debug)]
struct BankPage {
    pub data: Vec<Item>,
    pub pages: usize,
}

async fn get_all_items_in_bank(game_info: &Arc<GameInfo>) -> Result<Vec<Item>, ErrorBank> {
    let mut page = 1;
    let mut all_data = Vec::new();

    // Collect all map data from the API
    loop {
        let mut params = HashMap::new();
        params.insert("size", "100");
        let p = page.to_string();
        params.insert("page", &*p);

        let request = game_info.server
            .create_request(GET, "my/bank/items".to_string(), None, Some(params));

        let response = send_request_with_exponential_backoff(&request)
            .await
            .map_err(|e| ErrorBank::ErrorAction(ErrorAction::ErrorRequest(e)))?;

        let bank_items: BankPage = response.json().await
            .map_err(|_| ErrorBank::ErrorAction(ErrorAction::ErrorParsingResponse))?;

        // Collect all data
        all_data.extend(bank_items.data);

        // Check if we've reached the last page
        if page == bank_items.pages {
            break;
        }

        // Move to the next page
        page += 1;
    }
    Ok(all_data)
}

#[derive(Debug, Deserialize)]
struct BankInfoResponse {
    data: BankInfo,
}

async fn get_bank_details(game_info: &Arc<GameInfo>) -> Result<BankInfo, ErrorBank> {
    let request = game_info.server
        .create_request(GET, "my/bank".to_string(), None, None);

    let response = send_request_with_exponential_backoff(&request)
        .await
        .map_err(|e| ErrorBank::ErrorAction(ErrorAction::ErrorRequest(e)))?;

    let bank_infos: BankInfoResponse = response.json().await
        .map_err(|_| ErrorBank::ErrorAction(ErrorAction::ErrorParsingResponse))?;

    Ok(bank_infos.data)
}
