use tokio::time::{self, Duration};
use chrono::Local;

pub async fn handle_cooldown(char: &str, action: &str, cooldown: f32) {
    println!(
        "[{} - {}] Wait for {}: {}s",
        Local::now().format("%Y-%m-%d %H:%M:%S"),
        char,
        action,
        cooldown
    );
    time::sleep(Duration::from_secs_f32(cooldown)).await;
}
