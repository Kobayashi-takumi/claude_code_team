use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub from: String,
    pub to: String,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub message_type: MessageType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MessageType {
    Task,
    Status,
    Query,
    Response,
    Completion,
}

impl Message {
    pub fn new(from: &str, to: &str, content: &str, message_type: MessageType) -> Self {
        Self {
            from: from.to_string(),
            to: to.to_string(),
            content: content.to_string(),
            timestamp: Utc::now(),
            message_type,
        }
    }
}
