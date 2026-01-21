use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: DateTime<Utc>,
    pub level: LogLevel,
    pub message: String,
    pub context: HashMap<String, serde_json::Value>,
    pub component: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl LogEntry {
    pub fn new(level: LogLevel, message: String, component: String) -> Self {
        Self {
            timestamp: Utc::now(),
            level,
            message,
            context: HashMap::new(),
            component,
        }
    }

    pub fn with_context(mut self, key: String, value: serde_json::Value) -> Self {
        self.context.insert(key, value);
        self
    }
}
