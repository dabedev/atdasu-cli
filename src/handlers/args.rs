use std::env;

use crate::handlers::CommandManager;

pub fn get_args() -> Vec<String> {
    let mut arguments: Vec<String> = env::args().collect();
    arguments.remove(0);
    arguments
}

pub fn get_command(args: Vec<String>) -> Result<String, String> {
    let command: String = args.get(0).expect("No arguments were provided.").to_string();
    Ok(command)
}
