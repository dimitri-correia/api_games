use crate::gameinfo::GameInfo;
use crate::server::creation::RequestMethod::GET;
use crate::server::creation::Server;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use tracing::info;

#[derive(Debug, Deserialize)]
struct ApiResponse {
    data: Data,
}

#[derive(Debug, Deserialize)]
struct Data {
    status: String,
    version: String,
    pub max_level: u32,
    characters_online: u32,
    server_time: DateTime<Utc>,
    announcements: Vec<Announcement>,
    // last_wipe: String,
    // next_wipe: String,
}

#[derive(Debug, Deserialize)]
struct Announcement {
    message: String,
    created_at: DateTime<Utc>,
}

pub async fn get_status(server: &Server) -> Data {
    let response = server
        .create_request(GET, "".to_string(), None, None)
        .send()
        .await
        .expect("Error sending request")
        .json()
        .await
        .expect("Error parsing JSON");

    info!(
        "Server status: {}, version {}, characters online: {}, max level: {}",
            response.data.status,
            response.data.version,
            response.data.characters_online,
            response.data.max_level
    );
    for announcement in response.data.announcements.iter() {
        info!("Announcement: {}",announcement.message);
    }
    info!("Server time: {}", response.data.server_time);

    response.data
}