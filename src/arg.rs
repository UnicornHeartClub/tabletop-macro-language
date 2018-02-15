use step::Step;
use std::collections::HashMap;

// Top-level arguments
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Arg {
    Assign(Assign),
    Case(Case),
    Conditional(Conditional),
    Input(TextInterpolated),
    Prompt(Prompt),
    Roll(RollArg),
    Say(SayArg),
    Target(TargetArg),
    Template(TemplateArg),
    TestMode(bool),
    Token(TokenArg),
    Unrecognized(ArgValue),
    Variable(String),
}

// Command-level arguments
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ArgValue {
    Array(Vec<ArgValue>),
    Boolean(bool),
    Float(f32),
    Number(i32),
    Object(HashMap<String, ArgValue>),
    Primitive(Primitive),
    Text(String),
    TextInterpolated(TextInterpolated),
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
pub struct Case {
    pub input: ArgValue,
    pub options: Vec<SwitchOption>,
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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MacroOp {
    /// Case (switch) statement (!case)
    Case,
    /// Exit command
    Exit,
    /// Input command
    Input,
    /// Lamda (assignment or conditional argument)
    Lambda,
    /// Macro Name
    Name(String),
    /// Primitive operations
    Primitive,
    /// Prompt (!prompt)
    Prompt,
    /// Roll (!roll)
    Roll,
    /// Hidden Roll (!hroll)
    RollHidden,
    /// Whisper Roll (!wroll)
    RollWhisper,
    /// Say (!say)
    Say,
    /// Target (!target)
    Target,
    /// Template (!template)
    Template,
    /// Test Mode (!test)
    TestMode,
    /// Whisper (!whisper)
    Whisper,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Primitive {
    Add,
    Divide,
    Multiply,
    Subtract,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Prompt {
    pub message: TextInterpolated,
    pub options: Vec<SwitchOption>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SwitchOption {
    pub key: Option<String>,
    pub value: ArgValue,
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
    Primitive(Primitive),
    RO(ArgValue),
    RR(ArgValue),
    Sides(Vec<ArgValue>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum SayArg {
    Message(TextInterpolated),
    To(TokenArg),
    From(TokenArg),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum TargetArg {
    Message(TextInterpolated),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TextInterpolated {
    pub parts: Vec<ArgValue>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TemplateArg {
    Name(String),
    Attributes(ArgValue),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TokenArg {
    pub name: String,
    pub attribute: Option<String>,
    pub macro_name: Option<String>,
}

impl TokenArg {
    pub fn to_string(&self) -> String {
        let mut string = "@".to_string() + &self.name;

        match &self.attribute {
            &Some(ref name) => { string = string + "." + name; },
            &None => {},
        }

        match &self.macro_name {
            &Some(ref name) => { string = string + "->" + name; },
            &None => {},
        }

        string.to_string()
    }
}
