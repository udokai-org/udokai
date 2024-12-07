use actix::prelude::*;
use std::{collections::HashMap, time::Duration};

use crate::process_actor::{ProcessActor, ProcessExited};

pub struct Supervisor {
    children: HashMap<usize, Addr<ProcessActor>>,
    commands: HashMap<usize, String>, // Track commands for restarting
    next_id: usize,
}

impl Supervisor {
    pub fn new() -> Self {
        Supervisor {
            children: HashMap::new(),
            commands: HashMap::new(),
            next_id: 0,
        }
    }
}

impl Actor for Supervisor {
    type Context = Context<Self>;
}

pub struct StartProcess {
    pub command: String,
}

impl Message for StartProcess {
    type Result = usize; // Process ID
}

impl Handler<StartProcess> for Supervisor {
    type Result = usize;

    fn handle(&mut self, msg: StartProcess, ctx: &mut Context<Self>) -> Self::Result {
        let process_id = self.next_id;
        self.next_id += 1;

        let command = msg.command.clone();
        let process_actor = ProcessActor::new(process_id, command.clone(), ctx.address()).start();

        self.children.insert(process_id, process_actor);
        self.commands.insert(process_id, msg.command);

        process_id
    }
}

impl Handler<ProcessExited> for Supervisor {
    type Result = ();

    fn handle(&mut self, msg: ProcessExited, ctx: &mut Context<Self>) -> Self::Result {
        println!("Supervisor: Process {} exited.", msg.process_id);

        // Remove the exited process from tracking
        self.children.remove(&msg.process_id);

        // Restart the process
        if let Some(command) = self.commands.get(&msg.process_id).cloned() {
            println!("Supervisor: Restarting process {}", msg.process_id);

            // Restart after a short delay (optional)
            ctx.notify_later(StartProcess { command }, Duration::from_secs(1));
        }
    }
}

