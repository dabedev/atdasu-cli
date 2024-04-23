const NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Clone)]
pub struct Command {
    pub name: String,
    pub alias: Vec<String>,
    pub description: String,
    pub usage: String,
    pub priority: u8,
}

impl Command {
    pub fn new(
        name: String,
        alias: Vec<String>,
        description: String,
        usage: String,
        priority: u8
    ) -> Command {
        Command { name, alias, description, usage, priority }
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
        let predicate = |cmd: &&Command| (cmd.name == cmd_arg || cmd.alias.contains(&cmd_arg));
        let unknown_msg: String = format!("Unknown command: {}. Try using {} help.", cmd_arg, NAME);
        let cmd: &Option<&Command> = &self.commands.iter().find(predicate);
        match Some(cmd) {
            Some(cmd) => cmd.to_owned(),
            None => {
                println!("{}", unknown_msg);
                None
            }
        }
    }

    fn get_commands(&self) -> &Vec<Command> {
        &self.commands
    }
}
