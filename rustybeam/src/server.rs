pub mod server_mod {

    use std::net::{TcpListener, TcpStream};
    use std::io::{Read, Write};
    use std::thread;


    use transmitter;
    use transmitter::trans_mod::Transmitter;

    pub struct Server<'a> {
        ip: &'a str,
        port: &'a str,
        listener: TcpListener,
        stream: TcpStream,
        timeout: u64, // in seconds
    }



    impl<'a> Server<'a> {
        pub fn new(ip: &'a str, port: &'a str) -> Result<Server<'a>, std::io::Error> {
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
                    if *size > 0 {
        
                        let message = String::from_utf8_lossy(&buffer[..*size]);
        
                        let response = Transmitter::new("127.0.0.1", "1337", &message).get_message().unwrap();
        
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


    }

}