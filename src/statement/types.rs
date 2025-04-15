use std::fmt;

use super::operations::OperationType;


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


pub fn convert_symbol_to_type(symbol: &str) -> Result<OperationType, InvalidOperationError> {
    match symbol {
        "\\minus" => Ok(OperationType::Minus),
        "\\union" => Ok(OperationType::Union),
        "\\inter" => Ok(OperationType::Intersect),
        _ => Err(InvalidOperationError)
    }
}

pub fn convert_type_to_symbol(operation_type: &OperationType) -> String {
    match operation_type {
        OperationType::Minus => "\\minus".to_string(),
        OperationType::Union => "\\union".to_string(),
        OperationType::Intersect => "\\inter".to_string(),
        OperationType::Placeholder => "\\placeholder".to_string(),
    }
}
