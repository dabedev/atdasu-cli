use dasu::handlers::{ self, Manager, Command, CommandManager };
use std::process;

fn main() {
    let mut manager: CommandManager = CommandManager::new();
    handlers::load_commands(&mut manager);
    let command: String = handlers::parse_command().unwrap_or_else(|_err| {
        println!("No command was provided.");
        process::exit(1)
    });
    let command: &Command = manager.get_command(command).unwrap();
    println!("{}", command.name)
}
