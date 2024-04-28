use std::env::args;
use rustybeam::LoadBalancer;

fn main() {


    let args: Vec<String> = args().collect();
    if args.len() != 3 {
        eprintln!("Usage: cargo run <IP> <PORT>");
        std::process::exit(1);
    }

    let ip = &args[1];
    let port = &args[2];

    let rusty_beam = LoadBalancer::new(ip, port);

    rusty_beam.expect("REASON").start().expect("TODO: panic message");


}
