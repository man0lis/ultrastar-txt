//! # Ultrastar TXT Library
//! This is a small library that is able to parse and generate song files for the open source karaoke game Ultrastar.
#![deny(missing_docs)]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;
extern crate regex;

mod parser;
mod structs;
mod generator;

#[cfg(feature = "file-support")]
mod loader;

pub use parser::*;
pub use structs::*;
pub use generator::*;

#[cfg(feature = "file-support")]
pub use loader::*;
