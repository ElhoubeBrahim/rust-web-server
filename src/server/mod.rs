use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

pub struct Server {
    host: String,
    port: u16,
    listener: Option<TcpListener>,
}

impl Server {
    pub fn new() -> Server {
        Server {
            host: String::from("127.0.0.1"),
            port: 8080,
            listener: None,
        }
    }

    pub fn run(&mut self) {
        self.listener = Some(TcpListener::bind(format!("{}:{}", self.host, self.port)).unwrap());
        println!("Server is running on http://{}:{}", self.host, self.port);

        for stream in self.listener.as_ref().unwrap().incoming() {
            self.handle_request(stream.unwrap());
        }
    }

    pub fn handle_request(&self, mut stream: TcpStream) {
        let request = BufReader::new(&mut stream)
            .lines()
            .map(|line| line.unwrap())
            .take_while(|line| line != "")
            .collect::<Vec<String>>();

        println!("{:?}", request);

        let response = "HTTP/1.1 200 OK\r\n\r\n";
        stream.write_all(response.as_bytes()).unwrap();
    }
}
