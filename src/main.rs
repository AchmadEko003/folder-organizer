use std::fs;
use std::path::Path;
use clap::{Arg, Command};

fn organize_folder(folder_path: &str) -> std::io::Result<()> {
    let entries = fs::read_dir(folder_path)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                let ext_folder = Path::new(folder_path).join(ext);
                fs::create_dir_all(&ext_folder)?;
                let file_name = path.file_name().unwrap();
                let new_path = ext_folder.join(file_name);
                fs::rename(&path, &new_path)?;
            }
        }
    }
    Ok(())
}

fn welcome_message() {
    println!("Welcome to the Folder Organizer Tool!");
    println!("This tool will help you organize your files by their extension.");
}

fn main() {
    welcome_message();

    let mut input = String::new();

    println!("Please input your folder path to organize:");

    std::io::stdin().read_line(&mut input).unwrap();
    let _folder_path = input.trim();

    match organize_folder(_folder_path) {
        Ok(_) => println!("Folder organized successfully."),
        Err(e) => eprintln!("Error: {}", e),
    }
    
    println!("Closing in 5 seconds...");
    std::thread::sleep(std::time::Duration::from_secs(5));
}
