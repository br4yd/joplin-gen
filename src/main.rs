mod functions;
mod views;

use eframe::egui;

use fluent_bundle::{FluentBundle, FluentResource};
use sys_locale::get_locale;
use unic_langid::LanguageIdentifier;

use std::sync::Arc;

use include_dir::{include_dir, Dir};

static TRANSLATIONS: Dir = include_dir!("translations");

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
    localization: Arc<Localization>,
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
            Status::ToRead => write!(f, "Want to read"),
        }
    }
}

// Variables defined above need to be set here so they can actually be used
impl Default for JoplinGenerator {
    fn default() -> Self {
        let default_locale = get_default_locale();
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
            localization: Arc::new(Localization::new(&default_locale)),
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


// =====[ Translations ]=====
pub struct Localization {
    bundle: FluentBundle<Arc<FluentResource>>,
}

impl Localization {
    pub fn new(locale: &str) -> Self {
        let langid: LanguageIdentifier = locale.parse().expect("Invalid language identifier");
        let mut bundle = FluentBundle::new(vec![langid]);

        // Try to load the translation file from the included directory
        let file_name = format!("{}.ftl", locale);
        let source = TRANSLATIONS
            .get_file(file_name)
            .or_else(|| TRANSLATIONS.get_file("en.ftl"))
            .expect("Translation file not found")
            .contents_utf8()
            .expect("Unable to read translation file");

        let resource = FluentResource::try_new(source.to_string()).expect("Unable to parse translation file");

        bundle.add_resource(Arc::new(resource)).expect("Failed to add resource to bundle");

        Localization { bundle }
    }

    pub fn translate(&self, message_id: &str) -> String {
        // Search for the message in the bundle
        if let Some(msg) = self.bundle.get_message(message_id) {
            if let Some(pattern) = msg.value() {
                let mut errors = vec![];
                let value = self.bundle.format_pattern(&pattern, None, &mut errors);
                return value.to_string();
            }
        }
        // Fallback message if the translation is not found
        format!("Missing translation for '{}'", message_id)
    }
}

pub fn get_default_locale() -> String {
    if let Some(locale) = get_locale() {
        let lang = locale.split('_').next().unwrap_or("en");
        println!("get_default_locale: {}", lang);
        if lang == "de" {
            return "de".to_string();
        }
        return lang.to_string();
    }
    "en".to_string() // Fallback to English
}
