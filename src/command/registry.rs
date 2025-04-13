use super::types::{Commands,UnrecognizedCommandError};

pub struct CommandRegistry {
}

impl CommandRegistry {

    fn display_sets() {
        println!("Sets: ")
    }

    fn exit_repl() {
        std::process::exit(0)
    }

    fn display_help() {
        println!("")
    }

    pub fn execute(command: Result<Commands, UnrecognizedCommandError>) {
        match command {
            Ok(Commands::ExitRepl) => CommandRegistry::exit_repl(),
            Ok(Commands::GetSets) => CommandRegistry::display_sets(),
            Ok(Commands::Help) => CommandRegistry::display_help(),
            Err(UnrecognizedCommandError) => println!("command not found"),
        }
    }
    
    
}
