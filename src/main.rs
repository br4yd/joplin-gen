use std::{fs::File, io::Write};

use eframe::egui;
use rfd::FileDialog;
use dirs::home_dir;
use indoc::indoc;

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
                View::Home => {
                    // Bring a selection with what template for Joplin Hotfolder to create (currently book or person)
                    ui.heading("Joplin Generator");
                    // TODO: Implement i18n for translations (DE & EN)
                    if ui.button("New Book").clicked() {
                        self.current_view = View::NewBook; 
                    }
                    if ui.button("New Person").clicked() {
                        self.current_view = View::NewPerson;
                    }
                }

                View::NewBook => {
                    ui.heading("New Book");

                    // UI-Elements
                    ui.horizontal(|ui| {
                        ui.label("Author:");
                    ui.text_edit_singleline(&mut self.author);
                    });
                    
                    ui.horizontal(|ui| {
                        ui.label("Title:");
                    ui.text_edit_singleline(&mut self.booktitle);
                    });
                    
                    if ui.button("Save Book").clicked() {
                        // Process entered information
                        savebook(self.author.clone(), self.booktitle.clone());
                        self.current_view = View::Home; 
                    }
                }

                View::NewPerson => {
                    ui.heading("New Person");
                    // TODO: Everything in this part - currently just a placeholder
                    if ui.button("Save Person").clicked() {
                        // Process entered information
                        self.current_view = View::Home; 
                    }
                }
            }
        });
    }
}

fn savebook(author: String, booktitle: String) {
    let default_dir = home_dir();

    // Open a file selection window to select the storage location
    let mut file_dialog = FileDialog::new()
        .set_title("Select Joplin Hotfolder");

    // Set the user folder as default, if available
    if let Some(path) = default_dir {
        file_dialog = file_dialog.set_directory(&path); 
    }

    // Use pick_folder() to select a folder
    let save_path = file_dialog.pick_folder(); 

    if let Some(path) = save_path {
        // Create the file name based on author and title 
        let filename = format!("{} - {}.md", author, booktitle);
        let full_path = path.join(filename);

        // Try to create and write the file
        match File::create(&full_path) {
            Ok(mut file) => {
                // Create the Markdown table content
                let content = format!(
                    indoc!( // TODO: i18n within the string depending on language
                        "| Label | Value |
                        |---|---|
                        | Author | {} |
                        | Title | {} |"
                    ),
                    author, booktitle
                );

                if let Err(e) = file.write_all(content.as_bytes()) {
                    eprintln!("Error while writing file: {}", e);
                } else {
                    println!("Book saved at: {:?}", full_path);
                }
            }
            Err(e) => {
                eprintln!("Error while creating the file: {}", e);
            }
        }
    } else {
        println!("Cancelled saving the file.");
    }

}
