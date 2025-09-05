use libadwaita as adw;
use adw::prelude::*;
use libadwaita::gtk;
use gtk::{Box, Orientation, FlowBox, Label, Spinner, Image};

use crate::ui::headerbar::WaifuHeaderBar;
use crate::ui::content::WaifuContent;
use crate::api::jikan::JikanClient;
use crate::ui::character_widget::CharacterWidget;
use gtk::glib;

pub struct WaifuWindow {
    pub window: adw::ApplicationWindow,
}

impl WaifuWindow {
    pub fn new(app: &adw::Application) -> Self {
        let header_bar = WaifuHeaderBar::new();
        let content = WaifuContent::new();

        // Create a main vertical box for the entire window content
        let main_box = Box::builder()
            .orientation(Orientation::Vertical)
            .vexpand(true)
            .build();

        main_box.append(header_bar.container());
        main_box.append(content.container());

        let window = adw::ApplicationWindow::builder()
            .application(app)
            .title("Waifu Viewer")
            .default_width(1000)
            .default_height(800)
            .content(&main_box)
            .build();

        // Connect the about functionality
        let window_clone = window.clone();
        header_bar.connect_about(move || {
            Self::show_about_dialog(&window_clone);
        });
        
        // Connect the content search
        let character_container = content.get_character_container();
        let loading_spinner = content.get_loading_spinner();
        content.connect_search(move |query| {
            let container = character_container.clone();
            let spinner = loading_spinner.clone();
            let ctx = glib::MainContext::default();
            ctx.spawn_local(async move {
                // Show loading spinner
                spinner.set_visible(true);
                spinner.start();
                container.set_visible(false);
                
                // Clear existing children
                while let Some(child) = container.first_child() {
                    container.remove(&child);
                }
                
                Self::search_characters(container, spinner, &query).await;
            });
        });
        
        // Connect the fetch button from content
        let character_container = content.get_character_container();
        let loading_spinner = content.get_loading_spinner();
        Self::connect_fetch_button(content.get_fetch_button(), character_container, loading_spinner);

        Self {
            window,
        }
    }

    fn connect_fetch_button(fetch_button: gtk::Button, character_container: FlowBox, loading_spinner: Spinner) {
        fetch_button.connect_clicked(move |_| {
            let ctx = glib::MainContext::default();
            let character_container = character_container.clone();
            let spinner = loading_spinner.clone();
            ctx.spawn_local(async move {
                // Show loading spinner
                spinner.set_visible(true);
                spinner.start();
                character_container.set_visible(false);
                
                // Clear existing children
                while let Some(child) = character_container.first_child() {
                    character_container.remove(&child);
                }
                
                Self::fetch_and_display_top_characters(character_container, spinner).await;
            });
        });
    }

    async fn fetch_and_display_top_characters(container: FlowBox, loading_spinner: Spinner) {
        // Add loading label
        let loading_label = Label::builder()
            .label("Loading top waifus...")
            .build();
        container.insert(&loading_label, -1);

        let client = JikanClient::new();

        match client.get_top_characters().await {
            Ok(characters) => {
                // Remove loading label
                container.remove(&loading_label);
                
                // Add character widgets
                for character in characters.iter().take(20) {
                    let character_widget = CharacterWidget::new(character.clone());
                    container.insert(&character_widget.widget, -1);
                }
            }
            Err(e) => {
                // Remove loading label
                container.remove(&loading_label);
                
                // Check if it's a network error
                if e.is_connect() || e.is_timeout() {
                    // Show network error with icon
                    let error_box = Self::create_error_display(
                        "network-offline-symbolic",
                        "No Internet Connection",
                        "Please check your internet connection and try again."
                    );
                    container.insert(&error_box, -1);
                } else {
                    // Show generic error message
                    let error_label = Label::builder()
                        .label(&format!("Error fetching characters: {}", e))
                        .build();
                    container.insert(&error_label, -1);
                }
            }
        }
        
        // Hide loading spinner and show container
        loading_spinner.set_visible(false);
        loading_spinner.stop();
        container.set_visible(true);
    }
    
    async fn search_characters(container: FlowBox, loading_spinner: Spinner, query: &str) {
        // Add loading label
        let loading_label = Label::builder()
            .label(&format!("Searching for \"{}\"…", query))
            .build();
        container.insert(&loading_label, -1);

        let client = JikanClient::new();

        match client.search_characters(query).await {
            Ok(characters) => {
                // Remove loading label
                container.remove(&loading_label);
                
                if characters.is_empty() {
                    let no_results_label = Label::builder()
                        .label(&format!("No characters found for \"{}\"", query))
                        .build();
                    container.insert(&no_results_label, -1);
                } else {
                    // Add character widgets
                    for character in characters.iter().take(20) {
                        let character_widget = CharacterWidget::new(character.clone());
                        container.insert(&character_widget.widget, -1);
                    }
                }
            }
            Err(e) => {
                // Remove loading label
                container.remove(&loading_label);
                
                // Check if it's a network error
                if e.is_connect() || e.is_timeout() {
                    // Show network error with icon
                    let error_box = Self::create_error_display(
                        "network-offline-symbolic",
                        "No Internet Connection",
                        "Please check your internet connection and try again."
                    );
                    container.insert(&error_box, -1);
                } else {
                    // Show generic error message
                    let error_label = Label::builder()
                        .label(&format!("Error searching characters: {}", e))
                        .build();
                    container.insert(&error_label, -1);
                }
            }
        }
        
        // Hide loading spinner and show container
        loading_spinner.set_visible(false);
        loading_spinner.stop();
        container.set_visible(true);
    }
    
    fn create_error_display(icon_name: &str, title: &str, message: &str) -> Box {
        let error_box = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(10)
            .halign(gtk::Align::Center)
            .valign(gtk::Align::Center)
            .margin_top(40)
            .margin_bottom(40)
            .build();
            
        let error_icon = Image::builder()
            .icon_name(icon_name)
            .pixel_size(64)
            .build();
            
        let title_label = Label::builder()
            .label(title)
            .css_classes(vec!["title-2".to_string()])
            .build();
            
        let message_label = Label::builder()
            .label(message)
            .css_classes(vec!["body".to_string()])
            .build();
            
        error_box.append(&error_icon);
        error_box.append(&title_label);
        error_box.append(&message_label);
        
        error_box
    }

    
    fn show_about_dialog(window: &adw::ApplicationWindow) {
        // Create the about dialog using GTK AboutDialog instead of libadwaita AboutWindow
        let dialog = gtk::AboutDialog::builder()
            .transient_for(window)
            .modal(true)
            .program_name("Waifu Viewer")
            .version("1.0.0")
            .comments("A simple application to view and search anime characters (waifus)")
            .website("https://github.com/itzkazuri")
            .authors(vec!["itzkazuri".to_string()])
            .copyright("© 2025 itzkazuri. This application is testing for purpose.")
            .license_type(gtk::License::MitX11)
            .logo_icon_name("face-heart-symbolic")
            .build();
            
        dialog.present();
    }
}