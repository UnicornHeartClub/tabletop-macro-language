extern crate ttml;

use ttml::parser::parse_ttml;

#[test]
fn it_parses_valid_tml() {
    let ast = parse_ttml("#test\n!say \"Hello, tests!\"");
}

// fn it_parses_roll_command() {
// }

// fn it_parses_say_command() {
// }

// fn it_parses_whisper_command() {
// }
