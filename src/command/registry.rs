use super::types::{Commands,UnrecognizedCommandError};

use std::fs::OpenOptions;
use std::path::Path;
use std::io::Write;
use std::fs::File;

pub struct CommandRegistry {
}


const FILE_MAGIC: &[u8; 4] = b"RBMP";
const FILE_VERSION: u8 = 1;

impl CommandRegistry {

    pub fn execute(command: Result<Commands, UnrecognizedCommandError>, parameters: Vec<&str>) -> Option<File> {
        match command {
            Ok(Commands::ExitRepl) => CommandRegistry::exit_repl(),
            Ok(Commands::GetSets) => CommandRegistry::display_sets(),
            Ok(Commands::Help) => CommandRegistry::display_help(),
            Ok(Commands::Init) => CommandRegistry::init_file(parameters),
            Ok(Commands::Load) => CommandRegistry::load_file(parameters),
            Err(UnrecognizedCommandError) => CommandRegistry::command_not_found(),
        }
    }

    fn command_not_found() -> Option<File> {
        println!("command not found"); 
        None
    }

    fn load_file(parameters: Vec<&str>) -> Option<File> {
        if parameters.len() < 2 {
            println!("Add a filename after the load command (load filename.union)");
            return None;
        }

        let path = Path::new(&parameters[1]);

        let file_option = OpenOptions::new()
            .write(true)
            .read(true)
            .create(false)
            .truncate(true)
            .open(path);

        let mut file = match file_option {
            Ok(file_instance) => file_instance,
            Err(_) => return None,
        };

        // Initially, no bitmaps; write zero index count (u32)
        let _ = file.write_all(&0u32.to_le_bytes());
 
        println!("DB File: {}.union has been created", parameters[1]);

        Some(file)
    }

    fn init_file(parameters: Vec<&str>) -> Option<File> {
        if parameters.len() < 2 {
            println!("Add a filename after the init command (init filename)");
            return None;
        }

        let filename = String::from(format!("{}{}", parameters[1], ".union"));
        let path = Path::new(&filename);

        let file_option = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path);

        let mut file = match file_option {
            Ok(file_instance) => file_instance,
            Err(_) => return None,
        };


        // Write header (magic + version)
        let _ = file.write(FILE_MAGIC);
        let _ = file.write_all(&[FILE_VERSION]);

        // Initially, no bitmaps; write zero index count (u32)
        let _ = file.write_all(&0u32.to_le_bytes());
 
        println!("DB File: {}.union has been created", parameters[1]);

        Some(file)
    }

    fn display_sets() -> Option<File> {
        println!("Sets: ");
        None
    }

    fn exit_repl() -> Option<File> {
        std::process::exit(0);
    }

    fn display_help() -> Option<File> {
        println!("");
        None
    }
    
}
