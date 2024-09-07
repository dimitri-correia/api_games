use crate::server::Server;
use std::sync::Arc;

pub mod map;
pub mod resources;
pub mod items;
pub mod monster;

#[derive(Debug, Clone)]
pub struct GameInfo {
    pub monsters: Vec<monster::Monster>,
    pub items: Vec<items::Item>,
    pub resources: Vec<resources::Resource>,
    pub map: map::Map,
}


pub async fn get_game_info(server: &Arc<Server>) -> Arc<GameInfo> {
    let monsters = monster::get_all_monsters(&server).await;
    let items = items::get_all_items(&server).await;
    let resources = resources::get_all_resources(&server).await;
    let map = map::generate_map(&server).await;

    let game_info = Arc::new(GameInfo {
        monsters,
        items,
        resources,
        map,
    });
    game_info
}