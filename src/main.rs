mod functions;
mod views;

use eframe::egui;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([450.0, 260.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Joplin Generator",
        options,
        Box::new(|_cc| {
            Ok(Box::<JoplinGenerator>::default())
        }),
    )
}

// Future variables for books or persons need to be defined here
struct JoplinGenerator {
    current_view: View,
    author: String,
    booktitle: String,
    publish_year: String,
    page_count: String,
    iban: String,
    status: Status,
    beginning_date: String,
    finished_date: String,
    book_rating: f32,
}

// Future views need to be defined here
enum View {
    Home,
    NewBook,
    NewPerson,
}

#[derive(PartialEq, Clone)]
enum Status {
    Read,
    Reading,
    ToRead,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Read => write!(f, "Read"),
            Status::Reading => write!(f, "Reading"),
            Status::ToRead => write!(f, "Want to read"), // Oder eine andere passende Darstellung
        }
    }
}

// Variables defined above need to be set here so they can actually be used
impl Default for JoplinGenerator {
    fn default() -> Self {
        Self {
            current_view: View::Home,
            author: String::new(),
            booktitle: String::new(),
            publish_year: String::new(),
            page_count: String::new(),
            iban: String::new(),
            status: Status::ToRead,
            beginning_date: String::new(),
            finished_date: String::new(),
            book_rating: 0.0,
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