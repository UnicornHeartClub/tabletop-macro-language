use std::collections::HashMap;
use parser::StepValue;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Token {
    pub attributes: HashMap<String, StepValue>,
    pub macros: HashMap<String, StepValue>,
}
