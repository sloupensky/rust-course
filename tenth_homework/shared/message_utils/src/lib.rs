use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Image(String, Vec<u8>, i32),
    File(String, Vec<u8>, i32),
    Text(String, i32),
}

#[derive(Debug, Error)]
pub enum MessageError {
    #[error("Write into stream failed!")]
    StreamWriteFailed,
    #[error("Read from stream failed!")]
    StreamReadFailed,
    #[error("Serialization of message failed!")]
    SearializationFailure ,
    #[error("Connection closed")]
    ConnectionClosed
}


/// Send message through provided Writer
///
/// Send provided message of type `Message` and write it into provided tcp stream writer
pub async fn send_message(writer: &mut (impl AsyncWriteExt + Unpin), message: &Message) -> Result<(), MessageError> {
    let serialized_message = serde_json::to_string(&message).unwrap();

    writer.write_all(serialized_message.as_bytes()).await.map_err(|_| MessageError::StreamWriteFailed)?;

    Ok(())
}

/// Read message from provided reader
///
/// Read from provided tcp writer and return `Message` when received
pub async fn read_message(socket:  &mut (impl AsyncReadExt + Unpin)) -> Result<Message, MessageError> {
    let mut buffer = Vec::with_capacity(1024 * 1024);
    let mut temp = [0u8; 4096];

    loop {
        match socket.read(&mut temp).await {
            Ok(0) => return Err(MessageError::ConnectionClosed),
            Ok(n) => {
                buffer.extend_from_slice(&temp[..n]); // Append to dynamic buffer
                match serde_json::from_slice::<Message>(&buffer) {
                    Ok(msg) => return Ok(msg),
                    Err(_) => continue,
                }
            }
            Err(_) => return Err(MessageError::StreamReadFailed),
        }
    }
}

