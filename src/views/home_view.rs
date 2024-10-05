use eframe::egui;

use crate::{JoplinGenerator, View};

pub fn render_home_view(ui: &mut egui::Ui, app: &mut JoplinGenerator) {
    if ui.button("Deutsch").clicked() {
        app.set_language("de-DE");
    }
    if ui.button("English").clicked() {
        app.set_language("en");
    }

    let localization = &app.localization;

    ui.horizontal(|ui| {
        ui.label(localization.translate("language"));
    });

    ui.heading(localization.translate("app-title"));
    if ui.button(localization.translate("new-book")).clicked() {
        app.current_view = View::NewBook;
    }
    if ui.button(localization.translate("new-person")).clicked() {
        app.current_view = View::NewPerson;
    }
}
