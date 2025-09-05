mod api;
mod models;
mod ui;

use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Button, Orientation, ScrolledWindow, Entry, FlowBox, SelectionMode};
use api::jikan::JikanClient;
use ui::character_widget::CharacterWidget;
use glib;

#[tokio::main]
async fn main() {
    // Initialize GTK
    gtk::init().expect("Failed to initialize GTK");

    // Create a new application
    let app = Application::builder()
        .application_id("com.example.WaifuViewer")
        .build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    // Create a window and set the application
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Waifu Viewer")
        .default_width(900)
        .default_height(700)
        .build();

    // Load CSS
    load_css();

    // Create a vertical box container
    let vbox = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(10)
        .build();

    // Create a search entry
    let search_entry = Entry::builder()
        .placeholder_text("Search for waifus...")
        .margin_start(12)
        .margin_end(12)
        .margin_top(12)
        .build();

    // Add CSS class to search entry
    search_entry.style_context().add_class("search-entry");

    // Create a horizontal box for buttons
    let button_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(10)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Create a button to fetch top waifus
    let fetch_top_button = Button::builder()
        .label("Fetch Top Waifus")
        .build();

    // Add CSS class to button
    fetch_top_button.style_context().add_class("fetch-button");

    // Create a button to search waifus
    let search_button = Button::builder()
        .label("Search")
        .build();

    // Add suggested-action class for GNOME look
    search_button.style_context().add_class("suggested-action");

    // Add buttons to button box
    button_box.append(&fetch_top_button);
    button_box.append(&search_button);

    // Create a scrolled window for the character list
    let scrolled_window = ScrolledWindow::builder()
        .vexpand(true)
        .margin_start(12)
        .margin_end(12)
        .margin_bottom(12)
        .build();

    // Create a container for character widgets
    let character_container = FlowBox::builder()
        .selection_mode(SelectionMode::None)
        .halign(gtk::Align::Start)
        .valign(gtk::Align::Start)
        .build();

    // Add character container to scrolled window
    scrolled_window.set_child(Some(&character_container));

    // Add widgets to the vertical box
    vbox.append(&search_entry);
    vbox.append(&button_box);
    vbox.append(&scrolled_window);

    // Clone references for closures
    let character_container_clone = character_container.clone();
    
    // Connect to "clicked" signal of fetch top button
    fetch_top_button.connect_clicked(move |_| {
        let ctx = glib::MainContext::default();
        let character_container = character_container_clone.clone();
        
        ctx.spawn_local(async move {
            fetch_and_display_top_characters(character_container).await;
        });
    });

    // Clone references for search button
    let character_container_clone2 = character_container.clone();
    let search_entry_clone = search_entry.clone();
    
    // Connect to "clicked" signal of search button
    search_button.connect_clicked(move |_| {
        let query = search_entry_clone.text().to_string();
        if !query.is_empty() {
            let ctx = glib::MainContext::default();
            let character_container = character_container_clone2.clone();
            
            ctx.spawn_local(async move {
                fetch_and_display_searched_characters(character_container, &query).await;
            });
        }
    });

    // Connect to "activate" signal of search entry (Enter key)
    let character_container_clone3 = character_container.clone();
    search_entry.connect_activate(move |entry| {
        let query = entry.text().to_string();
        if !query.is_empty() {
            let ctx = glib::MainContext::default();
            let character_container = character_container_clone3.clone();
            
            ctx.spawn_local(async move {
                fetch_and_display_searched_characters(character_container, &query).await;
            });
        }
    });

    // Add the vertical box to the window
    window.set_child(Some(&vbox));

    // Present window
    window.present();
}

fn load_css() {
    let provider = gtk::CssProvider::new();
    // Load CSS from file
    if let Some(display) = gdk4::Display::default() {
        gtk::StyleContext::add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
        
        // Load CSS from string for now
        provider.load_from_data(include_str!("../assets/style.css"));
    }
}

async fn fetch_and_display_top_characters(container: gtk::FlowBox) {
    // Clear existing children
    while let Some(child) = container.first_child() {
        container.remove(&child);
    }
    
    // Show loading message
    let loading_label = gtk::Label::builder()
        .label("Loading top waifus...")
        .build();
    container.insert(&loading_label, -1);
    
    // Force UI update
    loading_label.show();
    
    // Create Jikan client
    let client = JikanClient::new();
    
    // Fetch top characters
    match client.get_top_characters().await {
        Ok(characters) => {
            // Remove loading message
            while let Some(child) = container.first_child() {
                container.remove(&child);
            }
            
            // Display characters
            for character in characters.iter().take(20) {
                let character_widget = CharacterWidget::new(character.clone());
                container.insert(&character_widget.widget, -1);
            }
        }
        Err(e) => {
            // Remove loading message
            while let Some(child) = container.first_child() {
                container.remove(&child);
            }
            
            let error_label = gtk::Label::builder()
                .label(&format!("Error fetching characters: {}", e))
                .build();
            container.insert(&error_label, -1);
        }
    }
}

async fn fetch_and_display_searched_characters(container: gtk::FlowBox, query: &str) {
    // Clear existing children
    while let Some(child) = container.first_child() {
        container.remove(&child);
    }
    
    // Show loading message
    let loading_label = gtk::Label::builder()
        .label(&format!("Searching for \"{}\"…", query))
        .build();
    container.insert(&loading_label, -1);
    
    // Force UI update
    loading_label.show();
    
    // Create Jikan client
    let client = JikanClient::new();
    
    // Fetch searched characters
    match client.search_characters(query).await {
        Ok(characters) => {
            // Remove loading message
            while let Some(child) = container.first_child() {
                container.remove(&child);
            }
            
            // Display characters
            if characters.is_empty() {
                let no_results_label = gtk::Label::builder()
                    .label(&format!("No characters found for \"{}\"", query))
                    .build();
                container.insert(&no_results_label, -1);
            } else {
                for character in characters.iter().take(20) {
                    let character_widget = CharacterWidget::new(character.clone());
                    container.insert(&character_widget.widget, -1);
                }
            }
        }
        Err(e) => {
            // Remove loading message
            while let Some(child) = container.first_child() {
                container.remove(&child);
            }
            
            let error_label = gtk::Label::builder()
                .label(&format!("Error searching characters: {}", e))
                .build();
            container.insert(&error_label, -1);
        }
    }
}
