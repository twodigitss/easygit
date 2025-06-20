use std::{
    io::Read, 
    process::{Command, Stdio}
};

pub fn run(comm: &str, params: Vec<String>){
    if params.is_empty() {
        println!("No parameters provided");
    }

    //TODO: handle .args() when params == empty
    let command = Command::new(comm)
        .args(params)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Command execution failed.");

    // let mut safe: String = String::new();
    // command.stdout.unwrap()
    //     .read_to_string(&mut safe)
    //     .expect("Failed to unwrap the output of the command");

    let output: String = match command.stdout {
        Some(mut stdout) => {
            let mut safe: String = String::new();
            stdout.read_to_string(&mut safe)
                .expect("Failed to unwrap the command's output.");
            safe
        },
        None => {
            eprintln!("Error reading stdout ");
            return; //TODO: manage properly
        }
        
    };
    println!("{:?}", output);
}
