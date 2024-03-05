use std::{
    io::Read,
    net::TcpListener,
};

mod config;
mod http;

use http::request::Request;

use config::CONFIG;

pub struct Server {
    host: String,
    port: u16,
    buffer_size: usize,
    listener: Option<TcpListener>,
}

impl Server {
    pub fn new() -> Server {
        let config = &CONFIG;

        Server {
            host: config.host().host.to_string(),
            port: config.host().port,
            buffer_size: config.connection().buffer_size,
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
            let mut buffer = vec![0; self.buffer_size];
            let mut stream = stream.unwrap();
            stream.read(&mut buffer).unwrap();
            let request_str = String::from_utf8_lossy(&buffer[..]).to_string();

            // Handle the incoming request
            let mut request = Request::new();
            request.parse(&request_str);
            request.handle(&mut stream);
        }
    }
}
