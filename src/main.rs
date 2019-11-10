//! Hello World server in Rust
//! Binds REP socket to tcp://*:5555
//! Expects "Hello" from client, replies with "World"
extern crate rustyline;

use std::thread;
use std::time::Duration;

use std::env;

use rustyline::config::{Builder, ColorMode};
use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
    
    let args: Vec<String> = env::args().collect();

    match &args[1] as &str {
        
        "server" => run_server(),

        "client" => line(),

        _ => println!("use: cargo run -- [server or client]"),


    }
}

fn run_server() {
    let context = zmq::Context::new();
    let responder = context.socket(zmq::REP).unwrap();
    
    println!("Server is starting...");
    assert!(responder.bind("tcp://*:5555").is_ok());

   let mut msg = zmq::Message::new();
   loop {
       responder.recv(&mut msg, 0).unwrap();
       println!("Received: {}", msg.as_str().unwrap());
       thread::sleep(Duration::from_millis(1000));
       responder.send("Message recieved", 0).unwrap();
   }
   
}

fn setup_client() -> zmq::Socket {
    println!("Connecting to hello world server...\n");

    let context = zmq::Context::new();
    let requester = context.socket(zmq::REQ).unwrap();

        return requester

//    for request_nbr in 0..10 {
  //      println!("Sending Hello {}...", request_nbr);
    //    requester.send("Hello", 0).unwrap();

      //  requester.recv(&mut msg, 0).unwrap();
        //println!("Received World {}: {}", msg.as_str().unwrap(), request_nbr);
    //}
}

fn line() {
    let r = setup_client();
    
    assert!(r.connect("tcp://localhost:5555").is_ok());

    let mut msg = zmq::Message::new();

    let config_builder = Builder::new();
    let config = config_builder
        .color_mode(ColorMode::Enabled)
        .build();

    let mut rl = Editor::<()>::with_config(config);
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    let c = ">>";
    //let pb = ProgressBar::new(1024);

    loop {
        let p = format!("{}", c);
 //       rl.helper_mut().expect("No helper").colored_prompt = format!("\x1b[1;32m{}\x1b[0m", p);
        let readline = rl.readline(&p );
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                r.send(line.as_str(), 0).unwrap();

                r.recv(&mut msg, 0).unwrap();

                println!("{}", msg.as_str().unwrap());
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    rl.save_history("history.txt").unwrap();
}
