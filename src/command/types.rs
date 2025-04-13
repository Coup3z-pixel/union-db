use crate::fmt;

// Custom error type; can be any type which defined in the current crate
// 💡 In here, we use a simple "unit struct" to simplify the example
#[derive(Debug)]
pub struct UnrecognizedCommandError;

// Implement std::fmt::Display for AppError
impl fmt::Display for UnrecognizedCommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An Error Occurred, Please Try Again!") // user-facing output
    }
}

pub enum Commands {
    Help,
    GetSets,
    ExitRepl,
}
