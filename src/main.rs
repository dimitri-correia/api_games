mod action;
mod server;
mod task;
mod map;
mod character;
mod bank;
mod movement;
mod utils;
mod routines;

use crate::character::get_all_chars_infos;
use crate::map::generate_map;
use crate::server::create_server;
use std::error::Error;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let server = Arc::new(create_server());

    let characters = get_all_chars_infos(&server).await;

    let mut handles = Vec::new();

    for char in characters {
        let server = Arc::clone(&server);

        let handle = tokio::spawn(async move {
            routines::action_for_char(char, server).await;
        });

        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        handle.await.unwrap();
    }

    Ok(())
}



