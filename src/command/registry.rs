use crate::config;
use crate::storage::file;
use crate::storage::model::set::Set;

use super::types::{Commands,UnrecognizedCommandError};

use std::fs::OpenOptions;
use std::path::Path;
use std::io::Write;

pub struct CommandRegistry {
}

impl CommandRegistry {

    pub fn execute(command: Result<Commands, UnrecognizedCommandError>, parameters: Vec<&str>) -> Option<Set> {
        match command {
            Ok(Commands::ExitRepl) => CommandRegistry::exit_repl(),
            Ok(Commands::GetSets) => CommandRegistry::display_sets(),
            Ok(Commands::Help) => CommandRegistry::display_help(),
            Ok(Commands::Init) => CommandRegistry::init_db(parameters),
            Ok(Commands::Load) => CommandRegistry::load_db(parameters),
            Err(UnrecognizedCommandError) => CommandRegistry::command_not_found(),
        }
    }

    fn command_not_found() -> Option<Set> {
        println!("command not found"); 
        None
    }

    fn load_db(parameters: Vec<&str>) -> Option<Set> {
        if parameters.len() < 2 {
            println!("Add a filename after the load command (load filename.union)");
            return None;
        }
 
        assert!(
            !std::fs::exists(parameters[1]).expect("Can't check existence of file does_not_exist.txt")
            );

        let path = Path::new(&parameters[1]);

        let option_loaded_file = file::load_file(&path); 

        let loaded_file = match option_loaded_file {
            Some(loaded_file) => loaded_file,
            _ => return None,
        };

        let loaded_set = Set::load_set_from_file(&loaded_file);

        println!("DB File: {}.union has been loaded", parameters[1]);

        Some(loaded_set)
    }

    fn init_db(parameters: Vec<&str>) -> Option<Set> {
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
        let _ = file.write(config::FILE_MAGIC);
        let _ = file.write_all(&[config::FILE_VERSION]);

        // Initially, no bitmaps; write zero index count (u32)
        let _ = file.write_all(&0u32.to_le_bytes());
 
        println!("DB File: {}.union has been created", parameters[1]);

        Some(Set::new(&filename))
    }

    fn display_sets() -> Option<Set> {
        println!("Sets: ");
        None
    }

    fn exit_repl() -> Option<Set> {
        std::process::exit(0);
    }

    fn display_help() -> Option<Set> {
        println!("");
        None
    }
    
}
