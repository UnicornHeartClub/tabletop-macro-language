extern crate serde;
extern crate serde_json;
extern crate ttml;

use std::mem;
use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_void};
use ttml::parser::parse_p;

fn main() {}

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
        
/// Run input and return a typed array for use in javascript
#[no_mangle]
pub extern "C" fn parse(raw_input: *mut c_char) -> *mut c_char {
    // Parse the input
    let input = safe_string(raw_input);

    let prog = parse_p(input.as_slice());

    // Parse the macro
    if prog.is_err() {
        // Push the error
        let err = prog.unwrap_err();
        // let json = serde_json::to_string(&err).unwrap();

        CString::new("Unexpected error (for now)").unwrap().into_raw()
    } else {
        let (_, program) = prog.unwrap();

        // Return as JSON
        let json = serde_json::to_string(&program).unwrap();
        CString::new(json).unwrap().into_raw()
    }
}

fn safe_string(input: *mut c_char) -> Vec<u8> {
    unsafe {
        CStr::from_ptr(input).to_bytes().to_owned()
    }
}
