// use gui::show;
// use guiced::show;

fn main() {
    // run client binary
    let mut client = std::process::Command::new("target/debug/client")
        .spawn()
        .expect("Failed to run client binary");
    // loop for 5 seconds and kill it
    let mut server = std::process::Command::new("target/debug/server")
        .spawn()
        .expect("Failed to run server binary");
    std::thread::sleep(std::time::Duration::from_secs(5));
    client.kill().expect("Failed to kill client binary");
    server.kill().expect("Failed to kill server binary");
    // show().expect("Failed to show GUI");
    println!("Hello, world!");
}
