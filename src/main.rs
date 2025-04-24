mod handlers;
mod config;
use config::tls::load_tls_config;
use handlers::request;
use rustls::{ServerConnection, StreamOwned};

use std::{net::TcpListener, sync::Arc};


fn main() -> std::io::Result<()> {
    let config = load_tls_config();
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for stream in listener.incoming() {
        let tcp_stream = stream.unwrap();
        match &config {
            Ok(conf) => {
                let arc_config = Arc::new(conf.clone());
                let conn = ServerConnection::new(arc_config.clone()).unwrap();
                let tls_stream = StreamOwned::new(conn, tcp_stream);
                request::handle_client(tls_stream.sock);
            }
            Err(_) => {
                println!("couldn't load certificates");
                request::handle_client(tcp_stream);
            } 
        }
        
    }
    Ok(())
}