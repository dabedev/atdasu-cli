//use dasu::commands;
use dasu::handlers;
use dasu::handlers::CommandManager;

fn main() {
    let mut manager = CommandManager::new();
    handlers::load_commands(&mut manager);
}
