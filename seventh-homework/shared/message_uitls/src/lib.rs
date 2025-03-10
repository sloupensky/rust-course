use std::io::{Read, Write};
use std::net::TcpStream;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Image(String, Vec<u8>),
    File(String, Vec<u8>),
    Text(String),
}

#[derive(Debug, Error)]
pub enum MessageError {
    #[error("Write into stream failed!")]
    StreamWriteFailed,
    #[error("Read from stream failed!")]
    StreamReadFailed,
    #[error("Serialization of message failed!")]
    SearializationFailure
}


pub fn send_message(mut stream: TcpStream, message: &Message) -> Result<(), MessageError> {
    let serialized_message = serde_json::to_string(&message).unwrap();
    let len = serialized_message.len() as u32;
    stream.write(&len.to_be_bytes()).unwrap();
    stream.write_all(serialized_message.as_bytes()).map_err(|_| MessageError::StreamWriteFailed)?;

    Ok(())
}


pub fn read_message(mut stream: &TcpStream) -> Result<Message, MessageError> {
    let mut len_bytes = [0u8; 4];
    stream.read_exact(&mut len_bytes).unwrap();

    let len = u32::from_be_bytes(len_bytes) as usize;
    let mut buffer = vec![0u8; len];

    stream.read_exact(&mut buffer).map_err(|_| MessageError::StreamReadFailed).map_err(|_| MessageError::StreamWriteFailed)?;

    let serialized_message = serde_json::from_slice(&buffer).map_err(|_| MessageError::SearializationFailure)?;

    Ok(serialized_message)
}