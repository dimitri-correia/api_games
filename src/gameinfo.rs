use crate::server::creation::{create_server, Server};
use crate::server::status::get_status;
use std::sync::Arc;

pub mod items;
pub mod map;
pub mod monster;
pub mod resources;

#[derive(Debug, Clone)]
pub struct GameInfo {
    pub monsters: Vec<monster::Monster>,
    pub items: Vec<items::ItemInfo>,
    pub resources: Vec<resources::Resource>,
    pub map: map::Map,
    pub server: Server,
    pub max_level: u32,
}

pub async fn get_game_info() -> Arc<GameInfo> {
    let server = create_server();

    let monsters = monster::get_all_monsters(&server).await;
    let items = items::get_all_items(&server).await;
    let resources = resources::get_all_resources(&server).await;
    let map = map::generate_map(&server).await;
    let max_level = get_status(&server).await.max_level;

    Arc::new(GameInfo {
        monsters,
        items,
        resources,
        map,
        server,
        max_level,
    })
}
