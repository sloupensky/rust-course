use flume;
use input_utils;
use log::{info, error};
mod client;
mod server;

fn main()  {
    init_logger();

    let Ok(address) = input_utils::get_address() else {
        error!("Invalid address!");
        std::process::exit(1);
    };
    let (tx, rx) = flume::unbounded::<Result<String, String>>();

    match input_utils::get_mode() {
        Ok(input_utils::Mode::Client) => {
            match client::handle_client(tx, address) {
                Ok(_) => {
                    while let Ok(message_result) = rx.recv() {
                        match message_result {
                            Ok(message) => {
                                info!("{}", message);
                                info!("Message processed, exiting ...");
                            },
                            Err(e) => {
                                error!("Error {}", e);
                                error!("Message wasn't processed, exiting ...");
                            }
                        }
                    }
                },
                Err(error) => {
                    error!("{}", error);
                }
            }
        },
        Ok(input_utils::Mode::Server) => {
            match server::start_server(address.as_str()) {
                Ok(message) => {
                    info!("{:?}", message);
                },
                Err(e) => {
                    error!("Error {}", e);
                }
            };
        }
        _ => {}
    }
}

fn init_logger() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();
}