use anyhow::{anyhow, Context, Result};
use file_utils;
use flume::{Sender, Receiver};
use input_utils;
use input_utils::InputOperationType;
use log::{error, info};
use message_utils;
use message_utils::Message; 
use tokio::net::TcpStream;

pub async fn handle_client_by_mode(tx: Sender<Result<String, String>>,rx: Receiver<Result<String, String>>, address: String) -> Result<()> {
    match handle_communication(tx, address).await {
        Ok(_) => {
            while let Ok(message_result) = rx.recv() {
                handle_message_result(message_result).await?
            }
        },
        Err(error) => {
            error!("{:?}", error);
        }
    }

    Ok(())
}

async fn handle_message_result(message_result: Result<String, String>) -> Result<()> {
    match message_result {
        Ok(message) => {
            info!("{:?}", message);
            info!("Message processed, exiting ...");
        },
        Err(e) => {
            error!("Error {:?}", e);
            error!("Message wasn't processed, exiting ...");
        }
    }
    
    Ok(())
}
async fn handle_communication(tx: Sender<Result<String, String>>, address: String) -> Result<()> {
    if let Err(e) = is_valid_user() {
        return Err(anyhow!("{}", e));
    }

    let operation_type = input_utils::get_operation_type()?;
    
    let message = get_message_by_operation_type(operation_type)?;
    let mut stream = get_client(address.as_str()).await.unwrap();
    let (mut reader, mut writer) = stream.split();
    
    let handle_message = handle_message(tx, &mut reader);
    let send_message = message_utils::send_message(&mut writer, &message);
    
    let (handle_result, send_message_result) = tokio::join!(handle_message, send_message);
    
    handle_result?;
    send_message_result?;
    
    Ok(())
}

async fn get_client(address: &str) -> Result<TcpStream> {
    let stream = TcpStream::connect(address)
        .await
        .context("Failed to connect to server")?;

    Ok(stream)
}

async fn handle_message(tx: Sender<Result<String, String>>, mut stream: &mut tokio::net::tcp::ReadHalf<'_>) -> Result<()> {
    let message = message_utils::read_message(&mut stream).await;

    match message {
        Ok(Message::Image(file, content)) => {
            match file_utils::save_and_convert_image(&file, &content) {
                Ok(_) => tx.send(Ok("Image was processed".to_string())).unwrap(),
                Err(e) => tx.send(Err(e.to_string())).unwrap(),
            };
        }
        Ok(Message::File(file, content)) => match file_utils::save_file(&file, &content) {
            Ok(_) => tx.send(Ok("File was processed".to_string())).unwrap(),
            Err(e) => tx.send(Err(e.to_string())).unwrap(),
        },
        Ok(Message::Text(string)) => {
            info!("Text message received: {}", string);
            tx.send(Ok("Text message was sent".to_string())).unwrap();
        }
        Err(e) => tx.send(Err(e.to_string())).unwrap(),
    };

    Ok(())
}

fn get_message_by_operation_type(operation_type: InputOperationType) -> Result<Message> {
    match operation_type.operation {
        input_utils::Operation::Image => Ok(Message::Image(
            operation_type.data.clone(),
            file_utils::read_file_to_vec(&operation_type.data).unwrap(),
        )),
        input_utils::Operation::File => Ok(Message::File(
            operation_type.data.clone(),
            file_utils::read_file_to_vec(&operation_type.data).unwrap(),
        )),
        input_utils::Operation::Text => Ok(Message::Text(operation_type.data.clone())),
        input_utils::Operation::Quit => {
            info!("Exiting...");
            std::process::exit(1);
        }
    }
}

fn is_valid_user() -> Result<bool> {
    let user = input_utils::get_user()?;

    data_utils::get_user_by_name(user).context("Nonexistent user")?;

    Ok(true)
}
