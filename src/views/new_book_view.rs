use eframe::egui;
use crate::{functions::save_book::save_book, JoplinGenerator, Status, View};

pub fn render_new_book_view(ui: &mut egui::Ui, app: &mut JoplinGenerator) {
    let localization = &app.localization;

    ui.heading(localization.translate("new-book"));

    egui::Grid::new("book_details_grid")
        .num_columns(2)
        .spacing([40.0, 4.0])
        .striped(true)
        .show(ui, |ui| {
            ui.label(localization.translate("new-book-view-author"));
            ui.text_edit_singleline(&mut app.author);
            ui.end_row();

            ui.label(localization.translate("new-book-view-title"));
            ui.text_edit_singleline(&mut app.booktitle);
            ui.end_row();

            ui.label(localization.translate("new-book-view-publish-year"));
            ui.text_edit_singleline(&mut app.publish_year);
            ui.end_row();

            ui.label(localization.translate("new-book-view-pages"));
            ui.text_edit_singleline(&mut app.page_count);
            ui.end_row();

            ui.label(localization.translate("new-book-view-iban"));
            ui.text_edit_singleline(&mut app.iban);
            ui.end_row();

            ui.label(localization.translate("new-book-view-status"));
            ui.horizontal(|ui| {
                ui.radio_value(&mut app.status, Status::ToRead, localization.translate("new-book-view-status-want-to-read"));
                ui.radio_value(&mut app.status, Status::Reading, localization.translate("new-book-view-status-reading"));
                ui.radio_value(&mut app.status, Status::Read, localization.translate("new-book-view-status-read"));
            });
            ui.end_row();

            // Conditionally display fields based on the selected status
            match app.status {
                Status::Reading => {
                    ui.label(localization.translate("new-book-view-reading-started"));
                    ui.text_edit_singleline(&mut app.beginning_date);
                    ui.end_row();
                }
                Status::Read => {
                    ui.label(localization.translate("new-book-view-reading-started"));
                    ui.text_edit_singleline(&mut app.beginning_date);
                    ui.end_row();

                    ui.label(localization.translate("new-book-view-reading-finished"));
                    ui.text_edit_singleline(&mut app.finished_date);
                    ui.end_row();

                    ui.label(localization.translate("new-book-view-rating"));
                    ui.add(egui::Slider::new(&mut app.book_rating, 1.0..=5.0)
                        .step_by(0.25)
                        .suffix("★"));
                    ui.end_row();
                }
                Status::ToRead => {
                    // No fields to display
                }
            }
        });

    ui.horizontal(|ui| {
        if ui.button(localization.translate("new-book-view-save-book-button")).clicked() {
            save_book(
                &localization,
                app.author.clone(),
                app.booktitle.clone(),
                app.publish_year.clone(),
                app.page_count.clone(),
                app.iban.clone(),
                app.status.clone(),
                app.beginning_date.clone(),
                app.finished_date.clone(),
                app.book_rating.clone(),
            );
            app.current_view = View::Home;
    
            // Reset variables
            app.author = "".to_string();
            app.booktitle = "".to_string();
            app.publish_year = "".to_string();
            app.page_count = "".to_string();
            app.iban = "".to_string();
            app.status = Status::ToRead;
            app.beginning_date = "".to_string();
            app.finished_date = "".to_string();
            app.book_rating = 1.0;
        }
    
        if ui.button(localization.translate("back-button")).clicked() {
            app.current_view = View::Home;
            
            // Reset variables
            app.author = "".to_string();
            app.booktitle = "".to_string();
            app.publish_year = "".to_string();
            app.page_count = "".to_string();
            app.iban = "".to_string();
            app.status = Status::ToRead;
            app.beginning_date = "".to_string();
            app.finished_date = "".to_string();
            app.book_rating = 1.0;
        }
    });
}