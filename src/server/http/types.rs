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