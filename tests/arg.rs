extern crate ttml;

use ttml::arg::TokenArg;

#[test]
fn token_arg_to_string() {
    let token = TokenArg {
        name: "test".to_string(),
        attribute: Some("test_attr".to_string()),
        macro_name: None,
    };
    assert_eq!(token.to_string(), "@test.test_attr".to_string());

    let token = TokenArg {
        name: "test".to_string(),
        attribute: None,
        macro_name: Some("macro_name".to_string()),
    };
    assert_eq!(token.to_string(), "@test->macro_name".to_string());

    let token = TokenArg {
        name: "test_token".to_string(),
        attribute: None,
        macro_name: None,
    };
    assert_eq!(token.to_string(), "@test_token".to_string());
}
