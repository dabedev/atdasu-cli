use dasu::handlers::{ self, parse_command_args, Command, CommandManager, Manager };
use dasu::commands::{ test, help };
use std::process;

const NAME: &str = env!("CARGO_PKG_NAME");

fn main() {
    let mut manager: CommandManager = CommandManager::new();
    handlers::load_commands(&mut manager);
    let command: String = handlers::parse_command().unwrap_or_else(|_err| {
        println!("No command was provided.");
        process::exit(1)
    });
    let command: &Command = manager.get_command(command).expect("No valid command was provided.");
    let command: &str = command.name.as_str();
    let command_args: Vec<String> = parse_command_args();
    match command {
        "test" => test(),
        "help" => help(command_args, manager),
        _ => {
            println!("No command was provided. Try using {NAME} ? or {NAME} help.");
            process::exit(1)
        }
    }
}
