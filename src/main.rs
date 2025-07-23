use std::io::{stdin,stdout,Write};
use std::fmt;
use std::env;
use std::fs::File;

pub mod command;
pub mod statement;
pub mod storage;

use storage::model::store::Store;

use crate::command::{handler::CommandHandler,registry::CommandRegistry};
use crate::statement::interpreter::StatementInterpreter;

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

    let args: Vec<String> = env::args().collect();
    let filepath: File;

    print!("Union DB Version 0.0.1\n");

    if args.len() == 2 {
        println!("Retrieving sets from {}", args[1]);
        // check if file exists

    }

    loop {
        
        let mut s = String::new();

        print_prompt();
        get_input(&mut s);

        // check for commands with .#### at the front
        if s.chars().nth(0) == Some('.') {
            let first_space = s.find(' ');

            let argument_space_position = match first_space {
                Some(x) => x,
                None => s.len(),
            };

            let _ = CommandRegistry::execute(
                CommandHandler::convert_to_command(&s.as_str()[0..argument_space_position]),
                s.as_str().split_whitespace().collect()
            );

            continue;
        }

        // check for commands for Database Language
        if s.chars().last() == Some(';') {
            let compiled_statement_result = StatementInterpreter::compile_from_str(
                &s.as_str()[0..s.len()-1]
            );

            let _db_result = match compiled_statement_result {
                Ok(mut statement) => statement.add_store(Store::new()).execute(),
                Err(statement::types::StatementCompilationError) => continue,
            };
        }
    }
}
