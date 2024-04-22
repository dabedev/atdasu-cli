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
}

impl Manager for CommandManager {
    fn add_command(&mut self, cmd: Command) {
        self.commands.push(cmd);
    }
}
