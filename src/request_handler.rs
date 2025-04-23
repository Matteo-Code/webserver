use std::{io::{Read, Write}, net::TcpStream};

pub fn handle_client(mut stream: TcpStream) {
    println!("New client connected: {}", stream.peer_addr().expect("Could not get info of new client"));

    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..]);

    let request_line = request.lines().next().unwrap_or("");
    let path = request_line.split_whitespace().nth(1).unwrap_or("/");
    println!("Requested path: {}", path);

    let body = "Hello, world!";
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/plain\r\n\r\n{}",
        body.len(),
        body
    );
    stream.write_all(response.as_bytes()).unwrap();
}
