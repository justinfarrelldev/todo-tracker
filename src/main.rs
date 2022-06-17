/*
    should be able to accept arguments
    init: initializes todo list in the repository
    add: adds a todo list item
    remove: removes a todo list item
*/

use std::env;

mod commands;
mod help;
mod new;
use crate::help::help;
use crate::new::new;
fn get_args() -> Vec<String> {
    return env::args().collect();
}

// Returns true when valid, false otherwise
fn handle_args(args: Vec<String>) -> bool {
    if args.len() == 1 {
        println!("No args passed.");
        return false;
    }
    match args[1].as_ref() {
        "help" => {
            help();
            return true;
        }
        "new" => {
            new(args);
            return true;
        }
        &_ => {
            println!("Invalid argument passed.");
            return false;
        }
    }
}

fn main() {
    let args: Vec<String> = get_args();
    handle_args(args);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_args() {
        assert_eq!(
            handle_args(vec!["path".to_string(), "help".to_string()]),
            true
        );
    }
    #[test]
    fn test_handle_args_none() {
        assert_eq!(handle_args(vec!["path".to_string()]), false);
    }
    #[test]
    fn test_handle_args_invalid() {
        assert_eq!(
            handle_args(vec!["path".to_string(), "testing".to_string()]),
            false
        );
    }
}
