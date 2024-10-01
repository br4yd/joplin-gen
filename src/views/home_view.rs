use eframe::egui;

use crate::{JoplinGenerator, View};

pub fn render_home_view(ui: &mut egui::Ui, app: &mut JoplinGenerator) {
    ui.heading("Joplin Generator");
    if ui.button("New Book").clicked() {
        app.current_view = View::NewBook; 
    }
    if ui.button("New Person").clicked() {
        app.current_view = View::NewPerson;
    }
}