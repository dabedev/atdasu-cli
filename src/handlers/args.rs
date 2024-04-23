use std::env;

const NAME: &str = env!("CARGO_PKG_NAME");

fn parse_args() -> Vec<String> {
    let mut arguments: Vec<String> = env::args().collect();
    arguments.remove(0);
    arguments
}

pub fn parse_command() -> Result<String, String> {
    let args: Vec<String> = parse_args();
    let error_msg: String = format!("No command was provided. Try using {NAME} help.");
    let command: String = args.get(0).expect(error_msg.as_str()).to_string();
    Ok(command)
}
