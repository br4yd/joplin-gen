// In new_book_view.rs
use eframe::egui;
use crate::{functions::save_book::save_book, JoplinGenerator, View}; // Importiere die save_book-Funktion

pub fn render_new_book_view(ui: &mut egui::Ui, app: &mut JoplinGenerator) {
    ui.heading("New Book");

    ui.horizontal(|ui| {
        ui.label("Author:");
        ui.text_edit_singleline(&mut app.author);
    });
    
    ui.horizontal(|ui| {
        ui.label("Title:");
        ui.text_edit_singleline(&mut app.booktitle);
    });
    
    if ui.button("Save Book").clicked() {
        save_book(app.author.clone(), app.booktitle.clone());
        app.current_view = View::Home; 

        app.author.clear();
        app.booktitle.clear();
    }
}