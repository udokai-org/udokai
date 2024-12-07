use actix::{Actor, ActorContext, Context, Handler, Message, System};

mod process_actor;
mod supervisor;

#[derive(Message, Debug)]
#[rtype(result = "()")]
struct Die;

#[derive(Message, Debug)]
#[rtype(result = "()")]
struct Hello;

struct MyActor;

impl Actor for MyActor {
    type Context = Context<Self>;
}

// To use actor with supervisor actor has to implement `Supervised` trait
impl actix::Supervised for MyActor {
    fn restarting(&mut self, _ctx: &mut Context<MyActor>) {
        println!("restarting");

    }
}

impl Handler<Die> for MyActor {
    type Result = ();

    fn handle(&mut self, die: Die, ctx: &mut Context<MyActor>) {
        println!("@@@@@@@@@ die {:?}", die);
        ctx.stop();
    }
}

impl Handler<Hello> for MyActor {
    type Result = ();

    fn handle(&mut self, hello: Hello, _ctx: &mut Context<MyActor>) {
        println!("@@@@@@@@@ hello {:?}", hello);
    }
}

pub fn start() {
    let sys = System::new();

    let addr = sys.block_on(async { actix::Supervisor::start(|_| MyActor) });
    addr.do_send(Hello);
    addr.do_send(Hello);
    addr.do_send(Hello);

    // Loop in another thread
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_secs(5));
        addr.do_send(Die);
    });

    match sys.run() {
        Ok(_) => println!("System shutdown"),
        Err(_) => println!("System error"),
    }
}

#[actix::main]
pub async fn main() {
    let supervisor = supervisor::Supervisor::new().start();

    // Start a process
    let process_id = supervisor
        .send(supervisor::StartProcess {
            command: "your_program".to_string(),
        })
        .await
        .unwrap();

    println!("Started process with ID: {}", process_id);

    // Supervisor will automatically restart the process if it fails
}
