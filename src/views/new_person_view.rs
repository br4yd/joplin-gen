// In new_person_view.rs
use eframe::egui;

use crate::{JoplinGenerator, View};

pub fn render_new_person_view(ui: &mut egui::Ui, app: &mut JoplinGenerator) {
    ui.heading("New Person");
    // TODO: Everything in this part - currently just a placeholder
    if ui.button("Save Person").clicked() {
        app.current_view = View::Home; 
    }
}