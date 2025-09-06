// src/ui/handlers.rs
use libadwaita as adw;
use adw::prelude::*;
use libadwaita::gtk;
use gtk::{FlowBox, Spinner, Label, Button};
use gtk::glib;

use crate::ui::pages::explore_page::ExplorePage;
use crate::ui::character_widget::CharacterWidget;
use crate::ui::utils::api_handler::ApiHandler;
use crate::ui::utils::error_display;

use crate::models::character::Character;

pub struct SearchHandler;

impl SearchHandler {
    pub fn connect_search_signals(explore_page: &ExplorePage) {
        let character_container = explore_page.character_container.clone();
        let loading_spinner = explore_page.loading_spinner.clone();
        
        // Connect search functionality
        let search_entry = explore_page.search_entry.clone();
        let search_button = explore_page.search_button.clone();
        let container = character_container.clone();
        let spinner = loading_spinner.clone();
        
        let search_callback = move |query: String| {
            let container = container.clone();
            let spinner = spinner.clone();
            let ctx = glib::MainContext::default();
            ctx.spawn_local(async move {
                Self::prepare_loading_state(&container, &spinner);
                Self::search_characters(container, spinner, &query).await;
            });
        };

        search_button.connect_clicked({
            let search_entry = search_entry.clone();
            let search_callback = search_callback.clone();
            move |_| {
                let query = search_entry.text().to_string();
                if !query.is_empty() {
                    search_callback(query);
                }
            }
        });

        search_entry.connect_activate({
            let search_callback = search_callback.clone();
            move |entry| {
                let query = entry.text().to_string();
                if !query.is_empty() {
                    search_callback(query);
                }
            }
        });


        // Connect fetch button functionality
        Self::connect_fetch_button(
            explore_page.fetch_button.clone(),
            character_container,
            loading_spinner,
        );
    }

    fn connect_fetch_button(
        fetch_button: Button,
        character_container: FlowBox,
        loading_spinner: Spinner,
    ) {
        fetch_button.connect_clicked(move |_| {
            let ctx = glib::MainContext::default();
            let container = character_container.clone();
            let spinner = loading_spinner.clone();
            ctx.spawn_local(async move {
                Self::prepare_loading_state(&container, &spinner);
                Self::fetch_and_display_top_characters(container, spinner).await;
            });
        });
    }

    fn prepare_loading_state(container: &FlowBox, spinner: &Spinner) {
        spinner.set_visible(true);
        spinner.start();
        container.set_visible(false);
        
        // Clear existing children
        while let Some(child) = container.first_child() {
            container.remove(&child);
        }
    }

    fn finish_loading_state(container: &FlowBox, spinner: &Spinner) {
        spinner.set_visible(false);
        spinner.stop();
        container.set_visible(true);
    }

    pub async fn fetch_and_display_top_characters(container: FlowBox, loading_spinner: Spinner) {
        let loading_label = Label::builder()
            .label("Loading top waifus...")
            .build();
        container.insert(&loading_label, -1);

        let api_handler = ApiHandler::new();

        match api_handler.get_top_characters().await {
            Ok(characters) => {
                container.remove(&loading_label);
                Self::add_character_widgets(&container, &characters, 20).await;
            }
            Err(e) => {
                container.remove(&loading_label);
                Self::handle_error(&container, &e, "Error fetching characters");
            }
        }
        
        Self::finish_loading_state(&container, &loading_spinner);
    }
    
    async fn search_characters(container: FlowBox, loading_spinner: Spinner, query: &str) {
        let loading_label = Label::builder()
            .label(&format!("Searching for \"{}\"...", query))
            .build();
        container.insert(&loading_label, -1);

        let api_handler = ApiHandler::new();

        match api_handler.search_characters(query).await {
            Ok(characters) => {
                container.remove(&loading_label);
                
                if characters.is_empty() {
                    let no_results_label = Label::builder()
                        .label(&format!("No characters found for \"{}\"", query))
                        .build();
                    container.insert(&no_results_label, -1);
                } else {
                    Self::add_character_widgets(&container, &characters, 20).await;
                }
            }
            Err(e) => {
                container.remove(&loading_label);
                Self::handle_error(&container, &e, "Error searching characters");
            }
        }
        
        Self::finish_loading_state(&container, &loading_spinner);
    }

    

    async fn add_character_widgets(
        container: &FlowBox,
        characters: &[Character],
        limit: usize,
    ) {
        for character in characters.iter().take(limit) {
            let character_widget = CharacterWidget::new(character.clone());
            container.insert(&character_widget.widget, -1);
        }
    }

    fn handle_error(container: &FlowBox, error: &crate::api::jikan::JikanError, context: &str) {
        match error {
            crate::api::jikan::JikanError::Network(req_err) if req_err.is_connect() || req_err.is_timeout() => {
                let error_box = error_display::create_error_display(
                    "network-offline-symbolic",
                    "No Internet Connection",
                    "Please check your internet connection and try again."
                );
                container.insert(&error_box, -1);
            }
            _ => {
                let error_label = Label::builder()
                    .label(&format!("{}: {}", context, error))
                    .build();
                container.insert(&error_label, -1);
            }
        }
    }
}