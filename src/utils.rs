use std::path::Path;

pub fn is_image_file(ext: &str) -> bool {
    match ext {
        "png" | "jpg" | "jpeg" | "gif" | "bmp" | "svg" | "webp" | "tiff" | "ico" => true,
        _ => false,
    }
    // match path.extension().and_then(|e| e.to_str()).map(|e| e.to_lowercase()) {
    //     Some(ext) => matches!(
    //         ext.as_str(),
    //         "png" | "jpg" | "jpeg" | "gif" | "bmp" | "svg" | "webp" | "tiff" | "ico"
    //     ),
    //     None => false,
    // }
}

pub fn is_document_file(path: &std::path::Path) -> bool {
    match path.extension().and_then(|e| e.to_str()).map(|e| e.to_lowercase()) {
        Some(ext) => matches!(
            ext.as_str(),
            "pdf" | "doc" | "docx" | "odt" | "rtf" | "txt" | "md" | "markdown"
                | "html" | "htm" | "xls" | "xlsx" | "csv" | "ppt" | "pptx"
                | "epub" | "mobi" | "tex" | "chm" | "xml" | "json"
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
