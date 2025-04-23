mod request_handler;

use std::net::TcpListener;


fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for stream in listener.incoming() {
        request_handler::handle_client(stream?);
    }
    Ok(())
}