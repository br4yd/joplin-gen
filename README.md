# Joplin Gen
This is a small tool, written in [Rust](https://www.rust-lang.org/), using [egui](https://github.com/emilk/egui).  
It's purpose is to generate Markdown files for books and persons from a template and fill it with information to then be used in [Joplin](https://joplinapp.org/) and it's [Hotfolder Plugin](https://github.com/JackGruber/joplin-plugin-hotfolder) so it's then being imported automatically depending on the settings.

In the file creation process you will need to select the Hotfolder defined via the [Hotfolder Plugin](https://github.com/JackGruber/joplin-plugin-hotfolder) for whatever file you want to create. Note that the [Hotfolder Plugin](https://github.com/JackGruber/joplin-plugin-hotfolder) only supports a single Hotfolder to be defined. I solve this by importing both - book notes and people notes into one "unsorted" folder and then manually move it around inside Joplin after it has been created.

## Why does it exist?
Simple. Joplin has not really good support or plugins for creating a good and nice template for the purpose of adding books with some key information to track the process and I'm to lazy to actually copy and paste a empty template back and forth which can create human error. So to solve this, this exists. I also wanted to have a GUI and not a terminal programm, because a GUI makes it easy to build on top of this software to be able to add quotes and my opinion to every quote in the future too. They could be created when creating a book and put into the same file. As a book can have one or one hundred quotes + opinions marked and written by the reader a GUI makes more sense which could implement some kind of visible feedback for already added quotes before actually creating the file in the Joplin Hotfolder.

## Building from source
You need to have Rust and the cargo toolchain installed on your computer. There is a documentation on that on the official Rust website: https://www.rust-lang.org/tools/install

After you have this installed clone this repository:  
```bash
git clone https://github.com/br4yd/joplin-gen
```

Now go into the cloned directory and build the project as a release optimized version:
```bash
cargo build -r
```

You should now have a `target` sub-directory containing the executable file for your OS. For me this is in `target/release`.