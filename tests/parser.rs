extern crate ttml;
extern crate nom;

use nom::IResult;
use ttml::parser::parse_ttml;

#[test]
fn it_parses_valid_macro_name() {
    let ast1 = parse_ttml("#test !say \"Hello\"");
    let res1 = IResult::Done(&b" !say \"Hello\""[..], "test");
    assert_eq!(ast1, res1);

    let ast2 = parse_ttml("#test-macro !say \"Hello\"");
    let res2 = IResult::Done(&b" !say \"Hello\""[..], "test-macro");
    assert_eq!(ast2, res2);

    let ast3 = parse_ttml("#test_macro !say \"Hello\"");
    let res3 = IResult::Done(&b" !say \"Hello\""[..], "test_macro");
    assert_eq!(ast3, res3);

    let ast4 = parse_ttml("#123abcZZZ !say \"Hello\"");
    let res4 = IResult::Done(&b" !say \"Hello\""[..], "123abcZZZ");
    assert_eq!(ast4, res4);

    let ast5 = parse_ttml("#Zxy123 !say \"Hello\"");
    let res5 = IResult::Done(&b" !say \"Hello\""[..], "Zxy123");
    assert_eq!(ast5, res5);
}

// fn it_parses_roll_command() {
// }

// fn it_parses_say_command() {
// }

// fn it_parses_whisper_command() {
// }
