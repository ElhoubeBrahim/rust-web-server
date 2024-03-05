use std::{collections::HashMap, io::Write, net::TcpStream};

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

    pub fn parse(&mut self, buffer: &str) {
        // Split the request data into its components
        let mut request_data = buffer.split("\r\n\r\n");

        // Parse the request components
        let mut parts = request_data.next().unwrap().split("\n");
        self.parse_request_line(parts.next().unwrap());
        self.parse_headers(parts.collect());

        // Parse the request body
        let body = request_data.next().unwrap_or("");
        self.parse_body(body);
    }

    pub fn handle(&self, stream: &mut TcpStream) {
        let mut response = Response::new(self);
        let raw = response.prepare();

        stream.write_all(raw.as_bytes()).unwrap();
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

    fn parse_body(&mut self, body: &str) {
        // Get the content-length header or default to 0
        let content_length = self
            .headers
            .get("content-length")
            .unwrap_or(&"0".to_string())
            .parse::<usize>()
            .unwrap();

        // Take the first `content_length` characters as the body
        self.body = body.to_string()
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
