use std::fs::File;
use std::io::Write;
use rfd::FileDialog;
use dirs::home_dir;

use indoc::indoc;

use crate::{Status, Localization}; // Stelle sicher, dass Localization importiert wird

pub fn save_book(
    localization: &Localization,
    author: String,
    booktitle: String,
    publish_year: String,
    page_count: String,
    iban: String,
    status: Status,
    beginning_date: String,
    finished_date: String,
    book_rating: f32,
) {
    let default_dir = home_dir();

    // Open a file selection window to select the storage location
    let mut file_dialog = FileDialog::new()
        .set_title(localization.translate("new-book-view-save-book-button")); // Übersetzter Titel für den Dialog

    // Set the user folder as default, if available
    if let Some(path) = default_dir {
        file_dialog = file_dialog.set_directory(&path); 
    }

    // Use pick_folder() to select a folder
    let save_path = file_dialog.pick_folder(); 

    if let Some(path) = save_path {
        // Create the file name based on author and title 
        let filename = format!("📚 {} - {}.md", author, booktitle);
        let full_path = path.join(filename);

        // Try to create and write the file
        match File::create(&full_path) {
            Ok(mut file) => {
                // Create the Markdown table content
                let content = format!(
                    indoc!(
                        "## {title}
                        | {label} | {value} |
                        |---|---|
                        | {author_label} | {} |
                        | {title_label} | {} |
                        | {publish_year_label} | {} |
                        | {pages_label} | {} |
                        | {iban_label}  | {} |
                        | {status_label} | {} |  
                        | {reading_started_label} | {} |
                        | {reading_finished_label} | {} |
                        | {rating_label} | {} ★ |
                        
                        ## {opinion}
                        
                        ## {quotes}
                        "
                    ),
                    author, 
                    booktitle, 
                    publish_year, 
                    page_count, 
                    iban, 
                    status, 
                    beginning_date,
                    finished_date,
                    book_rating,
                    title = localization.translate("new-book"),
                    label = localization.translate("label"),
                    value = localization.translate("value"),
                    author_label = localization.translate("new-book-view-author"),
                    title_label = localization.translate("new-book-view-title"),
                    publish_year_label = localization.translate("new-book-view-publish-year"),
                    pages_label = localization.translate("new-book-view-pages"),
                    iban_label = localization.translate("new-book-view-iban"),
                    status_label = localization.translate("new-book-view-status"),
                    reading_started_label = localization.translate("new-book-view-reading-started"),
                    reading_finished_label = localization.translate("new-book-view-reading-finished"),
                    rating_label = localization.translate("new-book-view-rating"),
                    opinion = localization.translate("opinion"),
                    quotes = localization.translate("quotes"),
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
