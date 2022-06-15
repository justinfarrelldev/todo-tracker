pub struct Argument {
    name: String,
    pub description: String,
}

pub struct Command {
    pub name: String,
    pub description: String,
    pub args: Vec<Argument>,
}

pub fn get_commands() -> [Command; 1] {
    let commands = [Command {
        name: String::from("help"),
        description: String::from("Shows all of the available commands and their arguments."),
        args: vec![],
    }];

    return commands;
}
