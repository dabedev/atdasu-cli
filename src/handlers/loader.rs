use toml;
use std::fs;
use crate::handlers::commands::{ Command, CommandManager, Manager };
use serde::Deserialize;

const NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Debug, Deserialize)]
struct CommandData {
    name: String,
    alias: Vec<String>,
    description: String,
    usage: String,
    priority: u8,
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

                let mut parsed_data: CommandData = toml
                    ::from_str(&command_data)
                    .expect("Failed to parse TOML data");

                parsed_data.usage = parsed_data.usage.replace("%cli%", NAME);
                parsed_data.usage = parsed_data.usage.replace("%cmd_name%", &parsed_data.name);

                let command = Command {
                    name: parsed_data.name,
                    alias: parsed_data.alias,
                    description: parsed_data.description,
                    usage: parsed_data.usage,
                    priority: parsed_data.priority,
                };

                manager.add_command(command);
            }
        }
    }
}
