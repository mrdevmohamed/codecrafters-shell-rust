#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::{self};

fn main() {
    const SHELL_BUILTIN: [&str; 3] = ["echo", "exit", "type"];
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        // Wait for user input
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        let mut parts = command.trim().split_whitespace();
        match parts.next() {
            Some("exit") => process::exit(0),
            Some("echo") => {
                print!("{}\n", parts.collect::<Vec<&str>>().join(" "))
            }
            Some("type") => match parts.next() {
                Some(target) => {
                    let result = SHELL_BUILTIN
                        .iter()
                        .copied()
                        .find(|&command| command == target);

                    match result {
                        Some(command) => println!("{command} is a shell builtin"),
                        None => println!("{target}: not found"),
                    }
                }
                None => println!("type: missing command name"),
            },
            Some(cmd) => println!("{cmd}: command not found"),
            None => {}
        }
    }
}
