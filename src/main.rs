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
mod events;

use std::error::Error;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let server = Arc::new(server::create_server());

    let characters = character::get_all_chars_infos(&server).await;

    let monsters = monster::get_all_monsters(&server).await;
    let items = items::get_all_items(&server).await;
    let resources = resources::get_all_resources(&server).await;
    let map = map::generate_map(&server).await;

    let game_info = Arc::new(GameInfo {
        monsters,
        items,
        resources,
        map,
    });

    let mut handles = Vec::new();

    for char in characters {
        let server = Arc::clone(&server);
        let game_info = Arc::clone(&game_info);

        let handle = tokio::spawn(async move {
            routines::action_for_char(char, server, game_info).await;
        });

        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        handle.await.unwrap();
    }

    Ok(())
}

#[derive(Debug, Clone)]
pub struct GameInfo {
    pub monsters: Vec<monster::Monster>,
    pub items: Vec<items::Item>,
    pub resources: Vec<resources::Resource>,
    pub map: map::Map,
}
