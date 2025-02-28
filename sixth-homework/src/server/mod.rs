use std::collections::HashMap;
use std::error::Error;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::thread;
use message_utils;
use log::{info, error};

pub fn start_server(address: &str) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(address)?;
    let mut clients: HashMap<SocketAddr, TcpStream> = HashMap::new();

    info!("Starting server on {}", address);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let addr = stream.peer_addr().unwrap();
        clients.insert(addr.clone(), stream.try_clone().unwrap());

        info!("Client {:?} connected!", addr);

        thread::Builder::new()
            .name(format!("server-thread-{}", clients.len()))
            .spawn(move || match handle_client(stream) {
                Ok(_) => (),
                Err(e) => {
                    error!("Error when handling client {}", e)
                }
            })?;
    }

    Ok(())
}

fn handle_client(stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let message = message_utils::read_message(&stream)?;
    info!("Received message");

    message_utils::send_message(stream, &message)?;
    info!("Sending message to clients");

    Ok(())
}
