use crate::input::Mode; 
use flume;

mod messaging;
mod input;
mod file;

fn main()  { 
    let Ok(address) = input::get_address() else {
        eprintln!("Invalid address!");
        std::process::exit(1);
    };
    let (tx, rx) = flume::unbounded::<Result<String, String>>();

    match input::get_mode() {
        Ok(Mode::Client) => {
            match messaging::handle_client(tx, address) {
                Ok(_) => {
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
                Ok(message) => {
                    println!("{:?}", message);
                },
                Err(e) => {
                    eprintln!("Error {}", e);
                }
            };
        }
        _ => {}
    }
}