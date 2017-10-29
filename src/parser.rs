use errors::*;
use nom::{multispace, IResult, is_alphanumeric};
use std::str::FromStr;
use std::str;

enum SyntaxType {
    /// An action that gets run (e.g. !say, !roll)
    Lambda,

    /// A manual selection done by the user
    Prompt,

    /// An expression that results in true or false
    Ternary,

    /// Macro definition
    Macro,
}

// Start the expression, we take a string and check to make sure we define a macro
// Read a # and grab up until the first whitespace
named!(take_macro_name <&str>, do_parse!(
    tag!("#") >>
    name: map_res!(
        is_not!(" \t\r\n"),
        str::from_utf8
    ) >>
    (name)
));


/// Parses TableTop Macro Language code into an Abstract Syntax Tree (AST)
/// This function can throw specific errors based on the input
pub fn parse_ttml(input: &str) -> IResult<&[u8], &str> {
    take_macro_name(input.as_bytes())
    // .chain_err(|| "unable to open tretrete file")?;
}

/// Executes and processes the AST, resulting in the final output of the program
pub fn execute_ast(ast: &str) -> () {
}
