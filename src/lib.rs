#[macro_use] extern crate serde_derive;
#[macro_use] extern crate nom;
extern crate chrono;
extern crate rand;
extern crate uuid;
extern crate libc;

pub mod die;
pub mod dom;
pub mod errors;
pub mod executor;
pub mod output;
pub mod parser;
pub mod roll;
pub mod token;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
