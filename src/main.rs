mod builtins;

use builtins::echo;
use builtins::pwd;
use builtins::type_command;
#[allow(unused_imports)]
use std::fs;
use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        // Wait for user input
        stdin.read_line(&mut input).unwrap();
        if input.trim() == "exit 0" {
            break;
        }
        //echo command
        else if input[0..=4].trim() == "echo" {
            echo(input[4..].trim());

        //type command
        } else if input[..=4].trim() == "type" {
            let command = input[4..].trim();
            type_command(command);
        }
        //pwd command
        else if input.trim() == "pwd" {
            pwd();
        }
        // Otherwise, search for the command in the PATH and execute it if it's found
        else {
            let args: Vec<&str> = input.split(' ').collect();
            let command = args[0].trim();
            let result = type_command(command);
            if !["not found", "path wrong", "builtin"].contains(&result.as_str()) {
                // If the command is found, execute it
                let output = std::process::Command::new(&result)
                    .args(&args[1..])
                    .output();

                match output {
                    Ok(output) => {
                        if !output.stdout.is_empty() {
                            print!("{}", String::from_utf8_lossy(&output.stdout));
                        }
                        if !output.stderr.is_empty() {
                            eprint!("{}", String::from_utf8_lossy(&output.stderr));
                        }
                    }
                    Err(e) => {
                        if e.kind() == std::io::ErrorKind::InvalidData {
                            eprintln!("{}: cannot execute binary file", command);
                        }
                    }
                }
            }
        }
    }
}
