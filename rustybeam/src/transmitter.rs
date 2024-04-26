pub mod trans_mod {

    use std::net::TcpStream;
    use std::io::{Read, Write};

    use server;
    use server::ServerMod::Server;
    
    pub struct Transmitter<'a> {
        ip: &'a str,
        port: &'a str,
        stream: TcpStream,
        message: &'a str,
        buffer: vec<u8>,
    }
    
    
    
    impl Transmitter {
        pub fn new(ip: &'a str, port: &'a str, message: &'a str ) -> Result<Transmitter<'a>, std::io::Error> {
            let address = format!("{}:{}", ip, port);
    
            let stream = TcpStream::connect(address)?;
            println!("Connected to {}:{}", ip, port);
        
            Ok(Transmitter {
                ip: ip,
                port: port,
                stream: stream,
                message: message,
                buffer: Vec::new(),
            })
        }
    
    
        pub fn get_message(&self){
            &self.stream.write_all(self.message.as_bytes())?;
            println!("Sent:\n{}", self.message);
        
            loop {
                let mut chunk = vec![0; 1024]; // Tampon temporaire pour lire les données
                let bytes_read = &self.stream.read(&mut chunk)?;
                if bytes_read == 0 {
                    break; // Fin de la connexion, sortir de la boucle
                }
                // Ajouter les données lues au vecteur de réponse
                &self.buffer.extend_from_slice(&chunk[..bytes_read]);
            }

            let response = String::from_utf8_lossy(&self.buffer).into_owned();
            println!("Received:\n{}", response);

            Ok(&self.buffer)
        }

    }
}