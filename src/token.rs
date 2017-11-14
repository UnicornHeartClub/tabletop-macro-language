use std::collections::HashMap;

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Token {
    pub attributes: HashMap<String, TokenAttributeValue>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TokenAttributeValue {
    Number(i32),
    Text(String),
}
