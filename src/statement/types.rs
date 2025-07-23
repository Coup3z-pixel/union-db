use std::fmt;

use super::operations::{definition::{define::Define, remove::Remove}, manipulation::{append::Append, intersect::Intersect, minus::Minus, union::Union}, Operation};

pub fn convert_symbol_to_type(symbol: &str) -> Result<Box<dyn Operation>, InvalidOperationError> {
    match symbol {
        "\\m" => Ok(Box::new(Minus ())),
        "\\u" => Ok(Box::new(Union ())),
        "\\i" => Ok(Box::new(Intersect ())),
        "\\d" => Ok(Box::new(Define ())),
        "\\r" => Ok(Box::new(Remove ())),
        "\\a" => Ok(Box::new(Append ())),
        _ => Err(InvalidOperationError)
    }
}

#[derive(Debug)]
pub struct StatementExecutionError;

// Implement std::fmt::Display for AppError
impl fmt::Display for StatementExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An Error Occurred, Please Try Again!") // user-facing output
    }
}

#[derive(Debug)]
pub struct InvalidOperationError;

// Implement std::fmt::Display for AppError
impl fmt::Display for InvalidOperationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An Error Occurred, Please Try Again!") // user-facing output
    }
}

#[derive(Debug)]
pub struct ParseTreeError;

// Implement std::fmt::Display for ParseTreeError
impl fmt::Display for ParseTreeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error parsing expression into a subtree") // user-facing output
    }
}

#[derive(Debug)]
pub struct StatementCompilationError;

impl fmt::Display for StatementCompilationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error compiling statement into a tree") // user-facing output
    }
}
