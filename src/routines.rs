use crate::action::{handle_action_with_cooldown, Action};
use crate::character::CharacterData;
use crate::gameinfo::GameInfo;
use crate::movement::GatherType;
use crate::server::Server;
use crate::{bank, movement};
use std::sync::Arc;

pub async fn action_for_char(character: CharacterData, server_clone: Arc<Server>, game_info: Arc<GameInfo>) {
    let mut character = character;

    // if events
    // handle_events(&server_clone, &character).await;

    loop {
        // wait cooldown if any
        // let cooldown_expiration = &character.cooldown_expiration;
        // handle_cooldown_expiration(&character.name, "cooldown remaining", cooldown_expiration).await;

        // move to bank
        if let Some(updated_char) =
            movement::move_to(&server_clone, &character, movement::Place::Bank, &game_info).await
        {
            character = updated_char.character;
        }

        // deposit all items
        if let Some(updated_char) =
            bank::deposit_all(&server_clone, &mut character).await
        {
            character = updated_char.character;
        }

        // get the max item the character can hold
        let max_item = 99; // character.inventory_max_items;

        // move to resource
        if let Some(updated_char) =
            movement::move_to(
                &server_clone,
                &character,
                movement::Place::Resource(GatherType::Mine),
                &game_info)
                .await
        {
            character = updated_char.character;
        }

        // gather resource
        // if character.name.eq("dim") {
        //     handle_action_with_cooldown(&server_clone, Action::Fight, &character.name, 300, None).await;
        //     continue;
        // }
        handle_action_with_cooldown(&server_clone, Action::Gathering, &character.name, max_item, None).await;
    }
}

