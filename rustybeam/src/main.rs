use std::env::args;


use rustybeam::Server;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 3 {
        eprintln!("Usage: cargo run <IP> <PORT>");
        std::process::exit(1);
    }

    let ip = &args[1];
    let port = &args[2];

    let mut rusty_beam = Server::new(ip, port);

    rusty_beam.expect("REASON").start().expect("TODO: panic message");


}
