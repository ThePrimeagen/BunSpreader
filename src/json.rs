use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InnerJsonMessage {
    pub width: i32,
    pub height: i32,
    pub girth: i32,
    pub depth: i32,
    pub length: i32,
    pub circumference: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JsonMessage {
    pub message: String,
    pub another_property: InnerJsonMessage,
}

