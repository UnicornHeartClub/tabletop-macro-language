extern crate ttml;
extern crate nom;

use nom::IResult;
use ttml::parser::parse_ttml;

#[test]
fn it_parses_valid_macro_name() {
    // macro names can be alphanumeric
    let ast1 = parse_ttml("#test !say \"Hello\"");
    let res1 = IResult::Done(&b" !say \"Hello\""[..], "test");
    assert_eq!(ast1, res1);

    // macro names can have dashes
    let ast2 = parse_ttml("#test-macro !say \"Hello\"");
    let res2 = IResult::Done(&b" !say \"Hello\""[..], "test-macro");
    assert_eq!(ast2, res2);

    // macro names can have underscores
    let ast3 = parse_ttml("#test_macro !say \"Hello\"");
    let res3 = IResult::Done(&b" !say \"Hello\""[..], "test_macro");
    assert_eq!(ast3, res3);

    // macro names can start with numbers
    let ast4 = parse_ttml("#123abcZZZ !say \"Hello\"");
    let res4 = IResult::Done(&b" !say \"Hello\""[..], "123abcZZZ");
    assert_eq!(ast4, res4);

    // macro names can start with capital letters
    let ast5 = parse_ttml("#Zxy123 !say \"Hello\"");
    let res5 = IResult::Done(&b" !say \"Hello\""[..], "Zxy123");
    assert_eq!(ast5, res5);

    // parse only the first macro when multiple macros in the same line
    let ast5 = parse_ttml("#test !say \"Hello\" #multiple !roll 1d20");
    let res5 = IResult::Done(&b" !say \"Hello\" #multiple !roll 1d20"[..], "test");
    assert_eq!(ast5, res5);
}

// fn it_parses_roll_command() {
// }

// fn it_parses_say_command() {
// }

// fn it_parses_whisper_command() {
// }
