use flume;
use input_utils;
use log::{error, info};
use crate::client::handle_client_by_mode;

mod client;
mod server;

#[tokio::main]
async fn main()  {
    init_logger();

    let Ok(address) = input_utils::get_address() else {
        error!("Invalid address!");
        std::process::exit(1);
    };
    let (tx, rx) = flume::unbounded::<Result<String, String>>();

    match input_utils::get_mode() {
        Ok(input_utils::Mode::Client) => {
            if let Err(e) = handle_client_by_mode(tx, rx, address).await {
                error!("Error {:?}", e);
            }
        },
        Ok(input_utils::Mode::Server) => {
            match server::start_server(address.as_str()).await {
                Ok(message) => {
                    info!("{:?}", message);
                },
                Err(e) => {
                    error!("Error {:?}", e);
                }
            };
        }
        Err(e) => {
            error!("{:?}", e);
        }
    }
}

fn init_logger() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();
}