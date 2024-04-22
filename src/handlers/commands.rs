pub struct Command {
    pub name: String,
    pub shortname: String,
}

impl Command {
    pub fn new(name: String, shortname: String) -> Command {
        Command { name, shortname }
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
}

impl Manager for CommandManager {
    fn add_command(&mut self, cmd: Command) {
        self.commands.push(cmd);
    }

    fn get_command(&self, arg: String) -> Option<&Command> {
        let cmd_arg: String = arg.clone();
        let predicate = |cmd: &&Command| (cmd.name == cmd_arg || cmd.shortname == cmd_arg);
        let unknown_msg: String = format!("Unknown command: {}", cmd_arg);
        let cmd: &&Command = &self.commands.iter().find(predicate).expect(&unknown_msg);
        Some(cmd)
    }
}
