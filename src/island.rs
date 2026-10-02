use gtk::glib;
use gtk::prelude::*;
use gtk::{
    gdk, Application, ApplicationWindow, Box, CssProvider, Label, Orientation, StyleContext,
};

use crate::theme::CSS;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IslandMode {
    Minimal,
    Compact,
    Expanded,
}

#[derive(Clone)]
pub struct DynamicIsland {
    window: ApplicationWindow,
    host: Box,
    title: Label,
    body: Label,
    mode: glib::Cell<IslandMode>,
}

impl DynamicIsland {
    pub fn new(app: &Application) -> Self {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Dynamic Island")
            .decorated(false)
            .resizable(false)
            .build();

        window.set_default_size(220, 70);
        window.set_focus_on_click(false);

        let css_provider = CssProvider::new();
        css_provider.load_from_string(CSS);

        if let Some(display) = gdk::Display::default() {
            StyleContext::add_provider_for_display(
                &display,
                &css_provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }

        let host = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(8)
            .build();
        host.set_widget_name("dynamic-island");
        host.add_css_class("minimal");
        host.set_halign(gtk::Align::Center);
        host.set_valign(gtk::Align::Center);
        host.set_margin_top(12);
        host.set_margin_bottom(12);
        host.set_margin_start(12);
        host.set_margin_end(12);

        let content = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(2)
            .build();

        let title = Label::new(Some("Now playing"));
        title.add_css_class("title");
        title.set_halign(gtk::Align::Start);

        let body = Label::new(Some("Rust dynamic island"));
        body.add_css_class("body");
        body.set_halign(gtk::Align::Start);
        body.set_wrap(true);
        body.set_width_chars(20);

        content.append(&title);
        content.append(&body);
        host.append(&content);
        window.set_child(Some(&host));

        let island = Self {
            window,
            host,
            title,
            body,
            mode: glib::Cell::new(IslandMode::Minimal),
        };

        island.apply_style();
        island
    }

    fn apply_style(&self) {
        self.host.remove_css_class("minimal");
        self.host.remove_css_class("compact");
        self.host.remove_css_class("expanded");

        let mode = self.mode.get();
        match mode {
            IslandMode::Minimal => {
                self.host.add_css_class("minimal");
                self.window.set_default_size(180, 60);
                self.title.set_text("Now playing");
                self.body.set_text("Rust dynamic island");
            }
            IslandMode::Compact => {
                self.host.add_css_class("compact");
                self.window.set_default_size(240, 68);
                self.title.set_text("Now playing");
                self.body.set_text("Rust dynamic island");
            }
            IslandMode::Expanded => {
                self.host.add_css_class("expanded");
                self.window.set_default_size(360, 120);
                self.title.set_text("Now playing");
                self.body.set_text("Rust dynamic island\nA compact media card");
            }
        }

        self.window.present();
    }

    pub fn set_mode(&self, mode: IslandMode) {
        self.mode.set(mode);
        self.apply_style();
    }

    pub fn show_notification(&self, title: &str, message: &str) {
        self.title.set_text(title);
        self.body.set_text(message);
        self.set_mode(IslandMode::Compact);
    }

    pub fn expand_notification(&self, title: &str, message: &str) {
        self.title.set_text(title);
        self.body.set_text(message);
        self.set_mode(IslandMode::Expanded);
    }

    pub fn present(&self) {
        self.window.present();
    }
}

pub mod prelude {
    pub use super::{DynamicIsland, IslandMode};
}
