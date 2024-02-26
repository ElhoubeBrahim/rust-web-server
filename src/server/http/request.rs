use std::collections::HashMap;

use super::types::{Method, Version, URI};

pub struct Request {
    method: Method,
    uri: URI,
    version: Version,
    headers: HashMap<String, String>,
}

impl Request {
    pub fn new() -> Request {
        Request {
            method: Method::GET,
            uri: URI::new(),
            version: Version::HTTP1_1,
            headers: HashMap::new(),
        }
    }

    pub fn parse(&mut self, buffer: &str) {
        let mut parts = buffer.split("\r\n");

        // Parse the request components
        self.parse_request_line(parts.next().unwrap());
        self.parse_headers(parts.collect());
    }

    pub fn handle(&self) {
        println!("Method: {:?}", self.method);
        println!("Path: {:?}", self.uri.path);
        println!("Params: {:?}", self.uri.params);
        println!("Version: {:?}", self.version);
        println!("Headers: {:?}", self.headers);
    }

    fn parse_request_line(&mut self, line: &str) {
        // Split the request line into its components
        // GET /path?query=string HTTP/1.1
        let parts = line.split_whitespace().collect::<Vec<&str>>();

        // Parse the request line components
        self.method = RequestParser::parse_method(parts[0]);
        self.uri = RequestParser::parse_uri(parts[1]);
        self.version = RequestParser::parse_version(parts[2]);
    }

    fn parse_headers(&mut self, lines: Vec<&str>) {
        for line in lines {
            let header = RequestParser::parse_header(line);
            self.headers.insert(header.0, header.1);
        }
    }
}

pub struct RequestParser {}
impl RequestParser {
    pub fn parse_method(method: &str) -> Method {
        match method {
            "GET" => Method::GET,
            "POST" => Method::POST,
            "PUT" => Method::PUT,
            "DELETE" => Method::DELETE,
            "OPTIONS" => Method::OPTIONS,
            _ => Method::GET,
        }
    }

    pub fn parse_uri(uri: &str) -> URI {
        // Split the URI into its path and parameters
        let mut parts = uri.split("?");

        let path = parts.next().unwrap();
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
        URI {
            path: path.to_string(),
            params,
        }
    }

    pub fn parse_version(version: &str) -> Version {
        match version {
            "HTTP/1.0" => Version::HTTP1_0,
            "HTTP/1.1" => Version::HTTP1_1,
            "HTTP/2.0" => Version::HTTP2_0,
            _ => Version::HTTP1_1,
        }
    }

    pub fn parse_header(header: &str) -> (String, String) {
        // Split the header into its key and value
        let mut parts = header.split(": ");

        // Header-key: Header-value
        let key = parts.next().unwrap().to_lowercase();
        let value = parts.next().unwrap_or("");

        // Return the parsed header
        (key.to_string(), value.to_string())
    }
}
