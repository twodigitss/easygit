use std::{
    io::Read, 
    process::{Command, Stdio}
};

pub fn run(
    comm: &str, 
    param: Option<&str>, 
    params: Option<&Vec<String>> 
) -> Result<String, String> {

    let mut command = Command::new(comm);
    match params{
        Some(array) => {command.args(array);},
        None => {
            match param {
                Some(value) => {command.arg(value);},
                None => {println!("No parameters provided");}
            }
        }
    }

    let mut command = Command::new(comm);
    let child = command.stdout(Stdio::piped())
        .spawn()
        .expect("Command execution failed.");

    let output: Result<String, String> = match child.stdout {
        Some(mut stdout) => {
            let mut safe: String = String::new();
            stdout.read_to_string(&mut safe)
                .expect("Failed to unwrap the command's output.");
            Ok(safe)
        },
        //TODO: manage properly
        None => {
            Err("Error reading outcome of the command ".to_string())
            // eprintln!("Error reading outcome of the command ");
        }
    };
    println!("{:?}", output);
    output
}
