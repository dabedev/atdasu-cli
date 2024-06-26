use dasu::handlers::{ self, parse_command_args, Command, CommandManager, Manager };
use dasu::commands::{ version, help, run };
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
        "run" => run(command_args),
        "version" => version(),
        "help" => help(command_args, manager),
        _ => {
            let error_msg: String = format!("No command was provided. Try using {} help.", NAME);
            println!("{}", error_msg);
            process::exit(1)
        }
    }
}
