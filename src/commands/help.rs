use crate::handlers::Manager;

use crate::handlers::{ CommandManager, Command };

const NAME: &str = env!("CARGO_PKG_NAME");

fn format_msg(cmd: &Command) -> String {
    let command_msg: String = format!(
        "{} - {}\nAlias: {}\nUsage: {}\n",
        cmd.name,
        cmd.description,
        cmd.shortname.join(", "),
        cmd.usage
    );
    command_msg
}

pub fn run(args: Vec<String>, manager: CommandManager) {
    let command: Option<&String> = args.get(0);
    match command {
        Some(cmd) => {
            let command: Option<&Command> = manager.get_command(cmd.to_owned());
            if let Some(cmd) = command {
                let command_msg: String = format_msg(cmd);
                println!("{}", command_msg)
            } else {
                let unknown_msg: String = format!(
                    "Unknown command: {}. Try using {} help.",
                    cmd,
                    NAME
                );
                println!("{}", unknown_msg)
            }
        }
        _ => {
            let commands_list: &Vec<Command> = manager.get_commands();
            let mut commands_msg: Vec<String> = Vec::new();
            for command in commands_list {
                let push_msg: String = format_msg(command);
                commands_msg.push(push_msg);
            }
            let commands_msg: String = commands_msg.join("\n");
            print!("{}\n", commands_msg)
        }
    }
}
