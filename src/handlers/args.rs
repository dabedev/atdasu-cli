use std::env;

const NAME: &str = env!("CARGO_PKG_NAME");

fn parse_args() -> Vec<String> {
    let mut arguments: Vec<String> = env::args().collect();
    arguments.remove(0);
    arguments
}

pub fn parse_command() -> Result<String, String> {
    let args: Vec<String> = parse_args();
    let error_msg: String = format!("No command was provided. Try using {} help.", NAME);
    let command: Option<&String> = args.get(0);
    if let Some(command) = command {
        Ok(command.to_owned())
    } else {
        Err(error_msg)
    }
}

pub fn parse_command_args() -> Vec<String> {
    let mut args: Vec<String> = parse_args();
    args.remove(0);
    args
}
