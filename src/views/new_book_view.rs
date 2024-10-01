use eframe::egui;
use crate::{functions::save_book::save_book, JoplinGenerator, Status, View};

pub fn render_new_book_view(ui: &mut egui::Ui, app: &mut JoplinGenerator) {
    ui.heading("New Book");

    egui::Grid::new("book_details_grid")
        .num_columns(2)
        .spacing([40.0, 4.0])
        .striped(true)
        .show(ui, |ui| {
            ui.label("Author:");
            ui.text_edit_singleline(&mut app.author);
            ui.end_row();

            ui.label("Title:");
            ui.text_edit_singleline(&mut app.booktitle);
            ui.end_row();

            ui.label("Publish year:");
            ui.text_edit_singleline(&mut app.publish_year);
            ui.end_row();

            ui.label("Pages:");
            ui.text_edit_singleline(&mut app.page_count);
            ui.end_row();

            ui.label("IBAN / EAN:");
            ui.text_edit_singleline(&mut app.iban);
            ui.end_row();

            ui.label("Status:");
            ui.horizontal(|ui| {
                ui.radio_value(&mut app.status, Status::Reading, "Reading");
                ui.radio_value(&mut app.status, Status::Read, "Read");
                ui.radio_value(&mut app.status, Status::ToRead, "Want to read");
            });
            ui.end_row();

            ui.label("Reading started:");
            ui.text_edit_singleline(&mut app.beginning_date);
            ui.end_row();

            ui.label("Reading finished:");
            ui.text_edit_singleline(&mut app.finished_date);
            ui.end_row();
        });

    ui.horizontal(|ui| {
        ui.label("Rating:");
        ui.add(egui::Slider::new(&mut app.book_rating, 1.0..=5.0)
            .step_by(0.25)
            .suffix("★"));
    });

    if ui.button("Save Book").clicked() {
        save_book(
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
        app.author = "".to_string().to_string();
        app.booktitle = "".to_string();
        app.publish_year = "".to_string();
        app.page_count = "".to_string();
        app.iban = "".to_string();
        app.status = Status::ToRead;
        app.beginning_date = "".to_string();
        app.finished_date = "".to_string();
        app.book_rating = 1.0;
    }

    if ui.button("Back").clicked() {
        app.current_view = View::Home;
        
        // Reset variables
        app.author = "".to_string().to_string();
        app.booktitle = "".to_string();
        app.publish_year = "".to_string();
        app.page_count = "".to_string();
        app.iban = "".to_string();
        app.status = Status::ToRead;
        app.beginning_date = "".to_string();
        app.finished_date = "".to_string();
        app.book_rating = 1.0;
    }
}