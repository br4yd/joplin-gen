use std::fs::File;
use std::io::Write;
use rfd::FileDialog;
use dirs::home_dir;

use indoc::indoc;

pub fn save_book(author: String, booktitle: String) {
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