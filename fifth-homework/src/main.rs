use crate::input::Mode; 
use flume;

mod messaging;
mod input;
mod file;

fn main()  {
    let Ok(address) = input::get_address() else {
        panic!("Invalid address")
    };
    let (tx, rx) = flume::unbounded::<Result<String, String>>();
    match input::get_mode() {
        Ok(Mode::Client) => {
            let client = match messaging::get_client(address.as_str()) {
                Ok(client) => client,
                Err(e) => {
                    eprintln!("Error {}", e);
                    return
                }
            };
            let operation_result = input::get_operation_type();
            
            match operation_result {
                Ok(operation_type) => {
                    let message = match operation_type.operation {
                        input::Operation::Image => {
                            messaging::Message::Image(operation_type.data.clone(), file::read_file_to_vec(&operation_type.data).unwrap())
                        }
                        input::Operation::File => {
                            messaging::Message::File(operation_type.data.clone(), file::read_file_to_vec(&operation_type.data).unwrap())
                        }
                        input::Operation::Text => {
                            messaging::Message::Text(operation_type.data.clone())
                        }
                        input::Operation::Quit => {
                            println!("Exiting...");
                            std::process::exit(1);
                        }
                    };
                    messaging::handle_message(tx, client.try_clone().unwrap()).unwrap();
                    messaging::send_message(client, &message).unwrap();
                    
                    while let Ok(message_result) = rx.recv() {
                        match message_result {
                            Ok(message) => {
                                println!("{}", message);
                                println!("Message processed, exiting ...");
                            },
                            Err(e) => {
                                eprintln!("Error {}", e);
                                eprintln!("Message wasn't processed, exiting ...");
                            }
                        }
                    }
                    
                },
                Err(error) => {
                    eprintln!("{}", error);
                }
            }
        },
        Ok(Mode::Server) => {
            match messaging::start_server(address.as_str()) {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("Error {}", e);
                }
            };
        }
        _ => {}
    }
}