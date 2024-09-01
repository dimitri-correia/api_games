mod action;
mod server;
mod task;
mod map;

use crate::action::{handle_action, Action};
use crate::server::create_server;
use serde_json::json;
use std::error::Error;
use crate::map::generate_map;
use crate::task::{handle_task, Task};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let server = create_server();

    let map = generate_map(&server).await?;

    // Send the request for the movement action
    // if false {
    //     let goto = &json!({
    //         "x": -1,
    //         "y": 0
    //     });
    //     handle_action_with_json(&server, Action::Move, "dim", goto).await?;
    // }

    // Send the request for the fight action
    if true {
        handle_action(&server, Action::Fight, "dim", 300, None).await?;
    }
    //
    // // Send the request for the gathering action
    // if false {
    //     handle_action(&server, Action::Gathering, "dim", 10, None).await?;
    // }
    //
    // // Send the request to unequip
    // if false {
    //     let unequip = &json!({
    //         "slot": "weapon"
    //     });
    //     handle_action_with_json(&server, Action::Unequip, "dim", unequip).await?;
    // }
    //
    // // Send the request to craft
    // if false {
    //     let craft = &json!({
    //         "code": "wooden_staff"
    //     });
    //     handle_action_with_json(&server, Action::Craft, "dim", craft).await?;
    // }
    //
    // // Send the request to equip
    // if false {
    //     let equip = &json!({
    //         "slot": "weapon",
    //         "code": "wooden_staff"
    //     });
    //     handle_action_with_json(&server, Action::Equip, "dim", equip).await?;
    // }
    //
    // // Send get task action
    // if false {
    //     handle_task(&server, "dim", Task::Complete).await?;
    // }

    Ok(())
}

