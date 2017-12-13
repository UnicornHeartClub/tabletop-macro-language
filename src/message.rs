use chrono::DateTime;
use chrono::prelude::Utc;

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    /// From Token ID
    pub from: Option<String>,

    /// To Token ID
    pub to: Option<String>,

    /// The message
    pub message: String,

    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

impl Message {
    pub fn new (message: String) -> Message {
        let timestamp = Utc::now();
        Message {
            timestamp,
            to: None,
            from: None,
            message,
        }
    }
}
