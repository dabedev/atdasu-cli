const NAME: &str = env!("CARGO_PKG_NAME");
pub struct Command {
    pub name: String,
    pub shortname: String,
    pub description: String,
}

impl Command {
    pub fn new(name: String, shortname: String, description: String) -> Command {
        Command { name, shortname, description }
    }
}

pub struct CommandManager {
    commands: Vec<Command>,
}

impl CommandManager {
    pub fn new() -> CommandManager {
        CommandManager { commands: Vec::new() }
    }
}

pub trait Manager {
    fn add_command(&mut self, cmd: Command);
    fn get_command(&self, arg: String) -> Option<&Command>;
    fn get_commands(&self) -> &Vec<Command>;
}

impl Manager for CommandManager {
    fn add_command(&mut self, cmd: Command) {
        self.commands.push(cmd);
    }

    fn get_command(&self, arg: String) -> Option<&Command> {
        let cmd_arg: String = arg.clone();
        let predicate = |cmd: &&Command| (cmd.name == cmd_arg || cmd.shortname == cmd_arg);
        let unknown_msg: String = format!("Unknown command: {}. Try using {} help.", cmd_arg, NAME);
        let cmd: &&Command = &self.commands.iter().find(predicate).expect(&unknown_msg);
        Some(cmd)
    }

    fn get_commands(&self) -> &Vec<Command> {
        &self.commands
    }
}
