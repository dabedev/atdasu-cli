use std::env;

fn parse_args() -> Vec<String> {
    let mut arguments: Vec<String> = env::args().collect();
    arguments.remove(0);
    arguments
}

pub fn parse_command() -> Result<String, String> {
    let args: Vec<String> = parse_args();
    let command: String = args.get(0).expect("No arguments were provided.").to_string();
    Ok(command)
}
