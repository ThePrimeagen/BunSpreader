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

impl Default for InnerJsonMessage {
    fn default() -> Self {
        return Self {
            width: 0,
            height: 0,
            girth: 0,
            depth: 0,
            length: 0,
            circumference: 0,
        };
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JsonMessage {
    pub message: String,
    pub another_property: InnerJsonMessage,
}

impl Default for JsonMessage {
    fn default() -> Self {
        return Self {
            message: String::from(""),
            another_property: InnerJsonMessage::default(),
        };
    }
}
