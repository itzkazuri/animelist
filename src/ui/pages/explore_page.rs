use libadwaita::gtk;
use gtk::prelude::*;
use gtk::{ScrolledWindow, FlowBox, SelectionMode, Box, Orientation, Button, Align, Entry, Spinner};

#[derive(Clone)]
pub struct ExplorePage {
    pub container: ScrolledWindow,
    pub character_container: FlowBox,
    pub fetch_button: Button,
    pub search_entry: Entry,
    pub search_button: Button,
    pub loading_spinner: Spinner,
}

impl ExplorePage {
    pub fn new() -> Self {
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
            .label("Search Waifu Random")
            .halign(Align::Center)
            .margin_top(15)
            .margin_bottom(15)
            .build();

        let loading_spinner = Spinner::builder()
            .halign(Align::Center)
            .valign(Align::Center)
            .width_request(48)
            .height_request(48)
            .margin_top(20)
            .margin_bottom(20)
            .visible(false)
            .build();

        let character_container = FlowBox::builder()
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

        let search_page_box = Box::builder()
            .orientation(Orientation::Vertical)
            .vexpand(true)
            .build();
        
        search_page_box.append(&search_box);
        search_page_box.append(&fetch_button);
        search_page_box.append(&loading_spinner);
        search_page_box.append(&character_container);

        let container = ScrolledWindow::builder()
            .vexpand(true)
            .hexpand(true)
            .child(&search_page_box)
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
}
