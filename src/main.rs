mod action;
mod server;
mod task;
mod character;
mod bank;
mod movement;
mod routines;
mod events;
mod gameinfo;

use std::error::Error;
use std::sync::Arc;
use tracing::{event, info, span, Level};
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

            // Create a span for the thread
            let span = span!(Level::INFO, "", %name);
            // Enter the span, which adds context to logs
            let _enter = span.enter();

            info!("Starting routine");

            routines::action_for_char(game_info_clone, &mut char).await;

            info!("Routine has ended");
        });

        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        handle.await.unwrap_or(());
    }

    Ok(())
}

pub fn setup_logs() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");
}