use std::net::TcpStream;
use std::io::{Read, Write};
use std::io::Result;



pub struct Server<'a> {
    ip: &'a str,
    port: &'a str,
    stream: TcpStream,
    buffer: Vec<u8>,
}
impl<'a> Server<'a> {
    pub fn new(ip: &'a str, port: &'a str) -> Result<Server<'a>> {
        let address = format!("{}:{}", ip, port);

        let stream = TcpStream::connect(address)?;
        println!("Connected to {}:{}", ip, port);

        Ok(Server {
            ip,
            port,
            stream,
            buffer: Vec::new(),
        })
    }


    pub fn make_request(&mut self, message: &str) -> Result<&Vec<u8>> {
        &self.stream.write_all(message.as_bytes())?;
        println!("Sent:\n{}", message);

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