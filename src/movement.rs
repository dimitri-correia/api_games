use crate::action::{handle_action_with_cooldown, AllActionResponse};
use crate::map::Map;
use crate::server::Server;
use serde_json::{json, Value};

pub enum Place {
    Bank,
    Resource,
}

pub async fn move_to(server: &Server, char: &str, place: Place, map: &Map) -> AllActionResponse {
    let goto = get_pos(place, map);
    handle_action_with_cooldown(server, crate::action::Action::Move, char, 1, Some(&goto)).await
}

fn get_pos(place: Place, map: &Map) -> Value {
    match place {
        Place::Bank => {
            let pos = map.bank.iter().next().unwrap().1.iter().next().unwrap();
            json!({
            "x": pos.x,
            "y": pos.y,
        })
        }
        Place::Resource => {
            let pos = map.resource.iter().next().unwrap().1.iter().next().unwrap();
            json!({
            "x": pos.x,
            "y": pos.y,
        })
        }
    }
}