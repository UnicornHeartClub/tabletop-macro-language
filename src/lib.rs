// Compiling for the web requires some extra modules
#[macro_use] extern crate nom;
#[macro_use] extern crate serde_derive;

extern crate serde_json;
extern crate serde;

pub mod arg;
pub mod output;
pub mod parser;
pub mod step;

use nom::Err::Error;
use nom::simple_errors::Context::Code;

#[derive(Debug, Serialize, Deserialize)]
struct OutputError {
    error: String,
    context: String,
}

use std::mem;
use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_void};
use parser::parse_p;

// In order to work with the memory we expose (de)allocation methods
#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    return ptr as *mut c_void;
}

#[no_mangle]
pub extern "C" fn dealloc_str(ptr: *mut c_char) {
    unsafe {
        let _ = CString::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut c_void, cap: usize) {
    unsafe  {
        let _buf = Vec::from_raw_parts(ptr, 0, cap);
    }
}
        
/// Run input and return a typed array for use in javascript
#[no_mangle]
pub extern "C" fn parse(raw_input: *mut c_char) -> *mut c_char {
    // covert the input
    let input = safe_string(raw_input);

    // run the parser
    let json = run(input);

    // return the result
    CString::new(json).unwrap().into_raw()
}

/// Run the internal parser and return a program or error
fn run (input: Vec<u8>) -> String {
    let prog = parse_p(input.as_slice());
    if prog.is_err() {
        // Push the error
        let error = prog.unwrap_err();

        let mut output_error = OutputError {
            error: format!("{:?}", error),
            context: "".to_string(),
        };

        if let Error(Code(list, _code)) = error {
            let context = String::from_utf8(list.to_vec()).unwrap();
            output_error.context = context;
        }

        // return error JSON
        serde_json::to_string(&output_error).unwrap()
    } else {
        let (_, program) = prog.unwrap();

        // return success JSON
        serde_json::to_string(&program).unwrap()
    }
}

fn safe_string(input: *mut c_char) -> Vec<u8> {
    unsafe {
        CStr::from_ptr(input).to_bytes().to_owned()
    }
}

#[test]
fn test_error_handling() {
    // missing ending single quotation
    let input: Vec<u8> = "#test !prompt test [ok, not 'ok]".as_bytes().to_vec();
    let result = run(input);

    let json: OutputError = serde_json::from_str(&result).unwrap();
    assert_eq!(json.context, "[ok, not 'ok]".to_string());

    // missing prompt input
    let input: Vec<u8> = "#test !prompt [test, tester, testest]".as_bytes().to_vec();
    let result = run(input);

    let json: OutputError = serde_json::from_str(&result).unwrap();
    assert_eq!(json.context, "[test, tester, testest]".to_string());
}
