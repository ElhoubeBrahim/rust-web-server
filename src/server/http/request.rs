use std::{
    collections::HashMap,
    io::{Error, Write},
    net::TcpStream,
};

use regex::Regex;

use crate::server::logger::{access::AccessLogger, error::ErrorLogger, LogLevel};

use super::{
    response::Response,
    types::{Method, Version, URI},
};

#[derive(Debug)]
pub struct Request {
    method: Method,
    uri: URI,
    version: Version,
    headers: HashMap<String, String>,
    body: String,
}

impl Request {
    pub fn new() -> Request {
        Request {
            method: Method::GET,
            uri: URI::new(),
            version: Version::HTTP1_1,
            headers: HashMap::new(),
            body: String::new(),
        }
    }

    pub fn parse(&mut self, buffer: &str) -> Result<(), Error> {
        // Split the request data into its components
        let mut request_data = buffer.split("\r\n\r\n");

        // Parse the request components
        let mut parts = request_data.next().unwrap().split("\n");
        self.parse_request_line(parts.next().unwrap())?;
        self.parse_headers(parts.collect())?;

        // Parse the request body
        let body = request_data.next().unwrap_or("");
        self.parse_body(body);

        Ok(())
    }

    pub fn handle(&self, stream: &mut TcpStream) {
        let mut response = Response::new(self);
        let raw = response.prepare();

        // Log the request and response
        let message = format!(
            "{:?} {} /{} {} {}",
            self.method,
            self.version.as_str(),
            self.uri.path,
            response.status(),
            response.body().len(),
        );
        AccessLogger::log(&message);

        stream.write_all(raw.as_bytes()).unwrap();
    }

    pub fn close(&self, status: u16, status_text: &str, stream: &mut TcpStream) {
        let response = format!("{} {} {}", self.version.as_str(), status, status_text);

        stream.write_all(response.as_bytes()).unwrap();
    }

    fn parse_request_line(&mut self, line: &str) -> Result<(), Error> {
        // Split the request line into its components
        // GET /path?query=string HTTP/1.1
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        println!("{:?}", line);

        // Ensure the request line is valid
        if parts.len() != 3 {
            ErrorLogger::log(
                LogLevel::ERROR,
                format!("Invalid request line: {}", line.trim_matches('\0')).as_str(),
            );
            return Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid request line",
            ));
        }

        // Parse the request line components
        self.method = RequestParser::parse_method(parts[0])?;
        self.uri = RequestParser::parse_uri(parts[1])?;
        self.version = RequestParser::parse_version(parts[2])?;

        Ok(())
    }

    fn parse_headers(&mut self, lines: Vec<&str>) -> Result<(), Error> {
        for line in lines {
            let header = RequestParser::parse_header(line)?;

            if header.0.is_empty() || header.1.is_empty() {
                continue;
            }

            self.headers.insert(header.0, header.1);
        }

        Ok(())
    }

    fn parse_body(&mut self, body: &str) {
        // Get the content-length header or default to 0
        let content_length = self
            .headers
            .get("content-length")
            .unwrap_or(&"0".to_string())
            .parse::<usize>()
            .unwrap();

        // Take the first `content_length` characters as the body
        self.body = body
            .to_string()
            .chars()
            .take(content_length)
            .collect::<String>();
    }

    pub fn uri(&self) -> URI {
        URI {
            path: self.uri.path.clone(),
            params: self.uri.params.clone(),
        }
    }

    pub fn version(&self) -> &Version {
        &self.version
    }
}

pub struct RequestParser {}
impl RequestParser {
    pub fn parse_method(method: &str) -> Result<Method, Error> {
        let method = match method {
            "GET" => Method::GET,
            "POST" => Method::POST,
            "PUT" => Method::PUT,
            "DELETE" => Method::DELETE,
            "OPTIONS" => Method::OPTIONS,
            _ => Method::GET,
        };

        Ok(method)
    }

    pub fn parse_uri(uri: &str) -> Result<URI, Error> {
        // Validate the URI format
        if uri != "/"
            && !Regex::new(r"^(\/?\w+)+(\.)?\w+(\?(\w+=[\w\d]+(&\w+=[\w\d]+)*)+){0,1}$")
                .unwrap()
                .is_match(uri)
        {
            ErrorLogger::log(
                LogLevel::ERROR,
                format!("Invalid URI format: {}", uri).as_str(),
            );
            return Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid URI format",
            ));
        }

        // Split the URI into its path and parameters
        let mut parts = uri.split("?");

        let path = parts.next().unwrap().trim_matches('/');
        let mut params = HashMap::new();

        // Parse the URI parameters => ?key=value&key=value
        parts.next().unwrap_or("").split("&").for_each(|param| {
            let mut parts = param.split("=");

            let key = parts.next().unwrap();
            let value = parts.next().unwrap_or("");

            if key.is_empty() {
                return;
            }

            params.insert(key.to_string(), value.to_string());
        });

        // Return the parsed URI
        Ok(URI {
            path: path.to_string(),
            params,
        })
    }

    pub fn parse_version(version: &str) -> Result<Version, Error> {
        let version = match version {
            "HTTP/1.0" => Version::HTTP1_0,
            "HTTP/1.1" => Version::HTTP1_1,
            "HTTP/2.0" => Version::HTTP2_0,
            _ => Version::HTTP1_1,
        };

        Ok(version)
    }

    pub fn parse_header(header: &str) -> Result<(String, String), Error> {
        // Split the header into its key and value
        let mut parts = header.split(": ");

        // Header-key: Header-value
        let key = parts.next().unwrap().to_lowercase();
        let value = parts.next().unwrap_or("");

        // Return the parsed header
        Ok((key.to_string(), value.to_string()))
    }
}
