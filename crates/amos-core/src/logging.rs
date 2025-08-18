use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use std::fmt;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub level: LogLevel,
    pub component: String,
    pub message: String,
    pub context: serde_json::Value,
}

impl LogEntry {
    pub fn new(level: LogLevel, component: &str, message: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            level,
            component: component.to_string(),
            message: message.to_string(),
            context: serde_json::Value::Object(serde_json::Map::new()),
        }
    }
    
    pub fn with_context(mut self, key: &str, value: serde_json::Value) -> Self {
        if let serde_json::Value::Object(ref mut map) = self.context {
            map.insert(key.to_string(), value);
        }
        self
    }
}

impl fmt::Display for LogEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {:?} [{}] {}",
            self.timestamp.format("%Y-%m-%d %H:%M:%S%.3f"),
            self.level,
            self.component,
            self.message
        )?;
        
        if !self.context.is_null() && self.context != serde_json::Value::Object(serde_json::Map::new()) {
            write!(f, " | {}", self.context)?;
        }
        
        Ok(())
    }
}

pub struct Logger {
    component: String,
    min_level: LogLevel,
}

impl Logger {
    pub fn new(component: &str) -> Self {
        Self {
            component: component.to_string(),
            min_level: LogLevel::Info,
        }
    }
    
    pub fn with_level(mut self, level: LogLevel) -> Self {
        self.min_level = level;
        self
    }
    
    pub fn trace(&self, message: &str) -> LogEntry {
        self.log(LogLevel::Trace, message)
    }
    
    pub fn debug(&self, message: &str) -> LogEntry {
        self.log(LogLevel::Debug, message)
    }
    
    pub fn info(&self, message: &str) -> LogEntry {
        self.log(LogLevel::Info, message)
    }
    
    pub fn warn(&self, message: &str) -> LogEntry {
        self.log(LogLevel::Warn, message)
    }
    
    pub fn error(&self, message: &str) -> LogEntry {
        self.log(LogLevel::Error, message)
    }
    
    pub fn fatal(&self, message: &str) -> LogEntry {
        self.log(LogLevel::Fatal, message)
    }
    
    fn log(&self, level: LogLevel, message: &str) -> LogEntry {
        let entry = LogEntry::new(level.clone(), &self.component, message);
        
        if self.should_log(&level) {
            println!("{}", entry);
        }
        
        entry
    }
    
    fn should_log(&self, level: &LogLevel) -> bool {
        match (&self.min_level, level) {
            (LogLevel::Trace, _) => true,
            (LogLevel::Debug, LogLevel::Trace) => false,
            (LogLevel::Debug, _) => true,
            (LogLevel::Info, LogLevel::Trace | LogLevel::Debug) => false,
            (LogLevel::Info, _) => true,
            (LogLevel::Warn, LogLevel::Trace | LogLevel::Debug | LogLevel::Info) => false,
            (LogLevel::Warn, _) => true,
            (LogLevel::Error, LogLevel::Error | LogLevel::Fatal) => true,
            (LogLevel::Error, _) => false,
            (LogLevel::Fatal, LogLevel::Fatal) => true,
            (LogLevel::Fatal, _) => false,
        }
    }
}

#[macro_export]
macro_rules! log_context {
    ($logger:expr, $level:ident, $msg:expr, $($key:expr => $value:expr),* $(,)?) => {{
        let mut entry = $logger.$level($msg);
        $(
            entry = entry.with_context($key, serde_json::json!($value));
        )*
        entry
    }};
}

pub use log_context;