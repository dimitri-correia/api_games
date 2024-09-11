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
    setup_logs();

    info!("Starting the bot");

    let game_info = gameinfo::get_game_info().await;
    let characters = character::get_all_chars_infos(&game_info.server).await;

    let mut handles = Vec::new();

    for mut char in characters.into_iter() {
        let game_info_clone = Arc::clone(&game_info);

        let handle = tokio::spawn(async move {
            let name = char.name.clone();
            utils::info(&name, "Starting routine");

            routines::action_for_char(game_info_clone, &mut char).await;

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

fn setup_logs() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");
}
