use errors::*;
use nom::IResult;
use std::str;

// Read "#" and take until the first whitespace
named!(take_macro_name <&str>, do_parse!(
    tag!("#") >>
    name: map_res!(
        is_not!(" \t\r\n"),
        str::from_utf8
    ) >>
    (name)
));

/// Parses TTML and executes on-the-fly
pub fn parse_ttml(input: &str) -> IResult<&[u8], &str> {
    take_macro_name(input.as_bytes())
}

#[test]
fn test_take_macro_name() {
    // macro names can be alphanumeric
    let ast1 = take_macro_name("#test !say \"Hello\"".as_bytes());
    let res1 = IResult::Done(&b" !say \"Hello\""[..], "test");
    assert_eq!(ast1, res1);

    // macro names can have dashes
    let ast2 = take_macro_name("#test-macro !say \"Hello\"".as_bytes());
    let res2 = IResult::Done(&b" !say \"Hello\""[..], "test-macro");
    assert_eq!(ast2, res2);

    // macro names can have underscores
    let ast3 = take_macro_name("#test_macro !say \"Hello\"".as_bytes());
    let res3 = IResult::Done(&b" !say \"Hello\""[..], "test_macro");
    assert_eq!(ast3, res3);

    // macro names can start with numbers
    let ast4 = take_macro_name("#123abcZZZ !say \"Hello\"".as_bytes());
    let res4 = IResult::Done(&b" !say \"Hello\""[..], "123abcZZZ");
    assert_eq!(ast4, res4);

    // macro names can start with capital letters
    let ast5 = take_macro_name("#Zxy123 !say \"Hello\"".as_bytes());
    let res5 = IResult::Done(&b" !say \"Hello\""[..], "Zxy123");
    assert_eq!(ast5, res5);

    // parse only the first macro when multiple macros in the same line
    let ast5 = take_macro_name("#test !say \"Hello\" #multiple !roll 1d20".as_bytes());
    let res5 = IResult::Done(&b" !say \"Hello\" #multiple !roll 1d20"[..], "test");
    assert_eq!(ast5, res5);
}

#[test]
fn test_roll_command() {
}
