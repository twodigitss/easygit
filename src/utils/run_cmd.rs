use std::{
    io::Read, 
    process::{Command, Stdio}
};

pub fn run(
    comm: &str, 
    param: Option<&str>, 
    params: Option<&Vec<String>> 
) -> String {

    let mut command = Command::new(comm);
    let command = command.stdout(Stdio::piped());
    match params{
        Some(array) => {command.args(array);},
        None => {
            match param {
                Some(value) => {command.arg(value);},
                None => {
                    println!("No parameters provided");
                }
            }
        }
    }

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
    println!("{:?}", output);
    output
}
