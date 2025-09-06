use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use serde_json;
use tokio::task;

use crate::models::character::Character;

const FAVORITES_FILE: &str = "favorites.json";

pub struct FavoritesStorage {
    file_path: PathBuf,
}

impl FavoritesStorage {
    pub fn new() -> Self {
        let file_path = if let Some(mut config_dir) = dirs::config_dir() {
            config_dir.push("waifu-viewer");
            fs::create_dir_all(&config_dir).unwrap();
            config_dir.join(FAVORITES_FILE)
        } else {
            PathBuf::from(FAVORITES_FILE)
        };
        
        Self { file_path }
    }

    pub fn get_favorites(&self) -> Result<Vec<Character>, String> {
        if !self.file_path.exists() {
            return Ok(Vec::new());
        }

        let mut file = File::open(&self.file_path).map_err(|e| e.to_string())?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(|e| e.to_string())?;

        if contents.is_empty() {
            return Ok(Vec::new());
        }

        serde_json::from_str(&contents).map_err(|e| e.to_string())
    }

    pub async fn add_favorite(&self, character: Character) -> Result<(), String> {
        let file_path = self.file_path.clone();
        task::spawn_blocking(move || {
            let mut favorites = Self::load_favorites_sync(&file_path)?;
            
            if !favorites.iter().any(|c| c.mal_id == character.mal_id) {
                favorites.push(character);
                Self::save_favorites_sync(&file_path, &favorites)?;
            }
            
            Ok::<(), String>(())
        }).await.map_err(|e| e.to_string())?.map_err(|e: String| e)
    }

    pub async fn remove_favorite(&self, character: Character) -> Result<(), String> {
        let file_path = self.file_path.clone();
        task::spawn_blocking(move || {
            let mut favorites = Self::load_favorites_sync(&file_path)?;
            
            favorites.retain(|c| c.mal_id != character.mal_id);
            
            Self::save_favorites_sync(&file_path, &favorites)
        }).await.map_err(|e| e.to_string())?.map_err(|e: String| e)
    }

    fn load_favorites_sync(file_path: &PathBuf) -> Result<Vec<Character>, String> {
        if !file_path.exists() {
            return Ok(Vec::new());
        }

        let mut file = File::open(file_path).map_err(|e| e.to_string())?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(|e| e.to_string())?;

        if contents.is_empty() {
            return Ok(Vec::new());
        }

        serde_json::from_str(&contents).map_err(|e| e.to_string())
    }

    fn save_favorites_sync(file_path: &PathBuf, favorites: &[Character]) -> Result<(), String> {
        let json = serde_json::to_string_pretty(favorites).map_err(|e| e.to_string())?;
        let mut file = File::create(file_path).map_err(|e| e.to_string())?;
        file.write_all(json.as_bytes()).map_err(|e| e.to_string())?;
        Ok(())
    }
}
