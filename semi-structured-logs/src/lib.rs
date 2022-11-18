/// various log levels
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

/// primary function for emitting logs
#[must_use]
pub fn log(level: LogLevel, message: &str) -> String {
    let level_str = match level {
        LogLevel::Debug => "DEBUG",
        LogLevel::Info => "INFO",
        LogLevel::Warning => "WARNING",
        LogLevel::Error => "ERROR",
    };
    format!("[{level_str}]: {message}")
}

#[must_use]
pub fn info(message: &str) -> String {
    log(LogLevel::Info, message)
}

#[must_use]
pub fn warn(message: &str) -> String {
    log(LogLevel::Warning, message)
}

#[must_use]
pub fn error(message: &str) -> String {
    log(LogLevel::Error, message)
}
