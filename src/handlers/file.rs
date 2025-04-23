use std::fs;

pub trait FileHandler {
    fn read(file_path: &str) -> Vec<u8>;
    fn content_type() -> &'static str;
}

pub struct TextFileHandler;

impl FileHandler for TextFileHandler {
    fn read(file_path: &str) -> Vec<u8> {
        let path = "public/".to_owned() + &file_path.trim_start_matches('/').to_string();
        match fs::read_to_string(&path) {
            Ok(content) => content.into_bytes(),
            Err(e) => {
                eprintln!("Error reading text file: {}", e);
                Vec::new()
            }
        }
    }

    fn content_type() -> &'static str {
        "text/plain; charset=utf-8"
    }
}

pub struct PdfFileHandler;

impl FileHandler for PdfFileHandler {
    fn read(file_path: &str) -> Vec<u8> {
        let path = "public/".to_owned() + &file_path.trim_start_matches('/').to_string();
        match fs::read(&path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error reading PDF file: {}", e);
                Vec::new()
            }
        }
    }

    fn content_type() -> &'static str {
        "application/pdf"
    }
}

pub struct HtmlFileHandler;

impl FileHandler for HtmlFileHandler {
     fn read(file_path: &str) -> Vec<u8> {
        let path = "public/".to_owned() + &file_path.trim_start_matches('/').to_string();
        match fs::read_to_string(&path) {
            Ok(content) => content.into_bytes(),
            Err(e) => {
                eprintln!("Error reading text file: {}", e);
                Vec::new()
            }
        }
    }

    fn content_type() -> &'static str {
        "text/html; charset=utf-8"
    }
}

pub struct CssFileHandler;

impl FileHandler for CssFileHandler {
     fn read(file_path: &str) -> Vec<u8> {
        let path = "public/".to_owned() + &file_path.trim_start_matches('/').to_string();
        match fs::read_to_string(&path) {
            Ok(content) => content.into_bytes(),
            Err(e) => {
                eprintln!("Error reading text file: {}", e);
                Vec::new()
            }
        }
    }

    fn content_type() -> &'static str {
        "text/css; charset=utf-8"
    }
}