use std::env::args;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

mod server;
use server::LoadBalancer;
use server::RoundRobin;


fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 3 {
        eprintln!("Usage: cargo run <IP> <PORT>");
        std::process::exit(1);
    }

    let ip = &args[1];
    let port = &args[2];

    let mut rusty_beam = LoadBalancer::new(ip, port);
    let mut round_robin = RoundRobin::new();

    round_robin.add_server(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 1337));
    

    rusty_beam.expect("REASON").start().expect("TODO: panic message");

}