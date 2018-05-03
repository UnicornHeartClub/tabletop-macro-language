use arg::{Arg, MacroOp};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Step {
    pub args: Vec<Arg>,
    pub op: MacroOp,
    pub result: StepResult,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum StepResult {
    /// Ignore Result (default)
    Ignore,
    /// Save Result (>>)
    Save,
}
