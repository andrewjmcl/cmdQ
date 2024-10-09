use std::io::{self, Write, prelude::*, BufReader};
use std::fs::File;
use std::path::Path;
use std::process::Command;

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    let mut asking = true;
    println!("cmds: [Q]uit, [D]elete, [S]ubmit, [V]iew, [L]oad");
    let mut cmds: Vec<String> = Vec::new();

    while asking {
        print!("$: ");
        io::stdout().flush().unwrap();

        let mut shell_cmd = String::new();
        io::stdin()
            .read_line(&mut shell_cmd)
            .expect("Failed to read line");
        let shell_cmd = shell_cmd.trim(); 

        match shell_cmd {
            "Q" => {
                println!("QUIT");
                return;
            }
            "D" => {
                if let Some(removed_cmd) = cmds.pop() {
                    println!("Deleted command: {}", removed_cmd);
                } else {
                    println!("No commands to delete.");
                }
            }
            "V" => {
                if !cmds.is_empty() {
                    println!("COMMANDS IN QUEUE:");
                    for (index, cmd) in cmds.iter().enumerate() {
                        println!("    {}. {}", index + 1, cmd);
                    }
                } else {
                    println!("No commands to show.");
                }
            }
            "S" => {
                asking = false;
            }
            "L" => {
                println!("Make sure commands are seperated by newlines.");
                print!("File: ");
                io::stdout().flush().unwrap(); 

                let mut cmd_file = String::new();
                io::stdin()
                    .read_line(&mut cmd_file)
                    .expect("Could not load file.");
                let cmd_file = cmd_file.trim(); 
                let lines = lines_from_file(cmd_file);
                for line in lines {
                    cmds.push(line);
                }
            }
            _ => {
                cmds.push(shell_cmd.to_string());
            }
        };
    }

    for cmd in cmds {
        println!("Executing: {}", cmd);

        let mut parts = cmd.split_whitespace();
        if let Some(program) = parts.next() {
            let args: Vec<&str> = parts.collect();

            let status = Command::new(program)
                .args(&args)
                .status();

            match status {
                Ok(status) => {
                    if !status.success() {
                        eprintln!("Command exited with non-zero status");
                    }
                }
                Err(e) => {
                    eprintln!("Failed to execute command: {}", e);
                }
            }
        } else {
            eprintln!("No command to execute.");
        }
    }
}
