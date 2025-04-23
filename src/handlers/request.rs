use std::{io::{Read, Write}, net::TcpStream};
use crate::handlers::file;

pub fn handle_client(mut stream: TcpStream) {
    println!("New client connected: {}", stream.peer_addr().expect("Could not get info of new client"));

    let path = extract_path(&stream);

    let contents;
    
    let content_type = find_content_type(&path);


    if(content_type == "application/pdf"){
        contents = file::get_raw_file_bytes(&path);
    }else if(content_type == "text/plain"){
        contents = file::get_raw_file_contents(&path).into();
    }else{
        contents = file::get_raw_file_bytes(&path);
    }

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

fn find_content_type(file_path: &str) -> &str {
    match file_path.rsplit('.').next().unwrap_or("") {
        "pdf" => "application/pdf",
        "txt" => "text/plain",
        "html" => "text/html",
        _ => "application/octet-stream", // fallback for unknown types
    }
}


fn extract_path(mut stream: &TcpStream) -> String{
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..]);

    let request_line = request.lines().next().unwrap_or("");
    let path = request_line.split_whitespace().nth(1).unwrap_or("/");
    println!("Requested path: {}", path);
    path.to_string()
}