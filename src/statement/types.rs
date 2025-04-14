use std::fmt;

pub enum OperationType {
    Minus,
    Intersect,
    Union
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
