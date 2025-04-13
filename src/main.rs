use std::io::{stdin,stdout,Write};
use std::fmt;

pub mod command;

use crate::command::{handler,registry};

fn print_prompt() -> () {
    print!("union > ");
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
            registry
                ::CommandRegistry
                ::execute(
                    handler
                    ::CommandHandler
                    ::convert_to_command(s.as_str()
                )
            )
        }

        // check for commands for DML
    }
}
