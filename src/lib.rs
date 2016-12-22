#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
extern crate regex;
extern crate rustc_serialize;

mod decoder;

pub mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain! { }
}
