use crate::action::{handle_action_with_cooldown, Action, AllActionResponse};
use crate::character::CharacterData;
use crate::map::{Map, Position};
use crate::server::Server;
use serde_json::json;

pub enum Place {
    Bank,
    Resource,
}

pub async fn move_to(server: &Server, char: &CharacterData, place: Place, map: &Map) -> Option<AllActionResponse> {
    let pos = get_pos(place, map);
    if char.x == pos.x && char.y == pos.y {
        return None;
    }

    let goto = json!({
        "x": pos.x,
        "y": pos.y,
    });

    Some(handle_action_with_cooldown(server, Action::Move, &char.name, 1, Some(&goto)).await)
}

fn get_pos(place: Place, map: &Map) -> &Position {
    match place {
        Place::Bank => {
            map.bank.iter().next().unwrap().1.iter().next().unwrap()
        }
        Place::Resource => {
            map.resource.iter().next().unwrap().1.iter().next().unwrap()
        }
    }
}