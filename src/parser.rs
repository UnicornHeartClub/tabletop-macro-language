use errors::*;

enum SyntaxType {
    Lambda, /// An action that gets run (e.g. !say, !roll)
    Prompt, /// A manual selection done by the user
}

/// Parses TableTop Macro Language code into an Abstract Syntax Tree (AST)
/// This function can throw specific errors based on the input
pub fn parse_ttml(input: &str) -> Result<()> {

    Ok(())
}

/// Executes and processes the AST, resulting in the final output of the program
pub fn execute_ast(ast: &str) -> () {
}
