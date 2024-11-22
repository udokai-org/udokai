// use gui::show;
// use guiced::show;
use tui;

use std::{io::{BufRead, BufReader, Write}, process::Stdio, sync::{Arc, Mutex}};

fn main() -> std::io::Result<()> {
    // run client binary
    let mut client = std::process::Command::new("target/debug/client")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to run client binary");
    // loop for 5 seconds and kill it
    let _server = std::process::Command::new("target/debug/server")
        .spawn()
        .expect("Failed to run server binary");

    // Get handles to the child's stdin and stdout
    let client_stdin = match client.stdin.take() {
        Some(stdin) => stdin,
        None => {
            eprintln!("Failed to open stdin");
            std::process::exit(1);
        }
    };

    let boxed_client_stdin = Arc::new(Mutex::new(client_stdin));
    match tui::show(move |input| {
        println!("Sent to client: {}", input);
        let mut cli_stdin = boxed_client_stdin.lock().unwrap();
        writeln!(cli_stdin, "{}", input).expect("Failed to write to client");
    }) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    std::thread::sleep(std::time::Duration::from_secs(10));

    let client_stdout = client.stdout.take().expect("Failed to open stdout");
    // Close the stdin stream to signal end of input
    // let _ = drop(client_stdin);

    // Read the client's stdout
    let stdout_reader = BufReader::new(client_stdout);
    for line in stdout_reader.lines() {
        let line = line?;
        println!("Received from client: {}", line);
    }

    // Wait for the client process to exit
    // let status = client.wait()?;
    // println!("Client exited with status: {}", status);
    //
    // client.kill()?;
    // server.kill()?;

    Ok(())
}
