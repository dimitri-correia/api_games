use crate::action::{handle_action_with_cooldown, Action, AllActionResponse};
use crate::character::CharacterData;
use crate::map::{Map, Position};
use crate::server::Server;
use serde_json::json;
use std::collections::HashMap;

pub enum Place {
    Bank,
    Resource,
}

pub async fn move_to(server: &Server, character: &CharacterData, place: Place, map: &Map) -> Option<AllActionResponse> {
    let current_position = Position {
        x: character.x,
        y: character.y,
    };

    let target_position = get_target_position(place, map, &current_position);

    if current_position == target_position {
        return None;
    }

    let movement_action = json!({
        "x": target_position.x,
        "y": target_position.y,
    });

    Some(
        handle_action_with_cooldown(
            server,
            Action::Move,
            &character.name,
            1,
            Some(&movement_action),
        )
            .await,
    )
}

fn get_target_position(place: Place, map: &Map, current_position: &Position) -> Position {
    match place {
        Place::Bank => find_closest_position(&map.bank, current_position),
        Place::Resource => find_closest_position(&map.resource, current_position),
    }
        .expect("Expected at least one valid position")
}

fn find_closest_position(
    positions_map: &HashMap<String, Vec<Position>>,
    current_position: &Position,
) -> Option<Position> {
    positions_map
        .values()
        .flat_map(|positions| positions.iter())
        .min_by(|pos1, pos2| {
            pos1.distance(&current_position)
                .partial_cmp(&pos2.distance(&current_position))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .cloned()
}
