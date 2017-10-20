extern crate ttml;

use ttml::die::Die;
use ttml::die::DieType;

#[test]
fn create_die_with_no_token() {
    let die = Die::new(20, None);
    assert_eq!(die.die, DieType::d20);
}

#[test]
fn create_die_with_token() {
    let token = "ABC";
    let die = Die::new(20, Some(token.to_owned()));
    assert_eq!(die.die, DieType::d20);
    assert_eq!(die.token, Some(token.to_owned()));
}
