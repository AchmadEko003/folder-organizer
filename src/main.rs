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

fn main() {
    let matches = Command::new("Folder Organizer Tool")
        .version("1.0")
        .author("Madko") // you can change this to your name
        .about("Organizes files in a folder by extension")
        .arg(Arg::new("folder")
            .help("Path to the folder to organize")
            .required(true)
            .index(1))
        .get_matches();

    let folder = matches.get_one::<String>("folder").unwrap();
    match organize_folder(folder) {
        Ok(_) => println!("Folder organized successfully."),
        Err(e) => eprintln!("Error: {}", e),
    }
    
    println!("Closing in 5 seconds...");
    std::thread::sleep(std::time::Duration::from_secs(5));
}
