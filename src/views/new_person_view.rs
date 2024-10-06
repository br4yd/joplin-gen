// In new_person_view.rs
use eframe::egui;

use crate::{JoplinGenerator, View};

pub fn render_new_person_view(ui: &mut egui::Ui, app: &mut JoplinGenerator) {
    let localization = &app.localization;

    ui.heading(localization.translate("new-person"));
    // TODO: Everything in this part - currently just a placeholder
    if ui.button(localization.translate("save-person-button")).clicked() {
        app.current_view = View::Home; 
    }

    if ui.button(localization.translate("back-person-button")).clicked() {
        app.current_view = View::Home;
    }
}