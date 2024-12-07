use actix::prelude::*;
use tokio::process::{Command, Child};
use tokio::io::{AsyncBufReadExt, BufReader};
use std::process::Stdio;

use crate::supervisor::Supervisor;

// Message sent to Supervisor when a subprocess exits
pub struct ProcessExited {
    pub process_id: usize,
}

impl Message for ProcessExited {
    type Result = ();
}

pub struct ProcessActor {
    pub process_id: usize,
    pub command: String,
    pub supervisor: Addr<Supervisor>,
}

impl ProcessActor {
    pub fn new(process_id: usize, command: String, supervisor: Addr<Supervisor>) -> Self {
        ProcessActor {
            process_id,
            command,
            supervisor,
        }
    }
}

impl Actor for ProcessActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let command = self.command.clone();
        let supervisor = self.supervisor.clone();
        let process_id = self.process_id;

        let process = async move {
            let mut child = Command::new(command)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .expect("Failed to spawn process");

            // Monitor the process output
            if let Some(stdout) = child.stdout.take() {
                let mut reader = BufReader::new(stdout);
                let mut buffer = String::new();
                while let Ok(bytes_read) = reader.read_line(&mut buffer).await {
                    if bytes_read == 0 {
                        break; // EOF
                    }
                    println!("Process {} output: {}", process_id, buffer.trim());
                    buffer.clear();
                }
            }

            // Wait for the child process to exit
            let exit_status = child.wait().await.expect("Failed to wait on child process");
            println!("Process {} exited with status: {:?}", process_id, exit_status);

            // Notify the Supervisor of the exit
            supervisor.do_send(ProcessExited { process_id });
        };

        ctx.spawn(actix::fut::wrap_future(process));
    }
}

