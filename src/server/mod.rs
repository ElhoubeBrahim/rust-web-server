use std::{
    io::{BufRead, BufReader},
    net::TcpListener,
};

mod config;
mod http;

use http::request::Request;

use config::CONFIG;

pub struct Server {
    host: String,
    port: u16,
    listener: Option<TcpListener>,
}

impl Server {
    pub fn new() -> Server {
        let config = &CONFIG;

        Server {
            host: config.host().host.to_string(),
            port: config.host().port,
            listener: None,
        }
    }

    pub fn run(&mut self) {
        // Start the server on the specified host and port
        self.listener = Some(TcpListener::bind(format!("{}:{}", self.host, self.port)).unwrap());
        println!("Server is running on http://{}:{}", self.host, self.port);

        // Listen for incoming connections
        for stream in self.listener.as_ref().unwrap().incoming() {
            // Read the incoming data
            let mut stream = stream.unwrap();
            let buffer = BufReader::new(&mut stream)
                .lines()
                .map(|line| line.unwrap())
                .take_while(|line| !line.is_empty())
                .collect::<Vec<String>>()
                .join("\r\n")
                .to_string();

            // Handle the incoming request
            let mut request = Request::new();
            request.parse(buffer.as_str());
            request.handle(&mut stream);
        }
    }
}
