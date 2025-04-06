use flume;
use input_utils;
use log::{error, info};
use crate::client::handle_client_by_mode;
#[macro_use] extern crate rocket;  
use rocket_sync_db_pools::{database, diesel}; 
use diesel::sqlite::SqliteConnection;
use tokio::join;
use tokio::signal;
use metric_utils::{AppMetrics};
use prometheus::{Registry}; 
use std::sync::Arc;

#[database("sqlite_db")]
pub struct DbConn(SqliteConnection);


mod client;
mod server;
mod routes;

#[tokio::main]
async fn main()  {
    init_logger();

    let (tx, rx) = flume::unbounded::<Result<String, String>>();

    match input_utils::get_mode() {
        Ok(input_utils::Mode::Client) => { 
            if let Err(e) = handle_client_by_mode(tx, rx).await {
                error!("Error {:?}", e);
            }
        },
        Ok(input_utils::Mode::Server) => {
            let registry = Registry::new();
            let metrics = Arc::new(AppMetrics::initialize(&registry));
            let web_server = server::spawn_web_server(Arc::clone(&metrics), registry);
            let tcp_server = server::spawn_server(Arc::clone(&metrics));
            let (web_result, tcp_result) = join!(tcp_server, web_server);

            loop {
                if let Ok(_) = signal::ctrl_c().await {
                    info!("Ctrl+C received, shutting down...");

                    web_result.abort();
                    tcp_result.abort();

                    std::process::exit(0);
                }
            }
        },
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