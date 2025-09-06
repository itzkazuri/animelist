use libadwaita as adw;
use libadwaita::gtk;
use gtk::prelude::*;

use crate::ui::headerbar::WaifuHeaderBar;
use crate::ui::content::WaifuContent;
use crate::ui::components::window_builder::WindowBuilder;
use crate::ui::components::signal_connector::SignalConnector;

#[allow(dead_code)]
pub struct WaifuWindow {
    pub window: adw::ApplicationWindow,
}

impl WaifuWindow {
    pub fn new(app: &adw::Application) -> Self {
        let header_bar = WaifuHeaderBar::new();
        let content = WaifuContent::new();
        
        let window = WindowBuilder::create_window(app, &header_bar, &content);
        SignalConnector::connect_signals(&window, &header_bar, &content);

        Self {
            window,
        }
    }
}
