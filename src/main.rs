use std::path::PathBuf;
use clap::{Command, value_parser};
use clipboard_saver::save_clipboard_tiff;

fn main() {
    let cmd = Command::new("clipboard-saver")
        .bin_name("clipboard-saver")
        .version("0.1.0")
        .args(&[
            clap::Arg::new("output")
                .index(1)
                .value_parser(value_parser!(PathBuf))
                .required(true),
        ]);

    let matches = cmd.get_matches();

    let output = match matches.get_one::<PathBuf>("output") {
        Some(output) => output,
        None => {
            println!("No output file specified");
            return;
        }
    };

    let file_name = match output.file_name() {
        Some(file_name) => file_name,
        None => {
            println!("Invalid output file name");
            return;
        }
    };

    let tiff = match save_clipboard_tiff() {
        Some(tiff) => tiff,
        None => {
            println!("Clipboard is empty or not an image");
            return;
        }
    };

    let image = match image::open(tiff) {
        Ok(image) => image,
        Err(err) => {
            println!("Failed to open clipboard image: {}", err);
            return;
        }
    };

    match image.save(output) {
        Ok(_) => println!("Saved clipboard image to {}", file_name.to_string_lossy()),
        Err(err) => println!("Failed to save clipboard image: {}", err),
    }
}
