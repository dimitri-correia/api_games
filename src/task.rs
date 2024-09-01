use crate::server::Server;
use std::error::Error;

pub enum Task {
    Complete,
    New,
    Cancel,
    Exchange,
}

pub fn get_task_name(task: Task) -> &'static str {
    match task {
        Task::Complete => "complete",
        Task::New => "new",
        Task::Cancel => "cancel",
        Task::Exchange => "exchange",
    }
}
pub async fn handle_task(server: &Server, char: &str, task: Task) -> Result<(), Box<dyn Error>> {
    let response = server.client
        .post(format!("https://api.artifactsmmo.com/my/{}/action/task/{}", char, get_task_name(task)))
        .headers(server.headers.clone())
        .send()
        .await?;

    println!("Task response: {}", response.text().await?);

    Ok(())
}