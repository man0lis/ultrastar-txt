//! # Ultrastar TXT Library
//! This is a small library that is able to parse and generate song files for the open source karaoke game Ultrastar.
#![deny(missing_docs)]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;
extern crate regex;
#[cfg(feature = "url-support")]
extern crate url;

/// this module contains the generator
pub mod generator;
/// this module contains the parser
pub mod parser;
/// this module contains the structs that represent the parsed data
pub mod structs;

#[cfg(feature = "file-support")]
/// this module contains functions to parse songs from a path
pub mod loader;

pub use crate::generator::*;
pub use crate::parser::*;
pub use crate::structs::*;

#[cfg(feature = "file-support")]
pub use crate::loader::*;
