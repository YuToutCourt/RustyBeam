use std::env::args;

mod server;
use server::Server;
use server::RoundRobin;


fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 3 {
        eprintln!("Usage: cargo run <IP> <PORT>");
        std::process::exit(1);
    }

    let ip = &args[1];
    let port = &args[2];

    let mut rusty_beam = Server::new(ip, port);
    let mut round_robin = RoundRobin::new();


    let server1 = Server::new("127.0.0.1", "1337");

    rusty_beam.expect("REASON").start().expect("TODO: panic message");

}