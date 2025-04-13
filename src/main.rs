use std::io::{stdin,stdout,Write};
use std::fmt;

pub mod command;

// Custom error type; can be any type which defined in the current crate
// 💡 In here, we use a simple "unit struct" to simplify the example
#[derive(Debug)]
struct UnrecognizedCommandError;

// Implement std::fmt::Display for AppError
impl fmt::Display for UnrecognizedCommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An Error Occurred, Please Try Again!") // user-facing output
    }
}

enum Commands {
    GetSets,
    ExitRepl,
}

fn print_prompt() -> () {
    print!("union > ");
}

fn convert_to_command(command_str: &str) -> Result<Commands, UnrecognizedCommandError> {
    match command_str {
        ".exit" => Ok(Commands::ExitRepl),
        ".sets" => Ok(Commands::GetSets),
        _ => Err(UnrecognizedCommandError)

    }
}

fn display_sets() {
    print!("Sets: ")
}

fn main() {

    print!("Union DB Version 0.0.1\n");

    loop {
        
        let mut s = String::new();

        print_prompt();

        let _ = stdout().flush();

        stdin().read_line(&mut s).expect("Did not enter a correct string");

        if let Some('\n')=s.chars().next_back() {
            s.pop();
        }
        if let Some('\r')=s.chars().next_back() {
            s.pop();
        }


        // check for commands with .#### at the front
        if s.chars().nth(0) == Some('.') {
            let command = convert_to_command(s.as_str());

            match command {
                Ok(Commands::ExitRepl) => std::process::exit(0),
                Ok(Commands::GetSets) => display_sets(),
                Err(UnrecognizedCommandError) => continue,
            }
            
            continue;
        }

        

        // check for commands for DML
    }
}
