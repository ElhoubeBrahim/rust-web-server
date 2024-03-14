use chrono::{DateTime, Local};
use crate::server::config::CONFIG;

use super::Logger;

pub struct AccessLogger;
impl AccessLogger {
    pub fn log(message: &str) {
        // Get current date and time
        let local: DateTime<Local> = Local::now();
        let date = local.format("%d-%m-%Y");
        let time = local.format("%H:%M:%S");

        // Prepare the message
        let message = format!("[{} {}] {}", date, time, message);
        Logger::write(&CONFIG.logs().access_log, &message);
    }
}