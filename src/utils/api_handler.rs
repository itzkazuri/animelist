use crate::api::jikan::{JikanClient, JikanError};
use crate::models::character::Character;

pub struct ApiHandler {
    jikan_client: JikanClient,
}

impl ApiHandler {
    pub fn new() -> Self {
        Self {
            jikan_client: JikanClient::new(),
        }
    }

    pub async fn get_top_characters(&self) -> Result<Vec<Character>, JikanError> {
        self.jikan_client.get_top_characters().await
    }

    pub async fn search_characters(&self, query: &str) -> Result<Vec<Character>, JikanError> {
        self.jikan_client.search_characters(query).await
    }

    
}