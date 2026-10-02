use std::time::Duration;

use gtk::glib;
use gtk::prelude::*;
use gtk::Application;

use dynamic_island_rs::island::{DynamicIsland, IslandMode};

fn main() {
    let app = Application::builder()
        .application_id("com.example.dynamic-island-rs")
        .build();

    app.connect_activate(|app| {
        let island = DynamicIsland::new(app);
        island.present();
        island.set_mode(IslandMode::Minimal);

        let first = island.clone();
        glib::timeout_add_local(Duration::from_millis(800), move || {
            first.show_notification("Now playing", "Rust dynamic island");
            glib::ControlFlow::Continue
        });

        let second = island.clone();
        glib::timeout_add_local(Duration::from_millis(2000), move || {
            second.expand_notification("Now playing", "Rust dynamic island\nThis is an expanded state");
            glib::ControlFlow::Continue
        });

        let third = island.clone();
        glib::timeout_add_local(Duration::from_millis(4000), move || {
            third.set_mode(IslandMode::Minimal);
            glib::ControlFlow::Break
        });
    });

    app.run();
}
