use std::env::args;

mod server;
use server::server_mod::Server;





fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 3 {
        eprintln!("Usage: cargo run <IP> <PORT>");
        std::process::exit(1);
    }

    let ip = &args[1];
    let port = &args[2];

    let rusty_beam = Server::new(ip, port);

    // lauch the server

}
