use toml;
use std::fs;
use std::path::PathBuf;
use crate::handlers::commands::{ Command, CommandManager, Manager };

pub fn load_commands(manager: &mut CommandManager) {
    let commands_path: String = String::from("src/commands/");
    let command_files = fs
        ::read_dir(commands_path.clone())
        .expect("Failed to read commands directory.");

    for file in command_files {
        if let Ok(entry) = file {
            let path: PathBuf = entry.path();

            if path.is_file() && path.extension().unwrap_or_default() == "rs" {
                let command_name: String = path.file_stem().unwrap().to_string_lossy().into_owned();
                let command_cfg_path: String = format!("{commands_path}{command_name}.toml");
                let command_data: String = fs
                    ::read_to_string(&command_cfg_path)
                    .unwrap_or(command_cfg_path);
                let command_data: String = toml::from_str(&command_data.to_string()).unwrap();
                println!("{:#?}", command_data);
                if command_name == "loader" {
                    continue;
                }

                let command: Command = Command {
                    name: command_name.clone(),
                    shortname: String::new(),
                };
                manager.add_command(command);
            }
        }
    }
}
