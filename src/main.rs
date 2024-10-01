mod functions;
mod views;

use eframe::egui;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Joplin Generator",
        options,
        Box::new(|cc| {
            // Image support (maybe needed later?)
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<JoplinGenerator>::default())
        }),
    )
}

// Future variables for books or persons need to be defined here
struct JoplinGenerator {
    current_view: View,
    author: String,
    booktitle: String,
}

// Future views need to be defined here
enum View {
    Home,
    NewBook,
    NewPerson,
}

// Variables defined above need to be set here so they can actually be used
impl Default for JoplinGenerator {
    fn default() -> Self {
        Self {
            current_view: View::Home,
            author: String::new(),
            booktitle: String::new(),
        }
    }
}

impl eframe::App for JoplinGenerator {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_view {
                View::Home => views::home_view::render_home_view(ui, self),
                View::NewBook => views::new_book_view::render_new_book_view(ui, self),
                View::NewPerson => views::new_person_view::render_new_person_view(ui, self),
            }
        });
    }
}