use std::collections::HashMap;

#[derive(Debug)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    OPTIONS,
}

#[derive(Debug)]
pub enum Version {
    HTTP1_0,
    HTTP1_1,
    HTTP2_0,
}

impl Version {
    pub fn as_str(&self) -> &str {
        match self {
            Version::HTTP1_0 => "HTTP/1.0",
            Version::HTTP1_1 => "HTTP/1.1",
            Version::HTTP2_0 => "HTTP/2.0",
        }
    }
}

#[derive(Debug)]
pub struct URI {
    pub path: String,
    pub params: HashMap<String, String>,
}

impl URI {
    pub fn new() -> URI {
        URI {
            path: String::new(),
            params: HashMap::new(),
        }
    }
}