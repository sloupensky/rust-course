use std::error::Error; 
use std::net::TcpStream; 
use std::{thread}; 
use flume::{Sender}; 
use message_utils;
use message_utils::Message;
use input_utils;
use input_utils::InputOperationType;
use file_utils;
use log::{info};

pub fn handle_client(tx: Sender<Result<String, String>>, address: String) -> Result<(), Box<dyn Error>> {
    let client = match get_client(address.as_str()) {
        Ok(client) => client,
        Err(e) => {
            return Err(e);
        }
    };
    let operation_result = input_utils::get_operation_type();

    match operation_result {
        Ok(operation_type) => {
            let message = get_message_by_operation_type(operation_type)?;
            handle_message(tx, client.try_clone().unwrap())?;
            message_utils::send_message(client, &message)?;
            
            Ok(())
        },
        Err(error) => {
            Err(error)
        }
    }
}


fn get_client(address: &str) -> Result<TcpStream, Box<dyn Error>> {
    let stream = TcpStream::connect(address)?;

    Ok(stream)
}

fn handle_message(
    tx: Sender<Result<String, String>>,
    stream: TcpStream,
) -> Result<(), Box<dyn Error>> {
    let thread = thread::Builder::new()
        .name("client-listening-thread".into())
        .spawn(move || {
            let message = message_utils::read_message(&stream);

            match message {
                Ok(Message::Image(file, content)) => {
                    match file_utils::save_and_convert_image(&file, &content) {
                        Ok(_) => tx.send(Ok("Image was processsed".to_string())).unwrap(),
                        Err(e) => tx.send(Err(e.to_string())).unwrap(),
                    };
                }
                Ok(Message::File(file, content)) => match file_utils::save_file(&file, &content) {
                    Ok(_) => tx.send(Ok("File was processsed".to_string())).unwrap(),
                    Err(e) => tx.send(Err(e.to_string())).unwrap(),
                },
                Ok(Message::Text(string)) => {
                    info!("Text message received: {}", string);
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

fn get_message_by_operation_type(operation_type: InputOperationType) -> Result<Message, Box<dyn Error>> {
    match operation_type.operation {
        input_utils::Operation::Image => {
            Ok(Message::Image(operation_type.data.clone(), file_utils::read_file_to_vec(&operation_type.data).unwrap()))
        }
        input_utils::Operation::File => {
            Ok(Message::File(operation_type.data.clone(), file_utils::read_file_to_vec(&operation_type.data).unwrap()))
        }
        input_utils::Operation::Text => {
            Ok(Message::Text(operation_type.data.clone()))
        }
        input_utils::Operation::Quit => {
            info!("Exiting...");
            std::process::exit(1);
        }
    }
}
