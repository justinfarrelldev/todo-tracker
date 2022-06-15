use crate::commands::get_commands;

pub fn help() {
    println!("================ COMMANDS LIST ================");
    for command in get_commands() {
        println!("{}: {}", command.name, command.description);
        if command.args.len() > 0 {
            println!("\tArguments: ");
        }
        for arg in command.args {
            println!("\t\t{}: {}", arg.name, arg.description);
        }
    }
}
