use crate::action::{handle_action_with_cooldown, Action};
use crate::map::Map;
use crate::server::Server;
use crate::utils::handle_cooldown;
use crate::{bank, character, movement};
use std::sync::Arc;
use crate::character::CharacterData;

pub async fn action_for_char(character: CharacterData, server_clone: Arc<Server>, map: Arc<Map>) {
    let map = &*map;
    loop {
        // wait cooldown if any
        let cooldown = character.cooldown as f32;
        handle_cooldown(&character.name, "cooldown remaining", cooldown).await;

        // move to bank
        movement::move_to(&server_clone, &character.name, movement::Place::Bank, map).await;

        // deposit all items
        bank::deposit_all(&server_clone, &character.name).await;

        // get the max item the character can hold
        let max_item = character::get_char_infos(&server_clone, &character.name).await.inventory_max_items;

        // move to resource
        movement::move_to(&server_clone, &character.name, movement::Place::Resource, map).await;

        // gather resource
        if character.name.eq("dim") {
            handle_action_with_cooldown(&server_clone, Action::Fight, &character.name, 300, None).await;
            continue;
        }
        handle_action_with_cooldown(&server_clone, Action::Gathering, &character.name, 100, None).await;
    }
}
