use anyhow::{Context, Result};
use data_utils::{get_connection, insert_message};
use log::{error, info};
use message_utils;
use message_utils::Message;
use tokio::net::TcpListener;

pub async fn start_server() -> Result<()> {
    let Ok(address) = input_utils::get_address() else {
        error!("Invalid address!");
        std::process::exit(1);
    };
    
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

        tokio::spawn(async move {
            loop {
                let (mut reader, mut writer) = socket.split();
                
                match handle_client(&mut reader, &mut writer).await {
                    Ok(_) => {
                        info!("Client handled");
                    }
                    Err(e) => {
                        error!("Server error: {:?}", e);
                        return;
                    }
                }
            }
        });
    }
}

async fn handle_client(
    mut reader: &mut tokio::net::tcp::ReadHalf<'_>,
    mut writer: &mut tokio::net::tcp::WriteHalf<'_>
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
