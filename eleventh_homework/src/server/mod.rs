use anyhow::{Context, Result};
use data_utils::{get_connection, insert_message};
use log::{error, info};
use prometheus::Registry;
use rocket_dyn_templates::Template;
use message_utils;
use message_utils::Message;
use tokio::net::TcpListener;
use metric_utils::AppMetrics;
use crate::{routes, DbConn};
use std::sync::Arc;

pub async fn spawn_server(metrics: Arc<AppMetrics>) ->  tokio::task::JoinHandle<()> {
    let Ok(address) = input_utils::get_address() else {
        error!("Invalid address!");
        std::process::exit(1);
    };
    tokio::spawn(async move {
        match start_server(address, Arc::clone(&metrics)).await {
            Ok(message) => {
                info!("{:?}", message);
            },
            Err(e) => {
                error!("Error {:?}", e);
            }
        }
    })
}

pub async fn spawn_web_server(metrics: Arc<AppMetrics>, registry: Registry) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        if let Err(e) = rocket::build()
            .mount("/", routes::get_routes())
            .manage(registry)
            .manage(metrics)
            .attach(DbConn::fairing())
            .attach(Template::fairing())
            .launch()
            .await
        {
            error!("Rocket launch failed: {:?}", e);
        }
    })
}


async fn start_server(address: String, metrics: Arc<AppMetrics>) -> Result<()> {
    let listener = TcpListener::bind(&address).await?;

    info!("Listening on: {}", &address);

    loop {
        let (mut socket, addr) = match listener.accept().await {
            Ok((socket, addr)) => (socket, addr),
            Err(e) => {
                error!("Failed to accept connection: {}", e);
                continue;
            }
        };

        info!("Accepted connection from: {}", addr);

        let metrics = Arc::clone(&metrics);
        metrics.active_clients.inc();
        tokio::spawn(async move {
            loop {
                let (mut reader, mut writer) = socket.split();
                
                match handle_client(&mut reader, &mut writer, Arc::clone(&metrics)).await {
                    Ok(_) => {
                        info!("Client handled");
                    }
                    Err(e) => {
                        error!("Server error: {:?}", e);
                        metrics.active_clients.dec();
                        return;
                    }
                }
            }
        });
    }
}

async fn handle_client(
    mut reader: &mut tokio::net::tcp::ReadHalf<'_>,
    mut writer: &mut tokio::net::tcp::WriteHalf<'_>,
    metrics: Arc<AppMetrics>,
) -> Result<()> {
    let message = message_utils::read_message(&mut reader)
        .await
        .context("Failed to read message or connection closed")?; 

    info!("Received message");

    message_utils::send_message(&mut writer, &message)
        .await
        .context("Failed to send message")?;
    info!("Sending message to clients");

    save_message(message).context("Failed to save message")?;
    
    metrics.sent_messages.inc();

    Ok(())
}

fn save_message(message: Message) -> Result<()> {
    let mut connection = get_connection();
    
    match message {
        Message::Text(text, user_id) => {
            insert_message(text, "".to_string(), "".to_string(), user_id, &mut connection)?;
        }
        Message::Image(path, _, user_id) => {
            insert_message("".to_string(), "".to_string(), path.to_string(), user_id, &mut connection)?;
        }
        Message::File(path, _, user_id) => {
            insert_message("".to_string(), path.to_string(), "".to_string(), user_id, &mut connection)?;
        }
    };

    Ok(())
}
