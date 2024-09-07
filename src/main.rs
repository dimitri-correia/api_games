mod action;
mod server;
mod task;
mod character;
mod bank;
mod movement;
mod utils;
mod routines;
mod events;
mod gameinfo;

use crate::gameinfo::get_game_info;
use std::error::Error;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let server = Arc::new(server::create_server());

    let characters = character::get_all_chars_infos(&server).await;
    let game_info = get_game_info(&server).await;

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
        handle.await.unwrap_or(());
    }

    Ok(())
}
