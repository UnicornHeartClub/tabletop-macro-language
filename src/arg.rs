use step::Step;

// Top-level arguments
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Arg {
    Assign(Assign),
    Conditional(Conditional),
    Roll(RollArg),
    Say(SayArg),
    Token(TokenArg),
    Unrecognized(String),
    Variable(String),
}

// Command-level arguments
#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum ArgValue {
    Boolean(bool),
    Float(f32),
    Number(i32),
    Primitive(Primitive),
    Text(String),
    Token(TokenArg),
    Variable(String),
    VariableReserved(i16),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Assign {
    pub left: ArgValue,
    pub right: Vec<ArgValue>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ComparisonArg {
    EqualTo,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Conditional {
    pub left: ArgValue,
    pub comparison: ComparisonArg,
    pub right: ArgValue,
    pub success: Option<Step>,
    pub failure: Option<Step>,
}

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Primitive {
    Add,
    Divide,
    Multiply,
    Subtract,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MacroOp {
    Primitive,
    /// Lamda (assignment or conditional argument)
    Lambda,
    /// Macro Name
    Name(String),
    /// Prompt (!prompt)
    Prompt,
    /// Roll (!roll)
    Roll,
    /// Say (!say)
    Say,
    /// Whisper (!whisper)
    Whisper,
}

// Arguments for the roll command, used by the parser
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RollArg {
    Advantage,
    Comment(ArgValue),
    D(ArgValue), // e.g. d20
    Disadvantage,
    E(ArgValue),
    GT(ArgValue),
    GTE(ArgValue),
    H(ArgValue),
    L(ArgValue),
    LT(ArgValue),
    LTE(ArgValue),
    Max(ArgValue),
    Min(ArgValue),
    ModifierNeg(ArgValue),
    ModifierPos(ArgValue),
    N(ArgValue), // e.g. 1 (part of 1d20)
    RO(ArgValue),
    RR(ArgValue),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum SayArg {
    Message(String),
    To(TokenArg),
    From(TokenArg),
}

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct TokenArg {
    pub name: String,
    pub attribute: Option<String>,
    pub macro_name: Option<String>,
}
