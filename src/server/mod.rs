use std::{io::Read, net::TcpListener, thread};

mod config;
mod http;
mod logger;

use config::CONFIG;
use http::request::Request;

use self::logger::error::ErrorLogger;

pub struct Server {
    host: String,
    port: u16,
    max_connections: u16,
    buffer_size: usize,
    listener: Option<TcpListener>,
}

impl Server {
    pub fn new() -> Server {
        let config = &CONFIG.as_ref();
        match config {
            Some(_) => {}
            None => {
                ErrorLogger::log(
                    logger::LogLevel::ERROR,
                    "Failed to load the server configuration file.",
                );
                std::process::exit(1);
            }
        }

        let config = config.unwrap();
        Server {
            host: config.host().host.to_string(),
            port: config.host().port,
            max_connections: config.connection().max_connections,
            buffer_size: config.connection().buffer_size,
            listener: None,
        }
    }

    pub fn run(&mut self) {
        // Start the server on the specified host and port
        let result = TcpListener::bind(format!("{}:{}", self.host, self.port));

        self.listener = match result {
            Ok(listener) => Some(listener),
            Err(_) => {
                ErrorLogger::log(
                    logger::LogLevel::ERROR,
                    format!("Failed to bind the server on \"{}:{}\". Check server configuration and port availability", self.host, self.port).as_str(),
                );
                std::process::exit(1);
            }
        };
        println!("Server is running on http://{}:{}", self.host, self.port);

        // Listen for incoming connections
        let mut connections = 0;
        for stream in self.listener.as_ref().unwrap().incoming() {
            connections += 1;
            if connections > self.max_connections {
                ErrorLogger::log(
                    logger::LogLevel::WARN,
                    format!(
                        "Max connections reached. {}/{}",
                        connections, self.max_connections
                    )
                    .as_str(),
                );
                break;
            }

            let mut buffer = vec![0; self.buffer_size];
            let mut stream = stream.unwrap();

            thread::spawn(move || {
                // Read the incoming data
                stream.read(&mut buffer).unwrap();
                let request_str = String::from_utf8_lossy(&buffer[..]).to_string();

                // Handle the incoming request
                let mut request = Request::new();
                request.parse(&request_str);
                request.handle(&mut stream);
            });
        }
    }
}
