use eframe::egui;

use crate::{JoplinGenerator, View};

pub fn render_home_view(ui: &mut egui::Ui, app: &mut JoplinGenerator) {
    let localization = &app.localization;

    ui.heading(localization.translate("app-title"));
    if ui.button(localization.translate("new-book")).clicked() {
        app.current_view = View::NewBook;
    }
    if ui.button(localization.translate("new-person")).clicked() {
        app.current_view = View::NewPerson;
    }
}
