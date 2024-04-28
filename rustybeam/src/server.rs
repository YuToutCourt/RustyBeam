use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

pub struct Server<'a> {
    ip: &'a str,
    port: &'a str,
    listener: TcpListener,
    timeout: u64, // in seconds
}

impl<'a> Server<'a> {
    pub fn new(ip: &'a str, port: &'a str) -> Result<Server<'a>, std::io::Error> {
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


    pub fn start(&mut self) -> Result<(), std::io::Error> {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("New connection: {}", stream.peer_addr().unwrap());
                    thread::spawn(move || {
                        self.handle_client(stream);
                    });
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
        Ok(())
    }

    fn handle_client(&mut self, mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        while match stream.read(&mut buffer) {
            Ok(size) => {
                if size > 0 {
    
                    let message = String::from_utf8_lossy(&buffer[..size]);
    
                    let response = Transmitter::new("127.0.0.1", "1337", &message).expect("REASON").get_message().unwrap();
    
                    stream.write_all(&response).unwrap();
    
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
    pub fn new(ip: &'a str, port: &'a str, message: &'a str ) -> Result<Transmitter<'a>, std::io::Error> {
        let address = format!("{}:{}", ip, port);

        let stream = TcpStream::connect(address)?;
        println!("Connected to {}:{}", ip, port);

        Ok(Transmitter {
            ip,
            port,
            stream,
            message,
            buffer: Vec::new(),
        })
    }


    pub fn get_message(&mut self) -> Result<&Vec<u8>, std::io::Error> {
        &self.stream.write_all(self.message.as_bytes())?;
        println!("Sent:\n{}", self.message);

        loop {
            let mut chunk = vec![0; 1024];
            let bytes_read = self.stream.read(&mut chunk)?;
            if bytes_read == 0 {
                break;
            }
            &self.buffer.extend_from_slice(&chunk[..bytes_read]);
        }

        let response = String::from_utf8_lossy(&self.buffer).into_owned();
        println!("Received:\n{}", response);

        Ok(&self.buffer)
    }



}