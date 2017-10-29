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
use ttml::parser::Output;

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

    // Parse and execute the macro
    // @todo

    // Send the final output
    let output = Output {
        input,
        executed: Utc::now(),
        execution_time: 0,
        messages: Vec::new(),
        errors: Vec::new(),
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
