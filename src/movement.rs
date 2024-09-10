use crate::action::{handle_action_with_cooldown, Action, AllActionResponse};
use crate::character::CharacterData;
use crate::gameinfo::map::Position;
use crate::gameinfo::GameInfo;
use crate::server::Server;
use serde_json::json;
use std::sync::Arc;

pub enum Place {
    Exact(Position),
    Bank,
    Resource(GatherType),
    Fight,
    Crafting(GatherType),
}

#[derive(Debug, Clone, Copy)]
pub enum GatherType {
    Wood,
    Fish,
    Mine,
}

impl GatherType {
    fn to_string(&self) -> String {
        match self {
            GatherType::Wood => "woodcutting".to_string(),
            GatherType::Fish => "fishing".to_string(),
            GatherType::Mine => "mining".to_string(),
        }
    }
}

pub async fn move_to(server: &Server, character: &CharacterData, place: Place, game_info: &Arc<GameInfo>) -> Option<AllActionResponse> {
    let current_position = Position {
        x: character.x,
        y: character.y,
    };

    let target_position = get_target_position(place, game_info, &current_position, character);

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
            character,
            Some(1),
            Some(&movement_action),
        )
            .await,
    )
}

fn get_target_position(place: Place, game_info: &Arc<GameInfo>, current_position: &Position, character: &CharacterData) -> Position {
    match place {
        Place::Exact(position) => Some(position),
        Place::Bank => {
            handle_bank(game_info, current_position)
        }
        Place::Resource(type_resource) => {
            handle_resource(game_info, current_position, type_resource, character)
        }
        Place::Fight => {
            handle_fight(game_info, current_position, character)
        }
        Place::Crafting(type_resource) => {
            handle_crafting(game_info, current_position, type_resource, character)
        }
    }.expect("Expected at least one valid position")
}

fn handle_bank(game_info: &Arc<GameInfo>, current_position: &Position) -> Option<Position> {
    let all_banks: Vec<Position> = game_info.map.bank.values().flat_map(|p| p.clone()).collect();
    find_closest_position(&all_banks, current_position)
}

fn handle_resource(game_info: &Arc<GameInfo>, current_position: &Position, type_resource: GatherType, character: &CharacterData)
                   -> Option<Position> {
    let char_lvl = match type_resource {
        GatherType::Wood => character.woodcutting_level,
        GatherType::Fish => character.fishing_level,
        GatherType::Mine => character.mining_level,
    };

    let resource = game_info.resources
        .iter()
        .filter(|&resource|
            resource.skill.eq(&type_resource.to_string()) &&
                resource.level <= char_lvl)
        .max_by_key(|&resource| resource.level)
        .expect("No resource found");

    find_closest_position(
        &game_info.map.resource.get(&resource.code).expect("Resource doesn't exists"),
        current_position,
    )
}

// todo
fn handle_crafting(game_info: &Arc<GameInfo>, current_position: &Position, type_resource: GatherType, character: &CharacterData)
                   -> Option<Position> {
    find_closest_position(
        &game_info.map.workshop.get(&type_resource.to_string()).expect("Workshop doesn't exists"),
        current_position,
    )
}

// todo : improve this function
fn handle_fight(game_info: &Arc<GameInfo>, current_position: &Position, character: &CharacterData)
                -> Option<Position> {
    let monster = game_info.monsters
        .iter()
        .filter(|&monster| monster.level <= character.level)
        .min_by_key(|&resource| resource.level)
        .expect("No resource found");

    find_closest_position(
        &game_info.map.monster.get(&monster.code).expect("Monster doesn't exists"),
        current_position,
    )
}

fn find_closest_position(
    positions_map: &Vec<Position>,
    current_position: &Position,
) -> Option<Position> {
    positions_map.iter()
        .min_by(|pos1, pos2| {
            pos1.distance(&current_position)
                .partial_cmp(&pos2.distance(&current_position))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .cloned()
}
