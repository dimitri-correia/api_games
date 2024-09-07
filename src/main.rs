mod action;
mod server;
mod task;
mod map;
mod char;
mod bank;
mod movement;
mod utils;

use crate::action::{handle_action_with_cooldown, Action};
use crate::map::{generate_map, Map};
use crate::server::{create_server, Server};
use std::error::Error;
use std::sync::Arc;
use crate::utils::handle_cooldown;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let server = Arc::new(create_server());

    let map = Arc::new(generate_map(Arc::clone(&server)).await);

    let chars = vec!["dim", "dim2", "dim3", "dim4", "dim5"];

    let mut handles = Vec::new();

    for char in &chars {
        let server = Arc::clone(&server);
        let map = Arc::clone(&map);
        let char = char.to_string();

        let handle = tokio::spawn(async move {
            action_for_char(&char, server, map).await;
        });

        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        handle.await.unwrap();
    }

    Ok(())
}


async fn action_for_char(char: &str, server_clone: Arc<Server>, map: Arc<Result<Map, Box<dyn Error>>>) {
    let map = &*map;
    loop {
        // wait cooldown if any
        let cooldown = char::get_char_infos(&server_clone, char).await.unwrap().cooldown as f32;
        handle_cooldown(char, "cooldown remaining", cooldown).await;

        // move to bank
        movement::move_to(&server_clone, char, movement::Place::Bank, map).await;

        // deposit all items
        bank::deposit_all(&server_clone, char).await;

        // get the max item the char can hold
        let max_item = char::get_char_infos(&server_clone, char).await.unwrap().inventory_max_items;

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

