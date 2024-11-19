use tokio::net::UnixListener;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use std::fs;

#[tokio::main]
pub async fn main() -> std::io::Result<()> {
    let socket_path = "/tmp/server1.sock";

    // Clean up the socket file if it exists
    if fs::metadata(socket_path).is_ok() {
        fs::remove_file(socket_path)?;
    }

    let listener = UnixListener::bind(socket_path)?;

    println!("SERVER: Server listening on {}", socket_path);

    loop {
        let (stream, _) = listener.accept().await?;
        tokio::spawn(handle_client(stream));
    }
}

async fn handle_client(stream: tokio::net::UnixStream) -> std::io::Result<()> {
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);

    let mut buffer = String::new();
    reader.read_line(&mut buffer).await?;
    let trigger = buffer.trim();

    println!("SERVER: Received trigger: {}", trigger);

    // Respond with a list of items
    let response = format!("Server1: first to '{}'\n", trigger);
    writer.write_all(response.as_bytes()).await?;
    // writer.flush().await?;

    let response2 = format!("Server1: second response to '{}'\n", trigger);
    writer.write_all(response2.as_bytes()).await?;
    writer.flush().await?;

    Ok(())
}
