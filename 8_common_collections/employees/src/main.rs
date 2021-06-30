mod cli;
mod employees;

fn main() {
    let mut db = employees::EmployeeDB::new();

    loop {
        cli::list_commands();

        let cmd = cli::get_command();
        if cmd == "list" {
            db.list();
        } else if cmd == "quit" {
            std::process::exit(0);
        } else if cmd.to_lowercase().starts_with("add") {
            let (name, dept) = match employees::parse_employee_data(&cmd) {
                Ok(employee_data) => employee_data,
                Err(_) => {
                    println!("usage: add <name> to <dept>\n");
                    continue;
                }
            };
            db.insert(&name, &dept);
        } else {
            println!("Command not recognized\n")
        }
    }
}
