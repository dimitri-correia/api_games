mod action;
mod server;
mod task;
mod map;
mod char;

use crate::action::{handle_action, Action};
use crate::map::generate_map;
use crate::server::{create_server, Server};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let server = create_server();

    let map = generate_map(&server).await?;

    // chars to handle tasks
    let chars = vec!["dim", "dim2", "dim3", "dim4", "dim5"];

    // Spawn tasks for each dimension with Action::Fight and value 300
    let mut handles = Vec::new();

    for &char in &chars {
        let server_clone = server.clone();
        let handle = tokio::spawn(async move {
            action_for_char(char, &server_clone).await;
        });
        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        handle.await.unwrap();
    }

    Ok(())
}

async fn action_for_char(char: &str, server_clone: &Server) {
    if char=="dim" {
        if let Err(e) = handle_action(&server_clone, Action::Fight, char, 300, None).await {
            eprintln!("Error handling action for dimension {}: {}", char, e);
        }
        return;
    }
    if let Err(e) = handle_action(&server_clone, Action::Gathering, char, 300, None).await {
        eprintln!("Error handling action for dimension {}: {}", char, e);
    }
}

