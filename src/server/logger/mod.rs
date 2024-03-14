pub mod access;

use std::fs::OpenOptions;
use std::io::Write;

pub struct Logger;
impl Logger {
    fn write(file: &str, message: &str) {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file)
            .expect("Failed to open the error log file.");

        writeln!(file, "{}", message).expect("Failed to write to the error log.");
    }
}
