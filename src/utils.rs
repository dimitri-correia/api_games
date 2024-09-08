use tokio::time::{self, Duration};
use tracing::info;

pub async fn handle_cooldown(char: &str, action: &str, cooldown: f32) {
    info(char, &format!("[{}] Wait for {}: {}s", char, action, cooldown));
    time::sleep(Duration::from_secs_f32(cooldown)).await;
}

pub fn info(char_name: &str, log: &str) {
    info!("[{}] {}", &char_name, &log);
}