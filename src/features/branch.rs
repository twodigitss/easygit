use crate::utils::{self};
use std::{
    io::Read, 
    process::{Command, Stdio}
};

/*  To create a new branch in Git using the command line, you can use the command 
 *  git branch <branch-name> to create the branch, or git checkout -b <branch-name> 
 *  to create and switch to the new branch at the same time. For example, 
 *  git checkout -b my-feature creates a new branch called "my-feature" and switches to it immediately.
 * */

fn get_branches() -> Vec<String>{
    let mut branches: Vec<String> = Vec::new();
    let mut command: Command = Command::new("git");
    let child = command.stdout(Stdio::piped())
        .arg("branch")
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
            eprintln!("Error reading outcome of the command. returning empty");
            //TODO: manage properly
            "".to_string()
        }
    };

    for line in output.lines() {
        branches.push(line.to_string());
    };

    println!("{branches:?}");
    branches


}

#[warn(dead_code)]
fn new() -> Result<(),String> {
    let name: String = utils::inputs::input("Branch name: ");
    let _ = utils::run_cmd::run("git", None, Some(
        &vec![ "branch".to_string(), name ]
    ));
    println!("Done!");
    Ok(())
}

pub fn switch(){
    //TODO: list all branches first.
    let branches: Vec<String> = get_branches();
    let selection: String = loop {
        let selection: String = utils::inputs::input(
            "(1) add everything or (2) select files?: "
        );
        match selection.trim(){
            "1" => { break "1".to_string(); },
            "2" => { break "2".to_string(); },
            _ => { eprint!("Invalid option."); }
        }
    };
    let selection = selection.parse::<i8>();

    println!("{:?}",branches.get(0));

    // utils::run_cmd::run("git", Some("branch"), None);


}
