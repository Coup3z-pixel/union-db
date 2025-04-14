use std::io::{stdin,stdout,Write};
use std::fmt;

pub mod command;
pub mod statement;

use crate::command::{handler::CommandHandler,registry::CommandRegistry};
use crate::statement::{compiler::StatementCompiler,executor::StatementExecutor};

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
    let database_executor = StatementExecutor::new();

    loop {
        
        let mut s = String::new();

        print_prompt();
        get_input(&mut s);

        // check for commands with .#### at the front
        if s.chars().nth(0) == Some('.') {
            CommandRegistry::execute(
                CommandHandler::convert_to_command(s.as_str()
            ));

            continue;
        }

        // check for commands for Database Language
        if s.chars().last() == Some(';') {
            let compiled_statement_result = StatementCompiler::compile_from_str(
                &s.as_str()[0..s.len()-1]
            );

            let _db_result = match compiled_statement_result {
                Ok(statement) => database_executor.execute(statement),
                Err(statement::types::StatementCompilationError) => continue,
            };
        }
    }
}
