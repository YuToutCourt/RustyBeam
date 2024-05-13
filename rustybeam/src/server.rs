use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::io::{Read, Write};
use std::time::Duration;
use std::io::Result;
use std::thread;

pub struct LoadBalancer<'a> {
    ip: &'a str,
    port: &'a str,
    listener: TcpListener,
    timeout: u64, // in seconds
}

impl<'a> LoadBalancer<'a> {
    pub fn new(ip: &'a str, port: &'a str) -> Result<LoadBalancer<'a>> {
        let address = format!("{}:{}", ip, port);

        let listener = TcpListener::bind(address).expect("Could not bind");
        println!("Server listening on {}:{}", ip, port);

        Ok(LoadBalancer {
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
                LoadBalancer::handle_client(stream);
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
                    match Server::new(&message ) {
                        Ok(mut trans) => {
                            println!("{:?}", trans);
                            if let Ok(reponse) = trans.get_message() {
                                stream.write_all(&reponse).unwrap();
                            } else {
                                stream.write_all(b"An error occurred while processing the request").unwrap();
                            }
                        }
                        Err(_) => {
                            println!("Timeout");
                            stream.write(b"HTTP/1.0 503 Service unavaible\r\nConnection: close\r\nContent-Type:
text/html;charset=utf-8\r\nContent-Length: 0\r\n\r\n"
                            ).unwrap();
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

#[derive(Debug)]
pub struct Server<'a> {
    stream: TcpStream,
    message: &'a str,
    buffer: Vec<u8>,
}
impl<'a> Server<'a> {
    pub fn new(message: &'a str, mut round_robin: RoundRobin ) -> Result<Server<'a>> {
        let address = round_robin.next();
        let sixty = Duration::new(5, 0);

        match TcpStream::connect_timeout(&address, sixty){
            Ok(stream) => Ok(Server {
                stream: stream,
                message,
                buffer: Vec::new(),
            }),
            Err(e) => Err(e),
        }
    }


    pub fn get_message(&mut self) -> Result<&Vec<u8>> {
        &self.stream.write_all(self.message.as_bytes())?;

        loop {
            let mut chunk = vec![0; 1024];
            let bytes_read = self.stream.read(&mut chunk)?;
            if bytes_read == 0 {
                break;
            }
            let _ = &self.buffer.extend_from_slice(&chunk[..bytes_read]);
        }

        let response = String::from_utf8_lossy(&self.buffer).into_owned();
        println!("la réponse: {:?}", response);
        Ok(&self.buffer)
    }

}

pub struct RoundRobin{
    servers: Vec<SocketAddr>,
    current: usize,
}

impl RoundRobin {
    pub fn new() -> RoundRobin{
        RoundRobin {
            servers: Vec::new(),
            current: 0,
        }
    }

    pub fn add_server(&mut self, server: SocketAddr) {
        self.servers.push(server);
    }

    pub fn next(&mut self) -> &SocketAddr {
        let server = &self.servers[self.current];
        if self.current == self.servers.len() - 1 {
            self.current = 0;
        } else {
            self.current += 1;
        }
        server
    }
}