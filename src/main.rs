use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{self, Command};

const SHELL_BUILTIN: [&str; 3] = ["echo", "exit", "type"];

fn find_executable(command: &str) -> Option<PathBuf> {
    let path = env::var_os("PATH")?;

    for directory in env::split_paths(&path) {
        let candidate = directory.join(command);

        if is_executable(&candidate) {
            return Some(candidate);
        }
    }

    None
}

#[cfg(unix)]
fn is_executable(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;

    fs::metadata(path)
        .map(|metadata| metadata.is_file() && metadata.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

#[cfg(windows)]
fn is_executable(path: &Path) -> bool {
    path.is_file()
        && path.extension().is_some_and(|extension| {
            ["exe", "cmd", "bat", "com"]
                .iter()
                .any(|ext| extension.eq_ignore_ascii_case(ext))
        })
}

fn command_type(command: &str) {
    if SHELL_BUILTIN.contains(&command) {
        println!("{command} is a shell builtin");
    } else {
        match find_executable(command) {
            Some(path) => println!("{}", path.display()),
            None => println!("{command}: not found"),
        }
    }
}

fn run_external(command: &str, args: &[&str]) {
    let Some(path) = find_executable(command) else {
        println!("{command}: command not found");
        return;
    };

    match Command::new(path).args(args).status() {
        Ok(status) => {
            if let Some(code) = status.code() {
                // Preserve the external program's exit status.
                let _ = code;
            }
        }
        Err(error) => {
            eprintln!("{command}: {error}");
        }
    }
}

fn main() {
    loop {
        print!("$ ");

        if io::stdout().flush().is_err() {
            break;
        }

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => {}
            Err(error) => {
                eprintln!("Input error: {error}");
                break;
            }
        }

        let mut parts = input.split_whitespace();

        match parts.next() {
            Some("exit") => process::exit(0),

            Some("echo") => {
                println!("{}", parts.collect::<Vec<_>>().join(" "));
            }

            Some("type") => match parts.next() {
                Some(target) => command_type(target),
                None => println!("type: missing command name"),
            },

            Some(command) => {
                let args: Vec<&str> = parts.collect();
                run_external(command, &args);
            }

            None => {}
        }
    }
}
