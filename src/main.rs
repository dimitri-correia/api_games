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
mod responsecode;

use std::error::Error;
use std::sync::Arc;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    info!("Starting the bot");

    let server = Arc::new(server::create_server());

    let characters = character::get_all_chars_infos(&server).await;
    let game_info = gameinfo::get_game_info(&server).await;

    let mut handles = Vec::new();

    for char in characters {
        let server = Arc::clone(&server);
        let game_info = Arc::clone(&game_info);

        let handle = tokio::spawn(async move {
            let name = char.name.clone();
            utils::info(&name, "Starting routine");
            routines::action_for_char(char, server, game_info).await;
            utils::info(&name, "Routine has ended");
        });

        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        handle.await.unwrap_or(());
    }

    Ok(())
}
