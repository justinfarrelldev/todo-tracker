pub struct Argument {
    pub name: String,
    pub description: String,
}

pub struct Command {
    pub name: String,
    pub description: String,
    pub args: Vec<Argument>,
}

pub fn get_commands() -> [Command; 2] {
    let commands = [
        Command {
            name: String::from("help"),
            description: String::from("Shows all of the available commands and their arguments."),
            args: vec![],
        },
        Command {
            name: String::from("new"),
            description: String::from("Creates a new todo list."),
            args: vec![Argument {
                name: String::from("--name"),
                description: String::from(
                    "The name of the list to create (will generate a file with the list name).",
                ),
            }],
        },
    ];

    return commands;
}
