use chrono::{DateTime, Local};

use crate::server::config::CONFIG;

use super::{log_level_to_string, LogLevel, Logger};

pub struct ErrorLogger;
impl ErrorLogger {
    pub fn log(level: LogLevel, message: &str) {
        // Get current date and time
        let local: DateTime<Local> = Local::now();
        let date = local.format("%d-%m-%Y");
        let time = local.format("%H:%M:%S");
        
        // Get error log file
        let config = &CONFIG.as_ref();
        let error_log = match config {
            Some(c) => &c.logs().error_log,
            None => "server/error.log",
        };

        println!("Error log: {}", error_log);

        // Prepare the message
        let message = format!(
            "[{}] {} {} - {}",
            log_level_to_string(level),
            date,
            time,
            message
        );
        Logger::write(error_log, &message);
    }
}
