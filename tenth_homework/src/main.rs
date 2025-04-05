use flume;
use input_utils;
use log::{error, info};
use crate::client::handle_client_by_mode;
#[macro_use] extern crate rocket;  
use rocket_sync_db_pools::{database, diesel}; 
use diesel::sqlite::SqliteConnection; 
use rocket_dyn_templates::Template;

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
            match server::start_server().await {
                Ok(message) => {
                    info!("{:?}", message);
                },
                Err(e) => {
                    error!("Error {:?}", e);
                }
            };
        },
        Ok(input_utils::Mode::Web) => {
            let rocket = rocket::build()
                .mount("/", routes::get_routes())
                .attach(DbConn::fairing())
                .attach(Template::fairing());

            match rocket.ignite().await.unwrap().launch().await {
                Ok(rocket) => {
                    info!("{:?}", rocket);
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