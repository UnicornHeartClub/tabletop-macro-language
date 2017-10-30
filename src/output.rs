use chrono::DateTime;
use chrono::prelude::Utc;
use token::Token;
use die::Die;

#[derive(Debug, Deserialize, Serialize)]
pub struct Output {
    /// The original input
    pub input: String,

    /// Errors, if any
    pub errors: Vec<ErrorOutput>,

    /// Timestamp
    pub executed: DateTime<Utc>,

    /// Time to execute final output
    pub execution_time: i64,

    /// Chat messages to be sent
    pub messages: Vec<String>,

    /// Dice rolls
    pub rolls: Vec<Die>,

    /// Tokens
    pub tokens: Vec<Token>,
 
    /// API Version
    pub version: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorOutput {
    /// Type of error
    error: String,

    /// Message
    message: String
}
