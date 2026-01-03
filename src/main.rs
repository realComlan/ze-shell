#[allow(unused_imports)]
use std::io::{self, Read, Write};
use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

fn is_executable(path: &Path) -> bool {
    match fs::metadata(path) {
        Ok(metadata) => {
            let mode = metadata.permissions().mode();
            metadata.is_file() && (mode & 0o111 != 0)
        },
        Err(_) => false,
    }
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // receive the user input
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).unwrap();

        match user_input
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
                                if ! command_found {
                                    println!("{}: not found", command);
                                }
                            },
                            Err(error) => eprintln!("PATH not set: {}", error), 
                        }
                    }
                }
            }
            ["exit", ..] => {
                return;
            }
            _ => {
                // print command not found anyway
                eprintln!("{}: command not found", user_input.trim());
                io::stdout().flush().unwrap();
            }
        }
    }
}
