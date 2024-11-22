use tokio::net::{UnixListener, UnixStream};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use serde_json::{Value, json};
use std::fs;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    shared::setup_logger("/tmp/server1.log")
        .expect("Failed to setup logger on server");

    let socket_path = "/tmp/server1.sock";

    // Clean up old socket file if it exists
    if fs::metadata(socket_path).is_ok() {
        fs::remove_file(socket_path)?;
    }

    let listener = UnixListener::bind(socket_path)?;

    log::info!("Server listening on {}", socket_path);

    loop {
        let (stream, _) = listener.accept().await?;
        tokio::spawn(handle_client(stream));
    }
}

async fn handle_client(stream: UnixStream) -> std::io::Result<()> {
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);
    let mut buffer = String::new();

    log::info!("New client connected!");

    loop {
        buffer.clear();

        // Read a message from the client
        let bytes_read = reader.read_line(&mut buffer).await?;
        if bytes_read == 0 {
            // Client disconnected
            log::info!("Client disconnected.");
            break;
        }

        // Parse the JSON message
        match serde_json::from_str::<Value>(&buffer.trim()) {
            Ok(message) => {
                log::info!("Received from client: {}", message);

                // Create a response
                let response = json!({
                    "event": "server_response",
                    "data": format!("Hello, client! Received: {}", message["event"]),
                });

                // Send the response back
                writer.write_all(response.to_string().as_bytes()).await?;
                writer.write_all(b"\n").await?;
                writer.flush().await?;
            }
            Err(e) => {
                log::error!("Failed to parse message: {}", e);
            }
        }
    }

    Ok(())
}

