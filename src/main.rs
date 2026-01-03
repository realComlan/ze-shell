use std::env;
use std::io::{self, Error, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn is_executable(path: &Path) -> bool {
    path.is_file()
        && path
            .metadata()
            .map(|m| m.permissions().mode() & 0o111 != 0)
            .unwrap_or(false)
}

fn execute_command(path: &Path, args: &[&str]) -> Result<Output, Error> {
    Command::new(path).args(args).output()
}

fn find_executable(command: &str) -> io::Result<(bool, PathBuf)> {
    let path_str = env::var("PATH").map_err(|e| io::Error::new(io::ErrorKind::NotFound, e))?;

    for dir in path_str.split(':') {
        let candidate = PathBuf::from(dir).join(command);
        if is_executable(&candidate) {
            return Ok((true, candidate));
        }
    }

    Ok((false, PathBuf::new()))
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // receive the user input
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).unwrap();

        match user_input
            .trim()
            .split_whitespace()
            .collect::<Vec<&str>>()
            .as_slice()
        {
            ["echo", rest @ ..] => {
                // print the second parameter back to the user
                println!("{}", rest.join(" "));
                io::stdout().flush().unwrap();
            }
            ["type", rest @ ..] => {
                let builtin_commands = vec![
                    ".",
                    ":",
                    "[",
                    "alias",
                    "bg",
                    "bind",
                    "break",
                    "builtin",
                    "caller",
                    "cd",
                    "command",
                    "compgen",
                    "complete",
                    "compopt",
                    "continue",
                    "declare",
                    "dirs",
                    "disown",
                    "echo",
                    "enable",
                    "eval",
                    "exec",
                    "exit",
                    "export",
                    "false",
                    "fc",
                    "fg",
                    "getopts",
                    "hash",
                    "help",
                    "history",
                    "jobs",
                    "kill",
                    "let",
                    "local",
                    "logout",
                    "mapfile",
                    "popd",
                    "printf",
                    "pushd",
                    "pwd",
                    "read",
                    "readarray",
                    "readonly",
                    "return",
                    "set",
                    "shift",
                    "shopt",
                    "source",
                    "suspend",
                    "test",
                    "times",
                    "trap",
                    "true",
                    "type",
                    "typeset",
                    "ulimit",
                    "umask",
                    "unalias",
                    "unset",
                    "wait",
                ];

                // loop over all commands following 'type'
                for command in rest.iter() {
                    if builtin_commands.contains(command) {
                        println!("{} is a shell builtin", command);
                    } else {
                        // check for executable rights, print command is at <path>
                        let (found, path) = find_executable(&command).unwrap();
                        if found {
                            println!("{} is {}", command, path.display());
                        } else {
                            eprintln!("{}: not found", command);
                        }
                    }
                }
            }
            ["exit", ..] => {
                return;
            }
            [command, args @ ..] => {
                let output = execute_command(Path::new(&command), &args);
                match output {
                    Ok(output) => println!("{}", String::from_utf8_lossy(&output.stdout)),
                    Err(err) => eprintln!("{}", err),
                }
            }
            _ => print!(""),
        }
    }
}
