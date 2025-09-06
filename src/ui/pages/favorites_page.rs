use libadwaita::gtk;
use gtk::prelude::*;
use gtk::{ScrolledWindow, FlowBox, SelectionMode, Align, Image, Box, Orientation};

use crate::storage::favorites::FavoritesStorage;
use crate::ui::character_widget::CharacterWidget;

#[derive(Clone)]
pub struct FavoritesPage {
    pub container: ScrolledWindow,
    pub favorites_container: FlowBox,
}

impl FavoritesPage {
    pub fn new() -> Self {
        let favorites_container = FlowBox::builder()
            .selection_mode(SelectionMode::None)
            .halign(gtk::Align::Fill)
            .valign(gtk::Align::Start)
            .homogeneous(false)
            .column_spacing(20)
            .row_spacing(20)
            .margin_start(20)
            .margin_end(20)
            .margin_bottom(20)
            .min_children_per_line(1)
            .max_children_per_line(10)
            .vexpand(true)
            .build();

        let container = ScrolledWindow::builder()
            .vexpand(true)
            .hexpand(true)
            .child(&favorites_container)
            .build();

        Self {
            container,
            favorites_container,
        }
    }

    pub fn load_favorites(&self) {
        // Clear existing favorites
        while let Some(child) = self.favorites_container.first_child() {
            self.favorites_container.remove(&child);
        }

        let storage = FavoritesStorage::new();
        match storage.get_favorites() {
            Ok(favorites) => {
                if favorites.is_empty() {
                    // Create a container for the "no favorites" message
                    let no_favorites_box = Box::builder()
                        .orientation(Orientation::Vertical)
                        .spacing(20)
                        .halign(Align::Center)
                        .valign(Align::Center)
                        .vexpand(true)
                        .build();
                    
                    // Create heart icon
                    let heart_icon = Image::builder()
                        .icon_name("ibuki")
                        .pixel_size(64)
                        .build();
                    
                    // Create bold label
                    let label = gtk::Label::builder()
                        .label("You have no favorite waifus yet.")
                        .halign(Align::Center)
                        .valign(Align::Center)
                        .css_classes(vec!["heading".to_string()])
                        .build();
                    
                    no_favorites_box.append(&heart_icon);
                    no_favorites_box.append(&label);
                    self.favorites_container.insert(&no_favorites_box, -1);
                } else {
                    for character in favorites {
                        let character_widget = CharacterWidget::new_with_delete_callback(character, {
                            let clone = self.clone();
                            move || {
                                clone.load_favorites();
                            }
                        });
                        self.favorites_container.insert(&character_widget.widget, -1);
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to load favorites: {}", e);
                let label = gtk::Label::builder()
                    .label("Failed to load favorites.")
                    .halign(Align::Center)
                    .valign(Align::Center)
                    .build();
                self.favorites_container.insert(&label, -1);
            }
        }
    }
}