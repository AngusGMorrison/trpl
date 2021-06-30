use std::io;

pub fn list_commands() {
    println!("Enter a command:");
    println!("* add <name> to <department>");
    println!("* list");
    println!("* quit");
    println!();
}

pub fn get_command() -> String {
    let mut command = String::new();
    io::stdin()
        .read_line(&mut command)
        .expect("Failed to read from stdin");
    println!();

    command.trim().to_string()
}
