pub mod ServerMod {

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

pub struct Server {
    ip: &str,
    port: &str,
    listener: TcpListener,
    stream: TcpStream,
    timeout: u64, // in seconds
}



impl Server {
    pub fn new(ip: &str, port: &str) -> Result<Server, std::io::Error> {
        let address = format!("{}:{}", ip, port);

        let listener = TcpListener::bind(address).expect("Could not bind");
        let stream = listener.accept().unwrap().0;
        println!("Server listening on {}:{}", ip, port);
    
        Ok(Server {
            ip: ip,
            port: port,
            listener: listener,
            stream: stream,
            timeout: 10,
        })
    }


    pub fn start(&self) -> Result<(), std::io::Error> {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("New connection: {}", stream.peer_addr().unwrap());
                    let server_clone = self.clone(); // Cloner la référence au serveur pour le passer au thread
                    thread::spawn(move || {
                        server_clone.handle_client();
                    });
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
        Ok(()) // Retourner Ok(()) après la boucle
    }

    fn handle_client(&self) {
        let mut buffer = [0; 1024];
        while match &self.stream.read(&mut buffer) {
            Ok(size) => {
                if size > 0 {
    
                    let message = String::from_utf8_lossy(&buffer[..size]);
    
                    let response = &self.emit_message(&message).unwrap();
    
                    &self.stream.write_all(&response).unwrap();
    
                }
                true
            },
            Err(_) => {
                println!("An error occurred, terminating connection with {}", &self.stream.peer_addr().unwrap());
                &self.stream.shutdown(std::net::Shutdown::Both).unwrap();
                false
            }
        } {}
    }


    fn emit_message(&self, message: &str) {}
    let server_ = "127.0.0.1:1337";

    let mut connection = TcpStream::connect(server_)?;

    connection.write_all(message.as_bytes())?;
    println!("Sent:\n{}", message);

    let mut buffer = Vec::new(); // Vecteur dynamique pour stocker la réponse

    loop {
        let mut chunk = vec![0; 1024]; // Tampon temporaire pour lire les données
        let bytes_read = connection.read(&mut chunk)?;
        if bytes_read == 0 {
            break; // Fin de la connexion, sortir de la boucle
        }
        // Ajouter les données lues au vecteur de réponse
        buffer.extend_from_slice(&chunk[..bytes_read]);
    }

    let response = String::from_utf8_lossy(&buffer).into_owned();
    println!("Received:\n{}", response);

    Ok(buffer)

}



/*
fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    while match stream.read(&mut buffer) {
        Ok(size) => {
            if size > 0 {

                let message = String::from_utf8_lossy(&buffer[..size]);

                let response = emit_message(&message).unwrap();

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

fn emit_message(message: &str) -> Result<Vec<u8>, std::io::Error> {
    let server_ = "127.0.0.1:1337";

    let mut connection = TcpStream::connect(server_)?;

    connection.write_all(message.as_bytes())?;
    println!("Sent:\n{}", message);

    let mut buffer = Vec::new(); // Vecteur dynamique pour stocker la réponse

    loop {
        let mut chunk = vec![0; 1024]; // Tampon temporaire pour lire les données
        let bytes_read = connection.read(&mut chunk)?;
        if bytes_read == 0 {
            break; // Fin de la connexion, sortir de la boucle
        }
        // Ajouter les données lues au vecteur de réponse
        buffer.extend_from_slice(&chunk[..bytes_read]);
    }

    let response = String::from_utf8_lossy(&buffer).into_owned();
    println!("Received:\n{}", response);

    Ok(buffer)

}
*/
}