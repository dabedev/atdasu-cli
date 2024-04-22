use toml;
use std::fs;
use crate::handlers::commands::{ Command, CommandManager, Manager };
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct CommandData {
    name: String,
    shortname: String,
}

pub fn load_commands(manager: &mut CommandManager) {
    let commands_path = "src/commands/";
    let command_files = fs::read_dir(commands_path).expect("Failed to read commands directory.");

    for file in command_files {
        if let Ok(entry) = file {
            let path = entry.path();

            if path.is_file() && path.extension().unwrap_or_default() == "toml" {
                let command_data = fs
                    ::read_to_string(&path)
                    .expect("Failed to read command data from file");

                let parsed_data: CommandData = toml
                    ::from_str(&command_data)
                    .expect("Failed to parse TOML data");

                let command = Command {
                    name: parsed_data.name,
                    shortname: parsed_data.shortname,
                };

                manager.add_command(command);
            }
        }
    }
}
