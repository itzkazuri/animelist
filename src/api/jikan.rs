use reqwest;
use serde::{Deserialize, Serialize};

use crate::models::character::Character;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JikanPagination {
    pub last_visible_page: u32,
    pub has_next_page: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JikanResponse {
    pub pagination: Option<JikanPagination>,
    pub data: Vec<Character>,
}

pub struct JikanClient {
    client: reqwest::Client,
    base_url: String,
}

impl JikanClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: "https://api.jikan.moe/v4".to_string(),
        }
    }

    pub async fn get_top_characters(&self) -> Result<Vec<Character>, reqwest::Error> {
        let url = format!("{}/top/characters", self.base_url);
        let response = self.client.get(&url).send().await?;
        let jikan_response: JikanResponse = response.json().await?;
        Ok(jikan_response.data)
    }
    
    pub async fn search_characters(&self, query: &str) -> Result<Vec<Character>, reqwest::Error> {
        let url = format!("{}/characters?q={}", self.base_url, query);
        let response = self.client.get(&url).send().await?;
        let jikan_response: JikanResponse = response.json().await?;
        Ok(jikan_response.data)
    }
}