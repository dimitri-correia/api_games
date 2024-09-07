use crate::action::{handle_action_with_cooldown, Action};
use crate::character::CharacterData;
use crate::server::Server;
use crate::GameInfo;
use std::sync::Arc;

pub async fn action_for_char(character: CharacterData, server_clone: Arc<Server>, game_info: Arc<GameInfo>) {
    let mut character = character;

    loop {
        // wait cooldown if any
        // let cooldown_expiration = &character.cooldown_expiration;
        // handle_cooldown_expiration(&character.name, "cooldown remaining", cooldown_expiration).await;

        // move to bank
        // movement::move_to(&server_clone, &character, movement::Place::Bank, map).await;

        // deposit all items
        // bank::deposit_all(&server_clone, &character).await;

        // get the max item the character can hold
        let max_item = character.inventory_max_items;

        // move to resource
        // movement::move_to(&server_clone, &character, movement::Place::Resource, map).await;

        // gather resource
        if character.name.eq("dim") {
            handle_action_with_cooldown(&server_clone, Action::Fight, &character.name, 300, None).await;
            continue;
        }
        handle_action_with_cooldown(&server_clone, Action::Gathering, &character.name, max_item, None).await;
    }
}

