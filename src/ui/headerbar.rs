use libadwaita as adw;
use adw::prelude::*;
use libadwaita::gtk;
use gtk::{Box, Orientation, MenuButton, Popover, Button};



pub struct WaifuHeaderBar {
    container: adw::HeaderBar,
    about_button: Button,
    
}

impl WaifuHeaderBar {
    pub fn new() -> Self {
        // Create the menu button with three dots
        let menu_button = MenuButton::builder()
            .icon_name("open-menu-symbolic")
            .build();

        // Create the popover menu
        let popover = Popover::new();
        let about_button = Button::builder()
            .label("About")
            .build();
        
        let popover_box = Box::builder()
            .orientation(Orientation::Vertical)
            .margin_top(5)
            .margin_bottom(5)
            .margin_start(5)
            .margin_end(5)
            .build();
        
        popover_box.append(&about_button);
        popover.set_child(Some(&popover_box));
        menu_button.set_popover(Some(&popover));

        let container = adw::HeaderBar::builder()
            .title_widget(&adw::WindowTitle::new("Waifu Viewer", ""))
            .build();

        container.pack_start(&menu_button);

        Self {
            container,
            about_button,
        }
    }

    pub fn container(&self) -> &adw::HeaderBar {
        &self.container
    }

    pub fn connect_about<F>(&self, about_callback: F) 
    where F: Fn() + 'static {
        self.about_button.connect_clicked(move |_| {
            about_callback();
        });
    }

    
}