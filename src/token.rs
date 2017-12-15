use std::collections::HashMap;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Token {
    pub attributes: HashMap<String, TokenAttributeValue>,
    pub macros: HashMap<String, TokenAttributeValue>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum TokenAttributeValue {
    Number(i32),
    Float(f32),
    Text(String),
}
