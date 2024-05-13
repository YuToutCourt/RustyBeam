/*
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::io::{Read, Write};
use std::time::Duration;
use std::io::{Result};
use std::thread;

pub struct Server<'a> {
    ip: &'a str,
    port: &'a str,
    listener: TcpListener,
    timeout: u64, // in seconds
}

impl<'a> Server<'a> {
    pub fn new(ip: &'a str, port: &'a str) -> Result<Server<'a>> {
        let address = format!("{}:{}", ip, port);

        let listener = TcpListener::bind(address).expect("Could not bind");
        println!("Server listening on {}:{}", ip, port);
    
        Ok(Server {
            ip,
            port,
            listener,
            timeout: 10,
        })
    }


    pub fn start(&mut self) -> Result<()> {
        let listener = self.listener.try_clone().expect("Could not clone listener");

        let handle_client = move |stream: TcpStream| {
            thread::spawn(move || {
                Server::handle_client(stream);
            });
        };

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    handle_client(stream);
                }
                Err(e) => {
                    eprintln!("An error occurred while accepting a connection: {}", e);
                }
            }
        }

        Ok(())
    }



    fn handle_client(mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        while match stream.read(&mut buffer) {
            Ok(size) => {
                if size > 0 {
    
                    let message = String::from_utf8_lossy(&buffer[..size]);

                    match Transmitter::new("127.0.0.1", "1337", &message) {
                        Ok(mut trans) => {
                            if let Ok(reponse) = trans.get_message() {
                                stream.write_all(&reponse).unwrap();
                            }
                            else {
                                stream.write_all(b"An error occurred while processing the request").unwrap();
                            }
                        }
                        Err(_) => {
                            stream.write_all(b"An error occurred while processing the request").unwrap();
                        }
                    }
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

pub struct Transmitter<'a> {
    ip: &'a str,
    port: &'a str,
    stream: TcpStream,
    message: &'a str,
    buffer: Vec<u8>,
}
impl<'a> Transmitter<'a> {
    pub fn new(ip: &'a str, port: &'a str, message: &'a str ) -> Result<Transmitter<'a>> {
        let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 1337);
        let sixty = Duration::new(5, 0);
        let stream = match TcpStream::connect_timeout(&address, sixty){
            Ok(a) => a,
            Err(e) => return Err(e),
        };
        println!("{:?}", stream);
        Ok(Transmitter {
            ip,
            port,
            stream,
            message,
            buffer: Vec::new(),
        })
    }


    pub fn get_message(&mut self) -> Result<&Vec<u8>> {
        &self.stream.write_all(self.message.as_bytes())?;
        println!("Sent:\n{}", self.message);

        loop {
            let mut chunk = vec![0; 1024];
            let bytes_read = self.stream.read(&mut chunk)?;
            if bytes_read == 0 {
                break;
            }
            let _ = &self.buffer.extend_from_slice(&chunk[..bytes_read]);
        }

        let response = String::from_utf8_lossy(&self.buffer).into_owned();
        println!("Received:\n{}", response);

        Ok(&self.buffer)
    }

}
*/