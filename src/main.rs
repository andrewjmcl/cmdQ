use std::io::{self, Write};
use std::process::Command;

fn main() {
    let mut asking = true;
    println!("cmds: [Q]uit, [D]elete, [S]ubmit, [V]iew");
    let mut cmds: Vec<String> = Vec::new();

    while asking {
        print!("$: ");
        io::stdout().flush().unwrap(); // Ensure the prompt is displayed immediately

        let mut shell_cmd = String::new();
        io::stdin()
            .read_line(&mut shell_cmd)
            .expect("Failed to read line");
        let shell_cmd = shell_cmd.trim(); // Remove any trailing newline characters

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
            _ => {
                cmds.push(shell_cmd.to_string());
            }
        }
    }

    for cmd in cmds {
        println!("Executing: {}", cmd);

        // Split the command into the program and its arguments
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
