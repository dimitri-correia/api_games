use crate::server::creation::{create_server, Server};
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
    pub server: Server,
}


pub async fn get_game_info() -> Arc<GameInfo> {
    let server = create_server();

    let monsters = monster::get_all_monsters(&server).await;
    let items = items::get_all_items(&server).await;
    let resources = resources::get_all_resources(&server).await;
    let map = map::generate_map(&server).await;

    Arc::new(GameInfo {
        monsters,
        items,
        resources,
        map,
        server,
    })
}