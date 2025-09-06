use libadwaita as adw;
use adw::prelude::*;
use glib::Bytes;
use waifu_viewer::ui::window::WaifuWindow;

mod resources {
    pub static COMPILED: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/compiled.gresource"));
}

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

    // Load resources
    let data = Bytes::from_static(resources::COMPILED);
    let resource = gio::Resource::from_data(&data).expect("Failed to create resource from data");
    gio::resources_register(&resource);

    let app = adw::Application::builder()
        .application_id("com.example.WaifuViewer")
        .build();

    app.connect_activate(|app| {
        let window = WaifuWindow::new(app);
        window.window.present();
    });

    app.run();
}
