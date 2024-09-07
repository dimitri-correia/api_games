use std::collections::HashMap;
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
    let pos_char = Position {
        x: char.x,
        y: char.y,
    };
    let pos_to_go = get_pos(place, map, pos_char);
    if char.x == pos_to_go.x && char.y == pos_to_go.y {
        return None;
    }

    let goto = json!({
        "x": pos_to_go.x,
        "y": pos_to_go.y,
    });

    Some(handle_action_with_cooldown(server, Action::Move, &char.name, 1, Some(&goto)).await)
}

fn get_pos(place: Place, map: &Map, pos_char: Position) -> &Position {
    match place {
        Place::Bank => {
            &find_closest_position(&map.bank, pos_char).unwrap()
        }
        Place::Resource => {
            &find_closest_position(&map.resource, pos_char).unwrap()
        }
    }
}

fn find_closest_position(
    map: &HashMap<String, Vec<Position>>,
    pos_char: Position,
) -> Option<Position> {
    map.values()
        .flat_map(|positions| positions.iter())
        .min_by(|pos1, pos2| {
            pos1
                .distance(&pos_char)
                .partial_cmp(&pos2.distance(&pos_char))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .cloned()
}