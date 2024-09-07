use std::error::Error;
use std::sync::Arc;
use crate::{bank, character, movement};
use crate::action::{handle_action_with_cooldown, Action};
use crate::map::Map;
use crate::server::Server;
use crate::utils::handle_cooldown;

pub async fn action_for_char(char: &str, server_clone: Arc<Server>, map: Arc<Result<Map, Box<dyn Error>>>) {
    let map = &*map;
    loop {
        // wait cooldown if any
        let cooldown = character::get_char_infos(&server_clone, char).await.unwrap().cooldown as f32;
        handle_cooldown(char, "cooldown remaining", cooldown).await;

        // move to bank
        movement::move_to(&server_clone, char, movement::Place::Bank, map).await;

        // deposit all items
        bank::deposit_all(&server_clone, char).await;

        // get the max item the char can hold
        let max_item = character::get_char_infos(&server_clone, char).await.unwrap().inventory_max_items;

        // move to resource
        movement::move_to(&server_clone, char, movement::Place::Resource, map).await;

        // gather resource
        if char.eq("dim") {
            handle_action_with_cooldown(&server_clone, Action::Fight, char, 300, None).await.unwrap();
            continue;
        }
        handle_action_with_cooldown(&server_clone, Action::Gathering, char, 100, None).await.unwrap();
    }
}
