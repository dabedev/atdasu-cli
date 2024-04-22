use super::handlers::{ self, Manager, Command, CommandManager };
use super::commands::{ test };
use std::process;

pub async fn menu() {
    let mut manager: CommandManager = CommandManager::new();
    handlers::load_commands(&mut manager);
    let command: String = handlers::parse_command().unwrap_or_else(|_err| {
        println!("No command was provided.");
        process::exit(1)
    });
    let command: &Command = manager.get_command(command).unwrap();
    let command: &str = command.name.as_str();
    match command {
        "test" => test().await,
    }
}
