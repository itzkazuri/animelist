use gtk4::{prelude::*, gio, Box, Label, Orientation, Image};
use crate::models::character::Character;
use gdk_pixbuf::Pixbuf;
use glib::{self, clone};
use std::sync::mpsc;

pub struct CharacterWidget {
    pub widget: Box,
}

impl CharacterWidget {
    pub fn new(character: Character) -> Self {
        let container = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(10)
            .margin_start(10)
            .margin_end(10)
            .margin_top(10)
            .margin_bottom(10)
            .build();

        // Create image widget
        let image = Image::builder()
            .width_request(100)
            .height_request(140)
            .build();

        // Set a default placeholder image
        image.set_icon_name(Some("image-missing"));

        // Load image asynchronously
        if let Some(image_url) = character.images.jpg.image_url.clone() {
            let image_clone = image.clone();
            let ctx = glib::MainContext::default();
            ctx.spawn_local(clone!(@strong image_clone, @strong image_url => async move {
                // Load the image in the background
                let handle = tokio::task::spawn(async move {
                    match reqwest::get(&image_url).await {
                        Ok(response) => response.bytes().await.map_err(|_| "Failed to get image bytes"),
                        Err(_) => Err("Failed to fetch image"),
                    }
                });

                // Wait for the image to load
                match handle.await {
                    Ok(Ok(image_data)) => {
                        let image_data = glib::Bytes::from(&image_data);
                        let stream = gio::MemoryInputStream::from_bytes(&image_data);
                        Pixbuf::from_stream_async(
                            &stream,
                            None::<&gio::Cancellable>,
                            clone!(@strong image_clone => move |result| {
                                match result {
                                    Ok(pixbuf) => {
                                        if let Some(scaled_pixbuf) = pixbuf.scale_simple(100, 140, gdk_pixbuf::InterpType::Bilinear) {
                                            image_clone.set_from_pixbuf(Some(&scaled_pixbuf));
                                        } else {
                                            image_clone.set_icon_name(Some("image-missing"));
                                        }
                                    }
                                    Err(_) => {
                                        image_clone.set_icon_name(Some("image-missing"));
                                    }
                                }
                            }),
                        );
                    }
                    _ => {
                        image_clone.set_icon_name(Some("image-missing"));
                    }
                }
            }));
        }

        // Create info container
        let info_container = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(5)
            .hexpand(true)
            .build();

        // Create character name label
        let name_label = Label::builder()
            .label(&format!("<b>{}</b>", character.name))
            .use_markup(true)
            .halign(gtk4::Align::Start)
            .build();

        // Create favorites label
        let favorites_label = Label::builder()
            .label(&format!("❤️ {} favorites", character.favorites))
            .halign(gtk4::Align::Start)
            .build();

        // Add widgets to info container
        info_container.append(&name_label);
        info_container.append(&favorites_label);

        // Add widgets to container
        container.append(&image);
        container.append(&info_container);

        // Add CSS class for styling
        let style_context = container.style_context();
        style_context.add_class("character-widget");

        Self {
            widget: container,
        }
    }
}