use std::{
    io::Read, 
    process::{Command, Stdio}
};

pub fn run(comm: &str, param: Option<&str>, params: Option<&Vec<String>> ){
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

    let child = command.stdout(Stdio::piped())
        .spawn()
        .expect("Command execution failed.");

    let output: String = match child.stdout {
        Some(mut stdout) => {
            let mut safe: String = String::new();
            stdout.read_to_string(&mut safe)
                .expect("Failed to unwrap the command's output.");
            safe
        },
        None => {
            eprintln!("Error reading outcome of the command ");
            return; //TODO: manage properly
        }
    };
    println!("{:?}", output);
}
