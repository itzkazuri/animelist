use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterImages {
    pub jpg: CharacterImageJpg,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterImageJpg {
    #[serde(rename = "image_url")]
    pub image_url: Option<String>,
    
    #[serde(rename = "small_image_url")]
    pub small_image_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    #[serde(rename = "mal_id")]
    pub mal_id: u32,
    
    #[serde(rename = "url")]
    pub url: String,
    
    #[serde(rename = "images")]
    pub images: CharacterImages,
    
    #[serde(rename = "name")]
    pub name: String,
    
    #[serde(rename = "name_kanji")]
    pub name_kanji: Option<String>,
    
    #[serde(rename = "nicknames")]
    pub nicknames: Vec<String>,
    
    #[serde(rename = "favorites")]
    pub favorites: u32,
    
    #[serde(rename = "about")]
    pub about: Option<String>,
}