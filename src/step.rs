use arg::{Arg, MacroOp};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Step {
    pub args: Vec<Arg>,
    pub op: MacroOp,
    pub result: StepResult,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum StepResult {
    /// Ignore Result (default)
    Ignore,
    /// Save Result (>>)
    Save,
}
