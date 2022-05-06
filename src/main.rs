use std::io;

mod core;

enum Command {
    Init,
    Add(core::Employee),
    List(String),
    ListAll,
    Quit,
    Help,
}

impl Command {
    fn parse(entry: &str) -> Command {
        let words: Vec<&str> = entry.split_whitespace().collect();
        match words.get(0) {
            // TODO fix panick
            Some(&"add") => {
                let employee = core::Employee {
                    name: words.get(1).expect("No name provided").to_string(),
                    department: words.get(3).expect("No department provided").to_string(),
                };
                Command::Add(employee)
            }
            // TODO fix panick
            Some(&"list") => {
                Command::List(words.get(1).expect("Department is missing").to_string())
            }
            Some(&"list-all") => Command::ListAll,
            Some(&"quit") => Command::Quit,
            None => Command::Init,
            _ => Command::Help,
        }
    }
}

struct EmployeeDBCLI {
    employee_db: core::EmployeeDB,
    command: Command,
}

impl EmployeeDBCLI {
    fn new() -> EmployeeDBCLI {
        EmployeeDBCLI {
            employee_db: core::EmployeeDB::new(),
            command: Command::Init,
        }
    }
    fn run(&mut self) -> bool {
        match &self.command {
            Command::Init => true,
            Command::Add(e) => {
                self.employee_db.add(&e);
                true
            }
            Command::List(s) => {
                self.employee_db.list_one(&s);
                true
            }
            Command::ListAll => {
                self.employee_db.list_all();
                true
            }
            Command::Quit => {
                println!("Bye.");
                false
            }
            Command::Help => {
                println!(
                    "Commands:\n{}\n{}\n{}\n{}\n{}",
                    "add <name> to <department>", "list <department>", "list-all", "help", "quit"
                );
                true
            }
        }
    }
}

fn main() {
    println!("Welcome to the company tool");
    let mut cli = EmployeeDBCLI::new();

    loop {
        let mut entry = String::new();
        io::stdin()
            .read_line(&mut entry)
            .expect("Failed to read entry.");
        cli.command = Command::parse(&entry);
        let go = cli.run();
        if !go {
            break;
        }
    }
}
