use std::net::TcpStream;
use std::io::{Read, Write};


pub struct Transmitter<'a> {
    ip: &'a str,
    port: &'a str,
    stream: TcpStream,
    message: &'a str,
    buffer: Vec<u8>,
}
impl<'a> Transmitter<'a> {
    pub fn new(ip: &'a str, port: &'a str, message: &'a str ) -> std::io::Result<Transmitter<'a>> {
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


    pub fn get_message(&mut self) -> std::io::Result<&Vec<u8>> {
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