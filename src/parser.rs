use errors::*;
use chrono::prelude::Utc;
use chrono::DateTime;
use nom::{is_digit, space, IResult};
use token::Token;
use die::Die;
use std::str;

#[derive(Debug, Deserialize, Serialize)]
pub struct Output {
    /// The original input
    pub input: String,

    /// Errors, if any
    pub errors: Vec<ErrorOutput>,

    /// Timestamp
    pub executed: DateTime<Utc>,

    /// Time to execute final output
    pub execution_time: i64,

    /// Chat messages to be sent
    pub messages: Vec<String>,

    /// Dice rolls
    pub rolls: Vec<Die>,

    /// Tokens
    pub tokens: Vec<Token>,
 
    /// API Version
    pub version: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorOutput {
    /// Type of error
    error: String,

    /// Message
    message: String
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

// Read a command 
named!(read_command <Vec<&[u8]>>, many0!(do_parse!(
    alt!(tag!("!roll") | tag!("!r")) >>
    command: take_until!(" ") >>
    (command)
)));


/// Parses TTML and executes on-the-fly
// pub fn parse_ttml(input: &str) -> IResult<&[u8], &str> {
    // // parse_macro(input.as_bytes())
// }

#[test]
fn test_take_macro_name() {
    // macro names can be alphanumeric
    let cmd1 = take_macro_name(b"#test !say \"Hello\"");
    let res1 = IResult::Done(&b" !say \"Hello\""[..], "test");
    assert_eq!(cmd1, res1);

    // macro names can have dashes
    let cmd2 = take_macro_name(b"#test-macro !say \"Hello\"");
    let res2 = IResult::Done(&b" !say \"Hello\""[..], "test-macro");
    assert_eq!(cmd2, res2);

    // macro names can have underscores
    let cmd3 = take_macro_name(b"#test_macro !say \"Hello\"");
    let res3 = IResult::Done(&b" !say \"Hello\""[..], "test_macro");
    assert_eq!(cmd3, res3);

    // macro names can start with numbers
    let cmd4 = take_macro_name(b"#123abcZZZ !say \"Hello\"");
    let res4 = IResult::Done(&b" !say \"Hello\""[..], "123abcZZZ");
    assert_eq!(cmd4, res4);

    // macro names can start with capital letters
    let cmd5 = take_macro_name(b"#Zxy123 !say \"Hello\"");
    let res5 = IResult::Done(&b" !say \"Hello\""[..], "Zxy123");
    assert_eq!(cmd5, res5);

    // parse only the first macro when multiple macros in the same line
    let cmd5 = take_macro_name(b"#test !say \"Hello\" #multiple !roll 1d20");
    let res5 = IResult::Done(&b" !say \"Hello\" #multiple !roll 1d20"[..], "test");
    assert_eq!(cmd5, res5);
}

#[test]
fn test_read_command() {
    let cmd = read_command(b"!roll 1d20");
    println!("cmd {}", String::from_utf8_lossy(cmd.unwrap().0));
}
