// setup limit for error-chain
#![recursion_limit = "1024"]

// #[macro_use] extern crate error_chain;
#[macro_use] extern crate serde_derive;
extern crate chrono;
extern crate serde;
extern crate serde_json;
extern crate ttml;

use chrono::DateTime;
use chrono::prelude::Utc;
use std::ffi::CString;
use std::ffi::CStr;
use std::os::raw::c_char;
use ttml::die::Die;
// use ttml::parser::execute_ast;
// use ttml::parser::parse_ttml;
use ttml::token::Token;

pub fn main() {
    // IGNORE ME!
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Output {
    /// The original input
    pub input: String,

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

pub fn safe_string(input: *mut c_char) -> String {
    unsafe {
        CStr::from_ptr(input).to_string_lossy().into_owned()
    }
}

/// Run input and return a typed array for use in javascript
#[no_mangle]
pub fn parse(raw_input: *mut c_char) -> *mut c_char {
    let input = safe_string(raw_input);
    let output = Output {
        input,
        executed: Utc::now(),
        execution_time: 0,
        messages: Vec::new(),
        rolls: Vec::new(),
        tokens: Vec::new(),
        version: String::from("0.1.0"),
    };

    let json = serde_json::to_string(&output).unwrap();
    CString::new(json).unwrap().into_raw()
}

#[test]
fn it_parses_input() {
    let chars = CString::new("#test!say \"Hello\"").unwrap().into_raw();
    let raw_output = parse(chars);
    let json = safe_string(raw_output);
    let output: Output = serde_json::from_str(&json).unwrap();

    assert_eq!(output.input, "#test!say \"Hello\"");
    assert_eq!(output.version, "0.1.0");
}
