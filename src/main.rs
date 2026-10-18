#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::{self};

fn main() {
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
            Some(cmd) => println!("{cmd}: command not found"),
            None => {}
        }
    }
}
