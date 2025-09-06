use libadwaita as adw;
use adw::prelude::*;
use libadwaita::gtk;
use gtk::{ResponseType, FlowBox, Spinner};
use gtk::glib;

use crate::ui::handlers::SearchHandler;

pub struct DialogManager;

impl DialogManager {
    pub fn show_about_dialog(window: &adw::ApplicationWindow) {
        let dialog = gtk::AboutDialog::builder()
            .transient_for(window)
            .modal(true)
            .program_name("Waifu Viewer")
            .version("1.0.0")
            .comments("A simple application to view and search anime characters (waifus)")
            .website("https://github.com/itzkazuri")
            .authors(vec!["itzkazuri".to_string()])
            .copyright("© 2025 itzkazuri. This application is testing for purpose.")
            .license_type(gtk::License::MitX11)
            .logo_icon_name("masha")
            .build();
            
        dialog.present();
    }

    
}