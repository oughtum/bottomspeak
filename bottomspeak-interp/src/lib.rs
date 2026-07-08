use std::error::Error;

pub(crate) mod diagnostics;
pub(crate) mod env;
pub mod interpreter;
pub(crate) mod lexer;
pub(crate) mod parser;
pub(crate) mod source;
pub(crate) mod vm;

pub(crate) type Result<T> = std::result::Result<T, Box<dyn Error>>;
