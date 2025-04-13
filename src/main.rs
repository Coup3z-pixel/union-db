use std::io::{stdin,stdout,Write};
use std::fmt;

pub mod command;
pub mod statement;

use crate::command::{handler,registry};
use crate::statement::{compiler,executor};

fn print_prompt() -> () {
    print!("union > ");
}

fn get_input(s: &mut String) -> () {
    let _ = stdout().flush();

    stdin().read_line(s).expect("Did not enter a correct string");

    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }

    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
}

fn main() {

    print!("Union DB Version 0.0.1\n");

    loop {
        
        let mut s = String::new();

        print_prompt();

        get_input(&mut s);

        // check for commands with .#### at the front
        if s.chars().nth(0) == Some('.') {
            registry
                ::CommandRegistry
                ::execute(
                    handler
                    ::CommandHandler
                    ::convert_to_command(s.as_str()
                )
            );

            continue;
        }

        // check for commands for Database Language
        if s.chars().last() == Some(';') {
        }
    }
}
