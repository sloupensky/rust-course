use std::collections::HashMap; 
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::thread;
use message_utils;
use log::{info};
use anyhow::{Result, Context};

pub fn start_server(address: &str) -> Result<()> {
    let listener = TcpListener::bind(address).context(format!("Failed to bind to {}", address))?;
    let mut clients: HashMap<SocketAddr, TcpStream> = HashMap::new();

    info!("Starting server on {}", address);

    for stream in listener.incoming() {
        let stream = stream?;
        let addr = stream.peer_addr()?;
        clients.insert(addr.clone(), stream.try_clone()?);

        info!("Client {:?} connected!", addr);

        thread::Builder::new()
            .name(format!("server-thread-{}", clients.len()))
            .spawn(move || {
                handle_client(stream)
            }).context("Failed to spawn server thread")?;
    }

    Ok(())
}

fn handle_client(stream: TcpStream) -> Result<()> {
    let message = message_utils::read_message(&stream).context("Failed to read message")?;
    info!("Received message");

    message_utils::send_message(stream, &message).context("Failed to send message")?;
    info!("Sending message to clients");

    Ok(())
}
