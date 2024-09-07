use crate::action::handle_action_with_cooldown;
use crate::server::Server;
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::char::get_char_infos;

#[derive(Debug, Serialize, Deserialize)]
struct DepositItem {
    code: String,
    quantity: u32,
}

async fn deposit_item(server: &Server, char: &str, item_code: &str, quantity: u32) {
    let item_data = DepositItem {
        code: item_code.to_string(),
        quantity,
    };
    handle_action_with_cooldown(server, crate::action::Action::BankDeposit, char, 1, Some(&json!(item_data))).await.unwrap()
}


pub async fn deposit_all(server: &Server, char: &str) {
    for i in get_char_infos(server, char).await.unwrap().inventory {
        println!("Depositing item: {:?}", i);
    }
    println!("Depositing all items");
}