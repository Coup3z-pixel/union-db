use super::types::{Commands,UnrecognizedCommandError};

pub struct CommandRegistry {
}

impl CommandRegistry {

    fn display_sets() {
        print!("Sets: ")
    }

    fn exit_repl() {
        std::process::exit(0)
    }

    pub fn execute(command: Result<Commands, UnrecognizedCommandError>) {
        match command {
            Ok(Commands::ExitRepl) => CommandRegistry::exit_repl(),
            Ok(Commands::GetSets) => CommandRegistry::display_sets(),
            Err(UnrecognizedCommandError) => print!("command not found"),
        }
    }
    
    
}
