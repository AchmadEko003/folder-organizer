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
