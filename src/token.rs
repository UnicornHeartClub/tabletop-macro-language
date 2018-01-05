use std::collections::HashMap;
use arg::ArgValue;

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct Token {
    pub attributes: HashMap<String, ArgValue>,
    pub macros: HashMap<String, ArgValue>,
}
