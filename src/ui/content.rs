use libadwaita::gtk;
use libadwaita::prelude::*;

use gtk::{ScrolledWindow, FlowBox, SelectionMode, Box, Orientation, Button, Align, Entry, Spinner};

pub struct WaifuContent {
    container: ScrolledWindow,
    character_container: FlowBox,
    fetch_button: Button,
    search_entry: Entry,
    search_button: Button,
    loading_spinner: Spinner,
}

impl WaifuContent {
    pub fn new() -> Self {
        // Create search components
        let search_entry = Entry::builder()
            .placeholder_text("Search for waifus...")
            .hexpand(true)
            .build();

        let search_button = Button::builder()
            .icon_name("system-search-symbolic")
            .tooltip_text("Search")
            .build();

        let search_box = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(10)
            .margin_start(20)
            .margin_end(20)
            .margin_top(20)
            .margin_bottom(10)
            .build();
        
        search_box.append(&search_entry);
        search_box.append(&search_button);

        let fetch_button = Button::builder()
            .label("Fetch Top Waifus")
            .halign(Align::Center)
            .margin_top(15)
            .margin_bottom(15)
            .build();

        // Create loading spinner
        let loading_spinner = Spinner::builder()
            .halign(Align::Center)
            .valign(Align::Center)
            .width_request(48)
            .height_request(48)
            .margin_top(20)
            .margin_bottom(20)
            .visible(false) // Initially hidden
            .build();

        // Improved grid layout for waifu list
        let character_container = FlowBox::builder()
            .selection_mode(SelectionMode::None)
            .halign(gtk::Align::Fill)
            .valign(gtk::Align::Start)
            .homogeneous(false) // Set to false to allow flexible sizing
            .column_spacing(20)
            .row_spacing(20)
            .margin_start(20)
            .margin_end(20)
            .margin_bottom(20)
            .min_children_per_line(1)
            .max_children_per_line(10)
            .vexpand(true) // Allow the container to expand vertically
            .build();

        let main_box = Box::builder()
            .orientation(Orientation::Vertical)
            .vexpand(true) // Allow the main box to expand vertically
            .build();
        
        main_box.append(&search_box);
        main_box.append(&fetch_button);
        main_box.append(&loading_spinner); // Add spinner before character container
        main_box.append(&character_container);

        let container = ScrolledWindow::builder()
            .vexpand(true)
            .hexpand(true)
            .propagate_natural_height(true)
            .propagate_natural_width(true)
            .child(&main_box)
            .build();

        Self {
            container,
            character_container,
            fetch_button,
            search_entry,
            search_button,
            loading_spinner,
        }
    }

    pub fn container(&self) -> &ScrolledWindow {
        &self.container
    }

    pub fn get_character_container(&self) -> FlowBox {
        self.character_container.clone()
    }

    pub fn get_fetch_button(&self) -> Button {
        self.fetch_button.clone()
    }
    
    pub fn get_search_entry(&self) -> Entry {
        self.search_entry.clone()
    }
    
    pub fn get_search_button(&self) -> Button {
        self.search_button.clone()
    }
    
    pub fn get_loading_spinner(&self) -> Spinner {
        self.loading_spinner.clone()
    }
    
    pub fn show_loading(&self) {
        // Clear any existing children in the container
        while let Some(child) = self.character_container.first_child() {
            self.character_container.remove(&child);
        }
        
        self.loading_spinner.set_visible(true);
        self.loading_spinner.start();
        self.character_container.set_visible(false);
    }
    
    pub fn hide_loading(&self) {
        self.loading_spinner.set_visible(false);
        self.loading_spinner.stop();
        self.character_container.set_visible(true);
    }
    
    pub fn connect_search<F>(&self, callback: F) 
    where F: Fn(String) + Clone + 'static {
        let search_entry_clone = self.search_entry.clone();
        let search_button_clone = self.search_button.clone();
        let callback_clone = callback.clone();
        
        // Connect search button
        search_button_clone.connect_clicked(move |_| {
            let query = search_entry_clone.text().to_string();
            if !query.is_empty() {
                callback_clone(query);
            }
        });
        
        // Connect Enter key on search entry
        let search_entry_clone2 = self.search_entry.clone();
        search_entry_clone2.connect_activate(move |entry| {
            let query = entry.text().to_string();
            if !query.is_empty() {
                callback(query);
            }
        });
    }
}