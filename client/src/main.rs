use tokio::net::UnixStream;
use tokio::io::{AsyncWriteExt, AsyncBufReadExt, BufReader};
use std::fs;

#[tokio::main]
pub async fn main() -> std::io::Result<()> {
    // List of server socket paths
    let server_sockets = vec![
        "/tmp/server1.sock",
        // "/tmp/server2.sock",
        // Add more paths as needed
    ];

    let mut tasks = vec![];

    for socket_path in server_sockets {
        // Check if the socket exists before trying to connect
        if fs::metadata(socket_path).is_ok() {
            let path = socket_path.to_string();
            tasks.push(tokio::spawn(async move {
                query_server(&path).await
            }));
        }
    }

    // Collect all responses
    let results = futures::future::join_all(tasks).await;

    for (i, result) in results.iter().enumerate() {
        match result {
            Ok(response) => println!("CLIENT: Server {} responded: {:?}", i + 1, response),
            Err(e) => eprintln!("CLIENT: Error with server {}: {}", i + 1, e),
        }
    }

    Ok(())
}

async fn query_server(socket_path: &str) -> std::io::Result<String> {
    let stream = UnixStream::connect(socket_path).await?;
    let (reader, mut writer) = stream.into_split();

    // Send a trigger
    writer.write_all(b"trigger_query\n").await?;
    writer.flush().await?;

    // Read the response
    let mut reader = BufReader::new(reader);
    let mut response = String::new();
    reader.read_line(&mut response).await?;
    Ok(response.trim().to_string())
}
