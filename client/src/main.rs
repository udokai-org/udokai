use tokio::net::UnixStream;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, stdin};
use serde_json::json;
use std::fs;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    shared::setup_logger("/tmp/client.log")
        .expect("Failed to setup logger on client");

    // List of server socket paths
    let server_sockets = vec![
        "/tmp/server1.sock",
        // "/tmp/server2.sock",
        // Add more server paths as needed
    ];

    // Connect to all servers
    let mut connections = vec![];
    for server_socket in server_sockets {
        if fs::metadata(server_socket).is_ok() {
            let path = server_socket.to_string();
            if let Ok(stream) = UnixStream::connect(&path).await {
                connections.push((path, stream));
            } else {
                log::info!("Failed to connect to server: {}", path);
            }
        } else {
            log::info!("Server socket not found: {}", server_socket);
        }
    }

    if connections.is_empty() {
        log::info!("No servers available. Exiting.");
        return Ok(());
    }

    log::info!("Client is ready. Send input via stdin.");

    // Read input from stdin
    let stdin = stdin();
    let mut stdin_reader = BufReader::new(stdin);
    let mut buffer = String::new();

    loop {
        buffer.clear();
        let bytes_read = stdin_reader.read_line(&mut buffer).await?;
        if bytes_read == 0 {
            break; // End of input
        }

        log::info!("@@@@@@@@@ buffer {:?}", buffer);

        let input = buffer.trim();
        if input.is_empty() {
            continue; // Ignore empty lines
        }

        // Send the input to all servers
        let event = json!({
            "event": "user_input",
            "data": input,
        });

        for (path, stream) in &mut connections {
            if let Err(e) = send_event_to_server(stream, event.clone()).await {
                log::info!("Error communicating with server {}: {}", path, e);
            }
        }
    }

    log::info!("Client exiting...");
    Ok(())
}

// Function to communicate with a single server
async fn send_event_to_server(stream: &mut UnixStream, event: serde_json::Value) -> std::io::Result<()> {
    let (reader, mut writer) = stream.split();
    let mut reader = BufReader::new(reader);

    // Send the user input event to the server
    writer.write_all(event.to_string().as_bytes()).await?;
    writer.write_all(b"\n").await?;
    writer.flush().await?;
    log::info!("Sent to server: {}", event);

    // Read the server's response
    let mut response = String::new();
    reader.read_line(&mut response).await?;
    log::info!("Received from server: {}", response.trim());

    Ok(())
}

