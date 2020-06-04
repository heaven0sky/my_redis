use std::net::{TcpListener, TcpStream};
use std::{thread};

pub struct Server {
    pub ip: String,
    pub port: i32,
    pub clients: Vec<TcpStream>
}

impl Server {
    pub fn new(ip: &str, port: &i32) -> Server {
        return Server {
            ip: ip.to_string(),
            port: *port,
            clients: Vec::new()
        }
    }

    pub fn handle_client(&mut self, stream: TcpStream) {
        self.clients.push(stream);
        println!("Hello, client!");
    }

    pub fn start(&mut self) {
        let addr: String = format!();
    }
}