use crate::utils::{self};
// use std::{
//     io::Read, 
//     process::{Command, Stdio}
// };

/*  To create a new branch in Git using the command line, you can use the command 
 *  git branch <branch-name> to create the branch, or git checkout -b <branch-name> 
 *  to create and switch to the new branch at the same time. For example, 
 *  git checkout -b my-feature creates a new branch called "my-feature" and switches to it immediately.
 * */

//TUPLE: (branches, actual)
//FIX: will this give an error if empty? handle that
pub fn get_branches() -> (Vec<String>, String) {
    let mut branches: Vec<String> = Vec::new();
    let mut actual_branch: &str = "main";
    
    let output = utils::run_cmd::run(
        "git", Some("branch"), None
    );

    for mut line in output.lines() {
        line = line.trim();
        if line.starts_with("*"){ 
            line = &line[2..line.len()]; 
            actual_branch = line;
        }
        branches.push(line.to_string());
    };

    println!("{branches:?}");
    (branches, actual_branch.to_string())

}

pub fn new_branch() -> Result<(),String> {
    let name: String = utils::inputs::input("Branch name: ");
    let _ = utils::run_cmd::run("git", None, Some(
        &vec![ "branch", &*name ]
    ));
    println!("Done!");
    Ok(())
}

pub fn switch(){
    let branches: (Vec<String>, String) = get_branches();

    let mut for_index = 1;
    for branch in branches.0.iter() {
        println!("({for_index}) : {branch}");
        for_index += 1;
    }
    let selection: String = utils::inputs::input("Branch? (number) ");
    let index: usize = selection
        .parse::<usize>()
        .unwrap_or(0);

    let choice = &branches.0[index - 1];
    let _ = utils::run_cmd::run("git", None, Some(
        &vec![ "checkout", choice ]
    ));
    println!("Switched to branch {choice:?}!");

}
