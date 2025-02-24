use chrono::prelude::*;
use flume::Sender;
use image::ImageReader;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::path::Path;
use std::{fs, thread};

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Image(String, Vec<u8>),
    File(String, Vec<u8>),
    Text(String),
}

pub fn start_server(address: &str) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(address).unwrap();
    let mut clients: HashMap<SocketAddr, TcpStream> = HashMap::new();
    
    println!("Starting server on {}", address);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let addr = stream.peer_addr().unwrap();
        clients.insert(addr.clone(), stream.try_clone().unwrap());

        println!("Client {:?} connected!", addr);

        thread::Builder::new()
            .name(format!("server-thread-{}", clients.len()))
            .spawn(move || match handle_client(stream) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("Error when handling client {}", e)
                }
            })?;
    }

    Ok(())
}

pub fn get_client(address: &str) -> Result<TcpStream, Box<dyn Error>> {
    let stream = TcpStream::connect(address)?;

    Ok(stream)
}
pub fn handle_message(
    tx: Sender<Result<String, String>>,
    stream: TcpStream,
) -> Result<(), Box<dyn Error>> {
    let thread = thread::Builder::new()
        .name("client-listening-thread".into())
        .spawn(move || {
            let message = read_message(&stream);

            match message {
                Ok(Message::Image(file, content)) => {
                    match save_and_convert_image(&file, &content) {
                        Ok(_) => tx.send(Ok("Image was processsed".to_string())).unwrap(),
                        Err(e) => tx.send(Err(e.to_string())).unwrap(),
                    };
                }
                Ok(Message::File(file, content)) => match save_file(&file, &content) {
                    Ok(_) => tx.send(Ok("File was processsed".to_string())).unwrap(),
                    Err(e) => tx.send(Err(e.to_string())).unwrap(),
                },
                Ok(Message::Text(string)) => {
                    println!("Text message received: {}", string);
                    tx.send(Ok("Text message was sent".to_string())).unwrap();
                }
                Err(e) => tx.send(Err(e.to_string())).unwrap(),
            };
        });

    match thread {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
}

pub fn send_message(mut stream: TcpStream, message: &Message) -> Result<(), Box<dyn Error>> {
    let serialized_message = serde_json::to_string(&message).unwrap();
    let len = serialized_message.len() as u32;
    stream.write(&len.to_be_bytes()).unwrap();
    stream.write_all(serialized_message.as_bytes())?;
    
    Ok(())
}

fn save_and_convert_image(file: &str, content: &[u8]) -> Result<(), Box<dyn Error>> {
    let path = Path::new(file);
    let filename = path.file_name().unwrap().to_str().unwrap();
    let filepath = format!("./images/{}", filename);
    let created_file = File::create(filepath.clone());
    let current_timestamp = Local::now().timestamp();

    match created_file {
        Ok(mut file) => {
            file.write_all(content)?;
            let img = ImageReader::open(filepath.clone())?.decode()?;

            img.save(format!("./images/{}.png", current_timestamp))?;
            fs::remove_file(filepath.clone())?;
            Ok(())
        }
        Err(e) => Err(Box::new(e)),
    }
}

fn save_file(file: &str, content: &[u8]) -> Result<(), Box<dyn Error>> {
    let path = Path::new(file);
    let filename = path.file_name().unwrap().to_str().unwrap();
    let created_file = File::create(format!("./files/{}", filename));

    match created_file {
        Ok(mut file) => Ok(file.write_all(content)?),
        Err(e) => Err(Box::new(e)),
    }
}

fn read_message(mut stream: &TcpStream) -> Result<Message, Box<dyn Error>> {
    let mut len_bytes = [0u8; 4];
    stream.read_exact(&mut len_bytes).unwrap();

    let len = u32::from_be_bytes(len_bytes) as usize;
    let mut buffer = vec![0u8; len];

    stream.read_exact(&mut buffer)?;

    let serialized_message = serde_json::from_slice(&buffer)?;
    
    Ok(serialized_message)
}

fn handle_client(stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let message = read_message(&stream)?;
    println!("Received message");

    send_message(stream, &message)?;
    println!("Sending message to clients");

    Ok(())
}
