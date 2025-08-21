use std::fs;
use std::path::Path;
mod utils;
use utils::get_path_right_click;

fn organize_folder(folder_path: &str) -> std::io::Result<()> {
    let entries = fs::read_dir(folder_path)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        println!("Processing file: {}", path.display());
        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                let ext_folder = Path::new(folder_path).join(ext.to_lowercase());
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
    
    let folder_path = if let Some(path) = get_path_right_click() {
        path
    } else {
        println!("Please input your folder path or you can drag and drop a folder to organize:");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut input_result = input.trim().to_string();
        input_result.retain(|c| c != '\'' && c != '"');
        input_result
    };

    if folder_path.is_empty() {
        eprintln!("No folder path provided.");
        return;
    }

    match organize_folder(&folder_path) {
        Ok(_) => println!("Folder organized successfully."),
        Err(e) => eprintln!("Error: {}", e),
    }
    
    println!("Closing in 5 seconds...");
    std::thread::sleep(std::time::Duration::from_secs(3));
}
