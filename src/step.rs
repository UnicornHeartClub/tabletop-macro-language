use arg::{Arg, MacroOp};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Step {
    pub args: Vec<Arg>,
    pub op: MacroOp,
    pub result: StepResult,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StepValue {
    Boolean(bool),
    Number(i32),
    Float(f32),
    Text(String),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum StepResult {
    /// Ignore Result (default)
    Ignore,
    /// Save Result (>>)
    Save,
}
