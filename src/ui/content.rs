use libadwaita as adw;
use adw::prelude::*;
use libadwaita::gtk;
use gtk::{Box, Orientation};
use adw::{ViewStack, ViewSwitcher};

use crate::ui::pages::explore_page::ExplorePage;
use crate::ui::pages::favorites_page::FavoritesPage;

#[derive(Clone)]
pub struct WaifuContent {
    pub container: Box,
    pub explore_page: ExplorePage,
    pub favorites_page: FavoritesPage,
    view_stack: ViewStack,
}

impl WaifuContent {
    pub fn new() -> Self {
        let explore_page = ExplorePage::new();
        let favorites_page = FavoritesPage::new();

        let view_stack = ViewStack::new();
        // Add tabs with icons
        let explore_page_ref = view_stack.add_titled(&explore_page.container, Some("explore"), "Explore");
        explore_page_ref.set_icon_name(Some("system-search-symbolic"));
        
        let favorites_page_ref = view_stack.add_titled(&favorites_page.container, Some("favorites"), "Your Waifus");
        favorites_page_ref.set_icon_name(Some("starred-symbolic"));

        let view_switcher = ViewSwitcher::builder()
            .stack(&view_stack)
            .build();

        let container = Box::builder()
            .orientation(Orientation::Vertical)
            .build();
        
        container.append(&view_switcher);
        container.append(&view_stack);

        let content = Self {
            container,
            explore_page,
            favorites_page,
            view_stack,
        };

        content.favorites_page.load_favorites();

        let content_clone = content.clone();
        content.view_stack.connect_visible_child_notify(move |stack| {
            if stack.visible_child_name() == Some("favorites".into()) {
                content_clone.favorites_page.load_favorites();
            }
        });

        content
    }

    pub fn container(&self) -> &Box {
        &self.container
    }
}
