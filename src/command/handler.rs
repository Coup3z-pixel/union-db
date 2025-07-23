use super::types::{Commands,UnrecognizedCommandError};

pub struct CommandHandler {
}

impl CommandHandler {
    pub fn convert_to_command(command_str: &str) -> Result<Commands, UnrecognizedCommandError> {
        match command_str {
            ".exit" => Ok(Commands::ExitRepl),
            ".sets" => Ok(Commands::GetSets),
            ".help" => Ok(Commands::Help),
            ".init" => Ok(Commands::Init),
            _ => Err(UnrecognizedCommandError)

        }
    }   
}
