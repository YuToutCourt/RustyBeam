use crate::server::Server;


pub struct RoundRobin<'a> {
    server_list: Vec<&'a Server<'a>>,
    current_index: usize,
}

impl<'a> RoundRobin<'a> {
    pub fn new() -> RoundRobin<'a> {
        RoundRobin {
            server_list: Vec::new(),
            current_index: 0,
        }
    }

    pub fn add_server(&mut self, server: &'a Server<'a>) {
        self.server_list.push(server);
    }

    pub fn next(&mut self) -> &'a Server<'a> {
        let server = self.server_list[self.current_index];
        self.current_index = (self.current_index + 1) % self.server_list.len();
        server
    }
}
