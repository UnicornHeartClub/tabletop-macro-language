#[macro_use] extern crate error_chain;
#[macro_use] extern crate serde_derive;
extern crate chrono;
extern crate rand;
extern crate uuid;

pub mod die;
pub mod errors;
pub mod parser;
pub mod roll;
pub mod token;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
