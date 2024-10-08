use crate::gameinfo::GameInfo;
use crate::server::creation::RequestMethod::POST;
use crate::server::RequestMethod::POST;
use crate::server::Server;
use crate::utils;
use std::error::Error;

pub enum Task {
    Complete,
    New,
    Cancel,
    Exchange,
}

fn get_task_name(task: Task) -> &'static str {
    match task {
        Task::Complete => "complete",
        Task::New => "new",
        Task::Cancel => "cancel",
        Task::Exchange => "exchange",
    }
}
pub async fn handle_task(
    game_info: &GameInfo,
    char: &str,
    task: Task,
) -> Result<(), Box<dyn Error>> {
    let response = game_info
        .server
        .create_request(
            POST,
            format!("my/{}/action/task/{}", char, get_task_name(task)),
            None,
            None,
        )
        .send()
        .await?;

    info(
        char,
        format!("Task response: {}", response.text().await?).as_str(),
    );

    Ok(())
}
