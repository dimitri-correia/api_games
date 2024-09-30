use crate::action::bank::Bank;
use crate::action::movement;
use crate::character::CharacterData;
use crate::events::get_all_maps_with_events;
use crate::gameinfo::map::Position;
use crate::gameinfo::GameInfo;
use std::sync::Arc;

pub async fn action_for_char(game_info: Arc<GameInfo>, mut character: &mut CharacterData) {
    let bank = Bank::new(&game_info).await.expect("Failed to create bank");

    let bank_deposit = bank.deposit_all_items_and_gold(&mut character).await;

    if bank_deposit.is_err() {
        // todo!("Failed to deposit items and gold");
        println!("Failed to deposit items and gold");
    }
    handle_event(&game_info, &mut character).await;

    movement::move_to(
        &character,
        movement::Place::Exact(Position { x: 1, y: -1 }),
        game_info)
        .await
}

async fn handle_event(game_info: &Arc<GameInfo>, char: &mut CharacterData) {
    todo!()
    let mut events = get_all_maps_with_events(&game_info.server).await;
}
//     // loop {
//     //     if let Some(updated_char) =
//     //         movement::move_to(&server_clone, &character, movement::Place::Bank, &game_info).await
//     //     {
//     //         character = updated_char.character;
//     //     }
//     //     if let Some(updated_char) =
//     //         bank::deposit_all(&server_clone, &mut character).await
//     //     {
//     //         character = updated_char.character;
//     //     }
//     //     if let Some(updated_char) =
//     //         movement::move_to(
//     //             &server_clone,
//     //             &character,
//     //             movement::Place::Exact(Position { x: 1, y: -1 }),
//     //             &game_info)
//     //             .await
//     //     {
//     //         character = updated_char.character;
//     //     }
//     //     handle_action_with_cooldown(
//     //         &server_clone,
//     //         Action::Fight,
//     //         &character,
//     //         Some(300),
//     //         None,
//     //     ).await;
//     //
//     // }
//
//     let bank_items = get_all_items_in_bank(&server_clone).await;
//
//     if character.name.eq("dim") {
//         tmp_crafting(&mut character, &server_clone, &game_info).await;
//     }
//
//     // if events
//     //
//
//     loop {
//         // wait cooldown if any
//         // let cooldown_expiration = &character.cooldown_expiration;
//         // handle_cooldown_expiration(&character.name, "cooldown remaining", cooldown_expiration).await;
//
//         // move to bank
//         if let Some(updated_char) =
//             movement::move_to(&server_clone, &character, movement::Place::Bank, &game_info).await
//         {
//             character = updated_char.character;
//         }
//
//         // deposit all items
//         if let Some(updated_char) =
//             bank::deposit_all(&server_clone, &character, &game_info).await
//         {
//             character = updated_char.character;
//         }
//
//         // move to resource
//         let gather_type = GatherType::Wood;
//         if let Some(updated_char) =
//             movement::move_to(
//                 &server_clone,
//                 &character,
//                 movement::Place::Resource(gather_type),
//                 // movement::Place::Exact(Position { x: 2, y: 0 }), // todo
//                 //movement::Place::Fight,
//                 &game_info)
//                 .await
//         {
//             character = updated_char.character;
//         }
//
//         // fight
//         // handle_action_with_cooldown(
//         //     &server_clone,
//         //     Action::Fight,
//         //     &character,
//         //     Some(300),
//         //     None,
//         // ).await;
//
//         // gather resource
//         handle_action_with_cooldown(
//             &server_clone,
//             Action::Gathering,
//             &character,
//             Some(character.inventory_max_items),
//             None,
//         ).await;
//
//         // move to crafting
//         if let Some(updated_char) =
//             movement::move_to(
//                 &server_clone,
//                 &character,
//                 movement::Place::Exact(Position { x: -2, y: -3 }), // todo
//                 // movement::Place::Crafting(gather_type),
//                 &game_info)
//                 .await
//         {
//             character = updated_char.character;
//         }
//
//         // transform resource
//         let craft = CraftItem {
//             code: "spruce_plank".to_string(),
//             quantity: character.inventory_max_items / 8,
//         };
//         handle_action_with_cooldown(
//             &server_clone,
//             Action::Craft,
//             &character,
//             None,
//             Some(&serde_json::json!(craft)),
//         ).await;
//     }
// }
//
// async fn tmp_crafting(mut char: &mut CharacterData, server_clone: &Arc<Server>, game_info: &Arc<GameInfo>) {
//     loop {
//         // move to bank
//         // tmp force move
//         let mut char = get_char_infos(&server_clone, &char.name).await;
//
//         movement::move_to(&server_clone, &char, movement::Place::Bank, game_info).await;
//
//         // deposit all items
//         let mut char = get_char_infos(&server_clone, &char.name).await;
//         bank::deposit_all(&server_clone, &mut char, ).await;
//
//         // withdraw item
//         let qtt = char.inventory_max_items;
//         bank::withdraw_item(&server_clone, &mut char, game_info, "ash_wood", qtt).await;
//
//         // tmp force move
//         let mut char =  get_char_infos(&server_clone, &char.name).await;
//
//         // move to crafting
//             movement::move_to(
//                 &server_clone,
//                 &char,
//                 movement::Place::Exact(Position { x: -2, y: -3 }), // todo
//                 // movement::Place::Crafting(gather_type),
//                 game_info)
//                 .await;
//         // transform resource
//         let craft = CraftItem {
//             code: "ash_plank".to_string(),
//             quantity: char.inventory_max_items / 8,
//         };
//         handle_action_with_cooldown(
//             &server_clone,
//             Action::Craft,
//             &char,
//             None,
//             Some(&serde_json::json!(craft)),
//         ).await;
//         // movement::move_to(
//         //     &server_clone,
//         //     &char,
//         //     movement::Place::Exact(Position { x: 1, y: 1 }), // todo
//         //     &game_info)
//         //     .await;
//         //
//         // // transform resource
//         // // compute qtt to be qtt divided by 6 rounded down to integer before
//         // let craft = CraftItem {
//         //     code: "cooked_chicken".to_string(),
//         //     quantity: qtt,
//         // };
//         // handle_action_with_cooldown(
//         //     &server_clone,
//         //     Action::Craft,
//         //     &char,
//         //     Some(1),
//         //     Some(&serde_json::json!(craft)),
//         // ).await;
//         //
//         // let craft = CraftItem {
//         //     code: "cooked_gudgeon".to_string(),
//         //     quantity: qtt_2,
//         // };
//         // handle_action_with_cooldown(
//         //     &server_clone,
//         //     Action::Craft,
//         //     &char,
//         //     Some(1),
//         //     Some(&serde_json::json!(craft)),
//         // ).await;
//
//         // // tmp force move
//         // let mut char = get_char_infos(&server_clone, &char.name).await;
//         //
//         // // move to weapon crafting
//         // movement::move_to(
//         //     &server_clone,
//         //     &char,
//         //     movement::Place::Exact(Position { x: 1, y: 3 }), // todo
//         //     &game_info)
//         //     .await;
//         //
//         // // transform resource
//         // let qtt = qtt/6;
//         // let craft = CraftItem {
//         //     code: "copper_ring".to_string(),
//         //     quantity: qtt,
//         // };
//         // handle_action_with_cooldown(
//         //     &server_clone,
//         //     Action::Craft,
//         //     &char,
//         //     Some(1),
//         //     Some(&serde_json::json!(craft)),
//         // ).await;
//     }
// }
//
