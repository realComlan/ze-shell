use std::env;
use std::fs;
use std::io::{self, Error, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn is_executable(path: &Path) -> bool {
    match fs::metadata(path) {
        Ok(metadata) => {
            let mode = metadata.permissions().mode();
            metadata.is_file() && (mode & 0o111 != 0)
        }
        Err(_) => false,
    }
}

fn execute_command(path: &Path, args: &[&str]) -> Result<Output, Error> {
    let output = Command::new(path).args(args).output();
    output
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
                        match env::var("PATH") {
                            Ok(path_str) => {
                                let mut command_found = false;
                                let path_dirs = path_str.split(":");
                                for dir in path_dirs {
                                    let candidate = PathBuf::from(dir).join(command);
                                    if is_executable(&candidate) {
                                        command_found = true;
                                        println!("{} is {}", command, candidate.display());
                                        break;
                                    }
                                }
                                if !command_found {
                                    println!("{}: not found", command);
                                }
                            }
                            Err(error) => eprintln!("PATH not set: {}", error),
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
