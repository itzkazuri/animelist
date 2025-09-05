mod api;
mod models;
mod ui;

use libadwaita as adw;
use adw::prelude::*;
use ui::window::WaifuWindow;

#[tokio::main]
async fn main() {
    // Cek kalau dijalankan sebagai root
    #[cfg(unix)]
    {
        if nix::unistd::Uid::effective().is_root() {
            eprintln!("onichan baka ngapain jalan aplikasi ini pakek root atau sudo ini cuman aplikasi biasa");
            // Optional: exit biar ga lanjut
            // std::process::exit(1);
        }
    }

    adw::init().expect("Failed to initialize Adwaita");

    let app = adw::Application::builder()
        .application_id("com.example.WaifuViewer")
        .build();

    app.connect_activate(|app| {
        let window = WaifuWindow::new(app);
        window.window.present();
    });

    app.run();
}
