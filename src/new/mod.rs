use std::io::stdin;

fn ask_name() -> String {
    println!("Enter the name for the checklist (will have .txt appended): ");
    let mut input = String::new();

    stdin().read_line(&mut input).expect("Invalid input");
    return input.trim().to_string();
}

fn get_file_name(args: Vec<String>) -> String {
    let file_name: String;
    if args.len() > 2 {
        file_name = args[2].to_string();
    } else {
        file_name = ask_name();
    }
    return String::from(file_name + ".txt");
}

pub fn new(args: Vec<String>) {
    let file_name: String = get_file_name(args);
    println!("got file name: {}", file_name)
}
