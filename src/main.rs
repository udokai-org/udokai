// use gui::show;
// use guiced::show;
use tui;

use std::{io::{BufRead, BufReader, Write}, process::Stdio, sync::{Arc, Mutex}, thread};
use std::process::{Command};

fn main() -> std::io::Result<()> {
    shared::setup_logger("/tmp/main.log")
        .expect("Failed to setup logger on main");
    log::info!("-----------------------");

    let mut server = std::process::Command::new("target/debug/server")
        .spawn()
        .expect("Failed to run server binary");

    // TODO: wait for server confirmation before starting client
    std::thread::sleep(std::time::Duration::from_secs(1));
    // Spawn the client process
    let mut client = std::process::Command::new("target/debug/client")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn client process");

    let client_stdin = client.stdin.as_mut().expect("Failed to open client stdin");
    let client_stdout = client.stdout.take().expect("Failed to open client stdout");

    // Write input to the client's stdin
    let inputs = vec!["hello", "status", "foo"];
    for input in inputs {
        writeln!(client_stdin, "{}", input)?; // Send input to client
        client_stdin.flush()?;
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    // Close the stdin stream to signal end of input
    // let _ = drop(client_stdin);

    // Read and print output from the client's stdout
    let stdout_reader = BufReader::new(client_stdout);
    for line in stdout_reader.lines() {
        log::info!(">>> {}", line?);
    }

    server.kill()?;
    client.kill()?;

    Ok(())
}



fn __main() -> std::io::Result<()> {
    shared::setup_logger("/tmp/main.log")
        .expect("Failed to setup logger on main");

    let mut server = std::process::Command::new("target/debug/server")
        .spawn()
        .expect("Failed to run server binary");

    // TODO: wait for server confirmation before starting client
    std::thread::sleep(std::time::Duration::from_secs(1));

    // run client binary
    let mut client = std::process::Command::new("target/debug/client")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run client binary");

    // loop for 5 seconds and kill it
    // Get handles to the child's stdin and stdout
    let client_stdin = match client.stdin.take() {
        Some(stdin) => stdin,
        _ => {
            log::error!("Failed to open stdin");
            std::process::exit(1);
        }
    };

    let mut client_stdout = client.stdout.take().expect("Failed to open stdout");
    let mut reader = BufReader::new(&mut client_stdout);
    log::info!("Client stdout: {:?}", reader);
    if let Some(response) = read_from_client(&mut reader) {
        log::info!(">>>> Client responded: {}", response);
    }


    let boxed_client_stdin = Arc::new(Mutex::new(client_stdin));
    let boxed_client_stdout = Arc::new(Mutex::new(client_stdout));

    let message_tests = vec!["Hello", "World", "How", "Are", "You"];

    for message in message_tests {
        log::info!("sending msg test to client: {}", message);
        let mut cli_stdin = boxed_client_stdin.lock().unwrap();
        writeln!(cli_stdin, "{}", message).expect("Failed to write to client");
        cli_stdin.flush().expect("Failed to flush client");

        // Read the client's stdout
        let mut client_stdout = match boxed_client_stdout.lock() {
            Ok(stdout) => stdout,
            Err(e) => {
                log::error!("Failed to lock stdout: {}", e);
                return Ok(());
            }
        };
        let mut reader = BufReader::new(&mut *client_stdout);
        log::info!(">> Reading from client {:?}", reader);
        if let Some(response) = read_from_client(&mut reader) {
            log::info!(">>> Client responded: {}", response);
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
        log::info!("@@@@@");
    }

    // match tui::show(move |input| {
    //     log::info!("Sent to client: {}", input);
    //     let mut cli_stdin = boxed_client_stdin.lock().unwrap();
    //     writeln!(cli_stdin, "{}", input).expect("Failed to write to client");
    //     cli_stdin.flush().expect("Failed to flush client");
    //
    //     // Close the stdin stream to signal end of input
    //     // let _ = drop(client_stdin);
    //
    //     // Read the client's stdout
    // }, move |messages: Vec<String>| {
    //     log::info!("Handling message");
    //     let mut client_stdout = match boxed_client_stdout.lock() {
    //         Ok(stdout) => stdout,
    //         Err(e) => {
    //             log::error!("Failed to lock stdout: {}", e);
    //             return vec![];
    //         }
    //     };
    //     log::info!("@@@@@ unlocked");
    //     let mut reader = BufReader::new(&mut *client_stdout);
    //     let mut msgs = messages.clone();
    //     log::info!("@@@@@ reading from client {:?}", msgs);
    //
    //     if let Some(response) = read_from_client(&mut reader) {
    //         log::info!("Client responded: {}", response);
    //         msgs.push(response);
    //     }
    //
    //     log::info!("@@@@@ returning {:?}", msgs);
    //
    //     // for line in reader.lines() {
    //     //     let data = line.expect("Failed to read line");
    //     //     log::info!("line: {}", data);
    //     //     msgs.push(data);
    //     // }
    //     //
    //     msgs
    // }) {
    //     Ok(_) => {}
    //     Err(e) => {
    //         log::error!("Error: {}", e);
    //     }
    // }

    std::thread::sleep(std::time::Duration::from_secs(10));

    // Wait for the client process to exit
    // let status = client.wait()?;
    // log::info!("Client exited with status: {}", status);
    //
    // client.kill()?;
    server.kill()?;

    Ok(())
}

fn read_from_client(reader: &mut BufReader<&mut std::process::ChildStdout>) -> Option<String> {
    let mut response = String::new();
    log::info!("attempt to read_line from client {:?}", response);
    if response.is_empty() {
        log::info!("response is empty");
        return None;
    }

    if let Ok(bytes) = reader.read_line(&mut response) {
        if bytes > 0 {
            return Some(response.trim().to_string());
        }
    }

    log::info!("No response from client");

    None
}
