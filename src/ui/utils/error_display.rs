use libadwaita::gtk;
use libadwaita::prelude::*;
use gtk::{Box, Orientation, Label, Image};

pub fn create_error_display(icon_name: &str, title: &str, message: &str) -> Box {
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