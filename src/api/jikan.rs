use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;

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



// Custom error type for better error handling
#[derive(Debug)]
pub enum JikanError {
    Network(reqwest::Error),
    JsonParsing(serde_json::Error),
}

impl std::fmt::Display for JikanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JikanError::Network(e) => write!(f, "Network error: {}", e),
            JikanError::JsonParsing(e) => write!(f, "JSON parsing error: {}", e),
        }
    }
}

impl std::error::Error for JikanError {}

impl From<reqwest::Error> for JikanError {
    fn from(error: reqwest::Error) -> Self {
        JikanError::Network(error)
    }
}

impl From<serde_json::Error> for JikanError {
    fn from(error: serde_json::Error) -> Self {
        JikanError::JsonParsing(error)
    }
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

    pub async fn get_top_characters(&self) -> Result<Vec<Character>, JikanError> {
        let url = format!("{}/top/characters", self.base_url);
        println!("Making request to: {}", url);
        
        let response = self.client.get(&url).send().await?;
        println!("Response status: {}", response.status());
        
        // Log response body for debugging
        let text = response.text().await?;
        println!("Response body length: {}", text.len());
        // Only log first 1000 characters to avoid overwhelming output
        if text.len() > 1000 {
            println!("First 1000 chars of response body: {}", &text[..1000]);
        } else {
            println!("Response body: {}", text);
        }
        
        // Try to parse JSON from text
        let jikan_response: JikanResponse = serde_json::from_str(&text)?;
        Ok(jikan_response.data)
    }
    
    pub async fn search_characters(&self, query: &str) -> Result<Vec<Character>, JikanError> {
        let url = format!("{}/characters?q={}", self.base_url, query);
        println!("Making request to: {}", url);
        
        let response = self.client.get(&url).send().await?;
        println!("Response status: {}", response.status());
        
        // Log response body for debugging
        let text = response.text().await?;
        println!("Response body length: {}", text.len());
        // Only log first 1000 characters to avoid overwhelming output
        if text.len() > 1000 {
            println!("First 1000 chars of response body: {}", &text[..1000]);
        } else {
            println!("Response body: {}", text);
        }
        
        // Try to parse JSON from text
        let jikan_response: JikanResponse = serde_json::from_str(&text)?;
        Ok(jikan_response.data)
    }
    
    
}