mod action;
mod server;
mod task;
mod map;
mod character;
mod bank;
mod movement;
mod utils;
mod routines;

use crate::action::{handle_action_with_cooldown, Action};
use crate::map::{generate_map, Map};
use crate::server::{create_server, Server};
use std::error::Error;
use std::sync::Arc;
use crate::routines::action_for_char;
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



