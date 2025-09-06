use libadwaita as adw;
use libadwaita::prelude::*;
use libadwaita::gtk;
use gtk::{Box, Orientation};

use crate::ui::headerbar::WaifuHeaderBar;
use crate::ui::content::WaifuContent;

pub struct WindowBuilder;

impl WindowBuilder {
    pub fn create_window(
        app: &adw::Application,
        header_bar: &WaifuHeaderBar,
        content: &WaifuContent,
    ) -> adw::ApplicationWindow {
        let main_box = Box::builder()
            .orientation(Orientation::Vertical)
            .vexpand(true)
            .build();

        main_box.append(header_bar.container());
        main_box.append(content.container());

        adw::ApplicationWindow::builder()
            .application(app)
            .title("Waifu Viewer")
            .default_width(1000)
            .default_height(800)
            .content(&main_box)
            .build()
    }
}