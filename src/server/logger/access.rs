use crate::server::config::CONFIG;
use chrono::{DateTime, Local};

use super::Logger;

pub struct AccessLogger;
impl AccessLogger {
    pub fn log(message: &str) {
        // Get current date and time
        let local: DateTime<Local> = Local::now();
        let date = local.format("%d-%m-%Y");
        let time = local.format("%H:%M:%S");

        // Get access log file
        let config = &CONFIG.as_ref();
        let access_log = match config {
            Some(c) => &c.logs().access_log,
            None => "server/access.log",
        };

        // Prepare the message
        let message = format!("[{} {}] {}", date, time, message);
        Logger::write(access_log, &message);
    }
}
