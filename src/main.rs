// setup limit for error-chain
#![recursion_limit = "1024"]

// #[macro_use] extern crate error_chain;
extern crate chrono;
extern crate serde;
extern crate serde_json;
extern crate ttml;

use chrono::prelude::Utc;
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;
use ttml::output::Output;

pub fn main() {
    // IGNORE ME!
}

pub fn safe_string(input: *mut c_char) -> String {
    unsafe {
        CStr::from_ptr(input).to_string_lossy().into_owned()
    }
}

/// Run input and return a typed array for use in javascript
#[no_mangle]
pub fn parse(raw_input: *mut c_char) -> *mut c_char {
    // Take the input and safely covert it to a String
    let input = safe_string(raw_input);

    let executed = Utc::now();

    let errors = Vec::new();
    let messages = Vec::new();
    let rolls = Vec::new();
    let tokens = Vec::new();
    let version = String::from("0.1.0");

    // let results = parse_macro(input.as_bytes());
    // match results {
        // IResult::Done(_, x) => println!("Parsing succeeded {}", String::from_utf8_lossy(x)),
        // _ => println!("Error while parsing!"),
    // }

    let finished = Utc::now().timestamp();
    let execution_time = finished - executed.timestamp();

    let output = Output {
        input: String::from(input),
        executed,
        execution_time,
        errors,
        messages,
        rolls,
        tokens,
        version,
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
