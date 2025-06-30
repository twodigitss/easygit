use std::{
    io::Read, 
    process::{Command, Stdio}
};

pub fn run(comm: &str, params: &Vec<&str>, silent: bool ) -> String {
    let mut command = Command::new(comm);
    let command = command.stdout(Stdio::piped()).args(params);

    let child = command.spawn().expect("Command execution failed.");
    let output: String = match child.stdout {
        Some(mut stdout) => {
            let mut safe: String = String::new();
            stdout.read_to_string(&mut safe)
                .expect("Failed to unwrap the command's output.");
            safe
        },
        None => {
            "Error getting the value".to_string()
        }
    };
    if !silent { println!("{:?}", output); }
    output
}
