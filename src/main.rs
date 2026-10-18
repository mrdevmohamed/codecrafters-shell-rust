#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
    }
}
