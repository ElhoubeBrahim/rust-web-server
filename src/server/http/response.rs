use std::{collections::HashMap, fs, io::Error};

use crate::server::config::CONFIG;

use super::request::Request;

pub struct Response<'a> {
    request: &'a Request,
    status_code: u16,
    headers: HashMap<String, String>,
    body: String,
}

impl<'a> Response<'a> {
    pub fn new(request: &'a Request) -> Response<'a> {
        Response {
            request,
            status_code: 200,
            headers: HashMap::new(),
            body: String::from(""),
        }
    }

    pub fn prepare(&mut self) -> String {
        let config = &CONFIG;
        let path = self.request.uri().path;
        match self.get_file_content(path) {
            Ok(content) => {
                self.body = content;
                self.get_raw()
            }
            Err(_) => {
                let not_found_path = config.file_system().not_found.to_string();

                self.status_code = 404;
                self.body = self.get_file_content(not_found_path).unwrap();
                self.get_raw()
            }
        }
    }

    pub fn status(&self) -> u16 {
        self.status_code
    }

    pub fn body(&self) -> &str {
        &self.body
    }

    fn get_raw(&mut self) -> String {
        // Set the response headers
        self.headers
            .insert(String::from("Content-Type"), String::from("text/html"));
        self.headers
            .insert(String::from("Server"), String::from("Rust Server"));
        self.headers
            .insert(String::from("Content-Length"), self.body.len().to_string());

        // Send the response
        let response = format!(
            "{} {} {}\r\n{}\r\n\r\n{}",
            self.request.version().as_str(),
            self.status_code,
            self.status_text(),
            self.headers(),
            self.body
        );

        response
    }

    fn get_file_content(&self, path: String) -> Result<String, Error> {
        let full_path = self.build_file_path(path);

        // Read the file content
        let content = fs::read_to_string(full_path)?;
        Ok(content)
    }

    fn build_file_path(&self, path: String) -> String {
        let config = &CONFIG;
        let root_dir = config.file_system().root_dir.to_string();
        let index_file = config.file_system().index_file.to_string();

        let mut full_path = format!("{}/{}", &root_dir, &path).to_string();

        // Check if file exists
        let metadata = fs::metadata(&full_path);
        if metadata.is_err() || metadata.unwrap().is_dir() {
            full_path = if path.is_empty() {
                format!("{}/{}", &root_dir, &index_file)
            } else {
                format!("{}/{}/{}", &root_dir, &path, &index_file)
            }
        }

        full_path
    }

    fn status_text(&self) -> String {
        match self.status_code {
            200 => String::from("OK"),
            404 => String::from("Not Found"),
            500 => String::from("Internal Server Error"),
            _ => String::from("OK"),
        }
    }

    fn headers(&self) -> String {
        let mut headers = String::new();

        for (key, value) in &self.headers {
            headers.push_str(&format!("{}: {}\r\n", key, value));
        }

        headers
    }
}
