use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorMessage {
    pub status: i32,
    pub message: String,
    pub details: Option<String>,
}

impl ErrorMessage {
    pub fn internal(message: &str, details: Option<String>) -> Self {
        Self {
            status: 0,
            message: message.into(),
            details,
        }
    }
}
