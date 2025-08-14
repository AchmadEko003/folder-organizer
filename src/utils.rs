use std::path::Path;

pub fn is_image_file(path: &Path) -> bool {
    match path.extension().and_then(|e| e.to_str()).map(|e| e.to_lowercase()) {
        Some(ext) => matches!(
            ext.as_str(),
            "png" | "jpg" | "jpeg" | "gif" | "bmp" | "svg" | "webp" | "tiff" | "ico"
        ),
        None => false,
    }
}

pub fn get_path_right_click() -> Option<String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        println!("Right-clicked path: {}", args[1]);
        Some(args[1].clone())
    } else {
        None
    }
}
