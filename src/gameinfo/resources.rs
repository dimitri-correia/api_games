use crate::server::RequestMethod::GET;
use crate::server::Server;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
pub struct Drop {
    pub code: String,
    pub rate: f32,
    pub min_quantity: u32,
    pub max_quantity: u32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Resource {
    pub name: String,
    pub code: String,
    pub skill: String,
    pub level: u32,
    pub drops: Vec<Drop>,
}

#[derive(Deserialize, Debug)]
pub struct ResourcePage {
    pub data: Vec<Resource>,
    pub pages: usize,
}

pub async fn get_all_resources(server: &Server) -> Vec<Resource> {
    let mut page = 1;
    let mut all_data = Vec::new();

    // Collect all resource data from the API
    loop {
        let mut params = HashMap::new();
        params.insert("size", "100");
        let p = page.to_string();
        params.insert("page", &*p);

        let response = server.create_request(GET, "resources".to_string(), None, Some(params))
            .send()
            .await.expect("Error sending request");

        let resource_page: ResourcePage = response.json().await.expect("Error parsing JSON");

        // Collect all data
        all_data.extend(resource_page.data);

        // Check if we've reached the last page
        if page == resource_page.pages {
            break;
        }

        // Move to the next page
        page += 1;
    }
    all_data
}
