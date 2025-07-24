use std::io::{stdin,stdout,Write};
use std::fmt;
use std::env;

pub mod command;
pub mod statement;
pub mod storage;
pub mod config;

use statement::types::StatementExecutionError;
use storage::file;
use storage::model::set::Set;
use storage::model::store::Store;
use statement::node::Node;

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
    let mut current_set: Option<Set> = None;
    let mut store: Store = Store::new();

    print!("Union DB Version 0.0.1\n");

    if args.len() == 2 {
        println!("Retrieving sets from {}", args[1]);
        // check if file exists
        let optional_file = file::load_file(std::path::Path::new(&args[1]));

        let file_instance = match optional_file {
            Some(loaded_file) => loaded_file,
            _ => return,  
        };

        current_set = Some(Set::load_set_from_file(&file_instance));
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
                _ => s.len(),
            };

            let optional_set = CommandRegistry::execute(
                CommandHandler::convert_to_command(&s.as_str()[0..argument_space_position]),
                s.as_str().split_whitespace().collect()
            );

            if optional_set.is_some() {
                current_set = optional_set;
            }

            continue;
        }

        // check for commands for Database Language
        if s.chars().last() == Some(';') {
            let mut statement;

            let compiled_statement_result = StatementInterpreter::compile_from_str(
                &s.as_str()[0..s.len()-1]
            );

            // Assign statement
            let compiled = match compiled_statement_result {
                Ok(stmt) => {
                    statement = stmt;
                    true
                },
                Err(statement::types::StatementCompilationError) => {
                    continue;
                }
            };

            if current_set.is_some() && compiled {
                let statement_set = current_set.clone().unwrap();

                let db_result = statement.add_set_to_store(statement_set).execute();

                match db_result {
                    Ok(node) => match node {
                        Node::Set(name) => { 
                            let result_set = statement.retrieve_set_from_store(name);

                            match result_set {
                                Some(set) => println!("{}", set),
                                _ => println!("Unexpected Error")
                            }
                        },
                        _ => println!("failed execuation")
                    },
                    Err(StatementExecutionError) => println!("Statement has failed to execute"),
                }
            }
        }
    }
}
