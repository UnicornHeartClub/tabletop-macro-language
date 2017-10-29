use errors::*;
use nom::IResult;
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

// Read "#" and take until the first whitespace
named!(take_macro_name <&str>, do_parse!(
    tag!("#") >>
    name: map_res!(
        is_not!(" \t\r\n"),
        str::from_utf8
    ) >>
    (name)
));


// Execute the complete macro
// named!(execute_macro, );

/// Parses TTML and executes on-the-fly
pub fn parse_ttml(input: &str) -> IResult<&[u8], &str> {
    take_macro_name(input.as_bytes())
}
