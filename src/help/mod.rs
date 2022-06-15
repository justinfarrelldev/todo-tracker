use crate::commands::get_commands;

pub fn help() {
    println!("================ COMMANDS LIST ================");
    for command in get_commands() {
        println!("{}: {}", command.name, command.description);
    }
}
