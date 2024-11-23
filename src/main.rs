// use gui::show;
// use guiced::show;
use tui;

use std::{io::{BufRead, BufReader, Write}, process::Stdio, sync::{Arc, Mutex}, thread};
use std::process::{Command};

fn main() -> std::io::Result<()> {
    shared::setup_logger("/tmp/main.log")
        .expect("Failed to setup logger on main");
    log::info!("-----------------------");

    // Server is spawned as a separate process because it is not connected
    // to the client directly. The client connects to the server via UnixSocket.
    let mut server = std::process::Command::new("target/debug/server")
        .spawn()
        .expect("Failed to run server binary");

    // TODO: wait for server confirmation before starting client effectively
    std::thread::sleep(std::time::Duration::from_secs(1));

    // Spawn the client process to start listening to user input in UI
    let mut client = std::process::Command::new("target/debug/client")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn client process");

    let client_stdin = Arc::new(Mutex::new(client.stdin));
    let client_stdout = client.stdout.take().expect("Failed to open stdout");
    let mutex_reader = Arc::new(Mutex::new(BufReader::new(client_stdout)));


    match tui::show(move |input| {
        log::info!("Sent to client: {}", input);
        let mut tclient = client_stdin.lock().unwrap();
        let stdin = match tclient.as_mut() {
            Some(stdin) => stdin,
            _ => {
                log::error!("Failed to open stdin");
                return vec![];
            }
        };
        match writeln!(stdin, "{}", input) {
            Ok(_) => {}
            Err(e) => {
                log::error!("Failed to write to client: {}", e);
            }
        }
        stdin.flush().expect("Failed to flush client");

        let mut reader = mutex_reader.lock().unwrap();

        let mut msgs = vec![];
        if let Some(response) = read_from_client(&mut *reader) {
            log::info!("Client responded: {}", response);
            if response == "exit" {
                log::info!("Client exited");
                std::process::exit(0);
            }

            if response.is_empty() {
                return vec![];
            }

            msgs.push(response);
        }

        log::info!("@@@@@ returning {:?}", msgs);

        msgs
    }) {
        Ok(_) => {}
        Err(e) => {
            log::error!("Error: {}", e);
        }
    }

    // TODO kill client
    // if let Err(e) = client.kill() {
    //     log::error!("Failed to kill client: {}", e);
    // }

    if let Err(e) = server.kill() {
        log::error!("Failed to kill server: {}", e);
    }

    Ok(())
}

fn read_from_client(reader: &mut BufReader<std::process::ChildStdout>) -> Option<String> {
    let mut response = String::new();
    log::info!("attempt to read_line from client {:?}", response);

    if let Ok(bytes) = reader.read_line(&mut response) {
        if bytes > 0 {
            return Some(response.trim().to_string());
        }
    }

    log::info!("No response from client");

    None
}
