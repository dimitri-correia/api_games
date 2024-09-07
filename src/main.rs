use std::sync::Arc;

mod action;
mod server;
mod task;
mod map;
mod char;
mod bank;
mod movement;

use crate::action::{handle_action, Action};
use crate::map::{generate_map, Map};
use crate::server::{create_server, Server};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let server = Arc::new(create_server());

    let map = Arc::new(generate_map(Arc::clone(&server)).await?);

    // chars to handle tasks
    let chars = vec!["dim", "dim2", "dim3", "dim4", "dim5"];

    // Spawn tasks for each dimension with Action::Fight and value 300
    let mut handles = Vec::new();

    for &char in &chars {
        let handle = tokio::spawn(async move {
            action_for_char(char, &server.clone(), Arc::clone(&map)).await;
        });
        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        handle.await.unwrap();
    }

    Ok(())
}

async fn action_for_char(char: &str, server_clone: &Server, map: Arc<Map>) {
    let map = &*map;
    loop {
        // move to bank
        movement::move_to(&server_clone, char, movement::Place::Bank, map).await;

        // deposit all items
        bank::deposit_all(&server_clone, char).await;

        // get the max item the char can hold
        let max_item = char::get_char_max_items(&server_clone, char).await.unwrap();

        // move to resource
        movement::move_to(&server_clone, char, movement::Place::Resource, map).await;

        // gather resource
        handle_action(&server_clone, Action::Gathering, char, max_item, None).await.unwrap();
    }
}

