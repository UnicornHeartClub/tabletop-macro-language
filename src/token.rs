use std::collections::HashMap;
use step::StepValue;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Token {
    pub attributes: HashMap<String, StepValue>,
    pub macros: HashMap<String, StepValue>,
}
