use crate::server::Server;
use std::io::Result;


pub struct RoundRobin<'a> {
    server_list: Vec<&'a Result<Server<'a>>>,
    current_index: usize,
}


impl<'a> RoundRobin<'a> {
    pub fn new() -> RoundRobin<'a> {

        RoundRobin {
            server_list: vec![
                &Server::new("127.0.0.1", "1337"),
                &Server::new("127.0.0.1", "1338"),
                &Server::new("127.0.0.1", "1339"),
            ],
            current_index: 0,
        }
    }


    pub fn add_server(&mut self, server: &'a Result<Server<'a>>) {
        self.server_list.push(server);
    }


    pub fn next(&mut self) -> &'a Result<Server<'a>> {
        let server = self.server_list[self.current_index];
        self.current_index = (self.current_index + 1) % self.server_list.len();
        server
    }
}