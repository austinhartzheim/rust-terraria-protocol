#![feature(trace_macros)]
#![recursion_limit = "1024"]
#[macro_use]
extern crate nom;

pub mod types;
mod parser;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
