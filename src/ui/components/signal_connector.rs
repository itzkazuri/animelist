use libadwaita as adw;
use adw::prelude::*;
use libadwaita::gtk;

use crate::ui::headerbar::WaifuHeaderBar;
use crate::ui::content::WaifuContent;
use crate::ui::dialogs::DialogManager;
use crate::ui::handlers::SearchHandler;

pub struct SignalConnector;

impl SignalConnector {
    pub fn connect_signals(
        window: &adw::ApplicationWindow,
        header_bar: &WaifuHeaderBar,
        content: &WaifuContent,
    ) {
        // Connect about functionality
        let window_clone = window.clone();
        header_bar.connect_about(move || {
            DialogManager::show_about_dialog(&window_clone);
        });

        // Connect search and fetch functionality
        SearchHandler::connect_search_signals(&content.explore_page);
    }
}
