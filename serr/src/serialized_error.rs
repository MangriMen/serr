use std::borrow::Cow;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SerializedError {
    pub code: Cow<'static, str>,
    pub fields: Option<serde_json::Value>,
    pub message: String,
}

pub trait ToSerializedError {
    fn to_serialized(&self) -> SerializedError;
}
