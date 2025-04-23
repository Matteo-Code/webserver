use std::{io::Write, net::TcpStream};
use std::io::BufReader;
use std::io::prelude::*;
use crate::handlers::file::{self, FileHandler};

pub fn handle_client(mut stream: TcpStream) {
    let webs= vec!["html", "htm"];
    let styles = vec!["css", "scss"];

    println!("New client connected: {}", stream.peer_addr().expect("Could not get info of new client"));

    let path = extract_path(&stream);

    let (contents, content_type) = if path.ends_with(".pdf") {
        (file::PdfFileHandler::read(&path), file::PdfFileHandler::content_type())
    } else if path.ends_with(".txt") {
        (file::TextFileHandler::read(&path), file::TextFileHandler::content_type())
    } else if webs.contains(&path.rsplit(".").next().unwrap_or("")) { 
        (file::HtmlFileHandler::read(&path), file::HtmlFileHandler::content_type())
    }else if styles.contains(&path.rsplit(".").next().unwrap_or("")) {
        (file::CssFileHandler::read(&path), file::CssFileHandler::content_type())
    } else {
        (file::PdfFileHandler::read(&path), file::PdfFileHandler::content_type())
    };

    if contents.is_empty() {
        let response = "HTTP/1.1 404 NOT FOUND\r\nContent-Length: 0\r\n\r\n";
        stream.write_all(response.as_bytes()).unwrap();
        println!("Failed to read file or file is empty: {}", path);
        return;
    }

    println!("{}", content_type);

    //let body = "Hello, world!";
    let body = contents;
    let header = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n",
        body.len(),
        content_type
    );
    stream.write_all(header.as_bytes()).unwrap();
    stream.write_all(&body).unwrap();
}


fn extract_path(stream: &TcpStream) -> String {
    let mut reader = BufReader::new(stream);
    let mut request_line = String::new();

    if reader.read_line(&mut request_line).is_ok() {
        let path = request_line
            .split_whitespace()
            .nth(1)
            .unwrap_or("/")
            .to_string();
        println!("Requested path: {}", path);
        path
    } else {
        "/".to_string()
    }
}