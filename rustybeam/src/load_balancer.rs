use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::io::Result;
use std::thread;

use crate::round_robin::RoundRobin;


pub struct LoadBalancer<'a> {
    ip: &'a str,
    port: &'a str,
    listener: TcpListener,
    timeout: u64, // in seconds
}

impl<'a> LoadBalancer<'a> {
    pub fn new(ip: &'a str, port: &'a str) -> LoadBalancer<'a> {
        let address = format!("{}:{}", ip, port);

        let listener = TcpListener::bind(address).expect("Could not bind");
        println!("Server listening on {}:{}", ip, port);
    
        LoadBalancer {
            ip,
            port,
            listener,
            timeout: 10,
        }
    }


    pub fn start(&mut self) -> Result<()> {
        let listener = self.listener.try_clone().expect("Could not clone listener");

        let mut round_robin = RoundRobin::new();

        let handle_client_thread = move |stream: TcpStream| {
            thread::spawn(move || {
                LoadBalancer::handle_client(stream, &mut round_robin);
            });
        };

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    handle_client_thread(stream);
                }
                Err(e) => {
                    eprintln!("An error occurred while accepting a connection: {}", e);
                }
            }
        }

        Ok(())
    }


    fn handle_client(mut stream: TcpStream, round_robin: &mut RoundRobin) {
        let mut buffer = [0; 1024];
        while match stream.read(&mut buffer) {
            Ok(size) => {
                if size > 0 {
    
                    let message = String::from_utf8_lossy(&buffer[..size]);

                    // Get the next server in the list and make a request to it with the message
                    let server = round_robin.next();
                    let response = server.make_request(&message).expect("Could not make request");

                    // Send the response back to the client
                    stream.write_all(response).expect("Could not write to stream");




                }
                true
            },
            Err(_) => {
                println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
                stream.shutdown(std::net::Shutdown::Both).unwrap();
                false
            }
        } {}
    }

}