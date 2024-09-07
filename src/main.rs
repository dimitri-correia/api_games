mod action;
mod server;
mod task;
mod map;
mod character;
mod bank;
mod movement;
mod utils;
mod routines;
mod monster;
mod items;
mod resources;

use crate::character::get_all_chars_infos;
use crate::server::create_server;
use std::error::Error;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let server = Arc::new(create_server());

    let characters = get_all_chars_infos(&server).await;
    let monsters = Arc::new(monster::get_all_monsters(&server).await);
    let items = Arc::new(items::get_all_items(&server).await);
    let resources = Arc::new(resources::get_all_resources(&server).await);
    let map = Arc::new(map::generate_map(&server).await);

    let mut handles = Vec::new();

    for char in characters {
        let server = Arc::clone(&server);
        let monsters = Arc::clone(&monsters);
        let items = Arc::clone(&items);
        let resources = Arc::clone(&resources);
        let map = Arc::clone(&map);

        let handle = tokio::spawn(async move {
            routines::action_for_char(char, server, monsters, items, resources, map).await;
        });

        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        handle.await.unwrap();
    }

    Ok(())
}



