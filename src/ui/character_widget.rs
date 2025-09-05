
use libadwaita::gtk::{self, gdk_pixbuf, glib, prelude::*, Box, Image, Label, Orientation};

use crate::models::character::Character;
use async_channel;
use std::io::Cursor;

pub struct CharacterWidget {
    pub widget: Box,
}

impl CharacterWidget {
    pub fn new(character: Character) -> Self {
        // Create the main container with fixed size
        let widget = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(10)
            .margin_top(15)
            .margin_bottom(15)
            .margin_start(15)
            .margin_end(15)
            .width_request(200)
            .height_request(350)
            .build();

        // Create image widget with placeholder
        let image = Image::builder()
            .icon_name("image-x-generic") // Default placeholder from Adwaita
            .pixel_size(48)
            .halign(gtk::Align::Center)
            .valign(gtk::Align::Start)
            .build();
        
        // Set a fixed size for the image area
        image.set_size_request(180, 270);
        
        // Handle image loading if URL is available
        if let Some(image_url) = &character.images.jpg.image_url {
            let image_url = image_url.clone();
            let (sender, receiver) = async_channel::unbounded::<Vec<u8>>();
            
            // Spawn background thread for image loading
            std::thread::spawn(move || {
                match reqwest::blocking::get(&image_url) {
                    Ok(response) => {
                        if let Ok(bytes) = response.bytes() {
                            // Send bytes to main thread
                            let _ = sender.send_blocking(bytes.to_vec());
                        }
                    }
                    Err(_) => {
                        // Send empty vector to indicate error
                        let _ = sender.send_blocking(Vec::new());
                    }
                }
            });
            
            // Handle image loading on main thread
            let image_clone = image.clone();
            glib::MainContext::default().spawn_local(async move {
                match receiver.recv().await {
                    Ok(bytes) => {
                        if !bytes.is_empty() {
                            // Create pixbuf from bytes using Cursor
                            let cursor = Cursor::new(bytes);
                            match gdk_pixbuf::Pixbuf::from_read(cursor) {
                                Ok(pixbuf) => {
                                    // Scale the pixbuf to fit within our constraints
                                    let current_width = pixbuf.width();
                                    let current_height = pixbuf.height();
                                    
                                    // Calculate new dimensions maintaining aspect ratio
                                    let max_width = 180;
                                    let max_height = 270;
                                    
                                    let scale_factor = f64::min(
                                        max_width as f64 / current_width as f64,
                                        max_height as f64 / current_height as f64
                                    );
                                    
                                    let new_width = (current_width as f64 * scale_factor) as i32;
                                    let new_height = (current_height as f64 * scale_factor) as i32;
                                    
                                    if let Some(scaled_pixbuf) = pixbuf.scale_simple(
                                        new_width.max(1), 
                                        new_height.max(1), 
                                        gdk_pixbuf::InterpType::Bilinear
                                    ) {
                                        image_clone.set_from_pixbuf(Some(&scaled_pixbuf));
                                    } else {
                                        image_clone.set_from_pixbuf(Some(&pixbuf));
                                    }
                                }
                                Err(_) => {
                                    // Use Adwaita's image-missing icon for load errors
                                    image_clone.set_icon_name(Some("image-missing"));
                                }
                            }
                        } else {
                            // Error occurred during image loading, use network-offline icon
                            image_clone.set_icon_name(Some("network-offline-symbolic"));
                        }
                    }
                    Err(_) => {
                        // Channel error, use image-missing icon
                        image_clone.set_icon_name(Some("image-missing"));
                    }
                }
            });
        } else {
            // No image URL, set a placeholder using Adwaita's image-missing icon
            image.set_icon_name(Some("image-missing"));
        }
        
        // Process character name with proper truncation
        let display_name = if character.name.chars().count() > 25 {
            let truncated: String = character.name.chars().take(22).collect();
            format!("{}...", truncated)
        } else {
            character.name.clone()
        };
        
        // Create name label with proper sizing
        let name_label = Label::builder()
            .label(&display_name)
            .wrap(true)
            .justify(gtk::Justification::Center)
            .lines(2)
            .ellipsize(gtk::pango::EllipsizeMode::End)
            .halign(gtk::Align::Center)
            .valign(gtk::Align::Center)
            .width_request(180)
            .height_request(40)
            .build();

        // Add widgets to container
        widget.append(&image);
        widget.append(&name_label);

        Self { widget }
    }
}