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
pub fn get_branches(print_result: bool) -> (Vec<String>, String) {
    let mut branches: Vec<String> = Vec::new();
    let mut actual_branch: &str = "main";
    
    let output = utils::run_cmd::run("git", &vec![ "branch" ], print_result);

    for mut line in output.lines() {
        line = line.trim();
        if line.starts_with("*"){ 
            line = &line[2..line.len()]; 
            actual_branch = line;
        }
        branches.push(line.to_string());
    };

    // println!("{branches:?}");
    (branches, actual_branch.to_string())

}

fn new_branch() -> Result<(),String> {
    let name: String = utils::inputs::input("Branch name: ");
    utils::run_cmd::run("git", &vec![ "branch", &name ], true);
    utils::run_cmd::run("git", &vec![ "checkout", "-b", &name ], true);
    println!("Branch {name} created and switched!");
    Ok(())
}

pub fn switch(){
    let mut branches: (Vec<String>, String) = get_branches(true);
    branches.0.insert(branches.0.len(), "New branch".to_string());
    
    let mut for_index = 1;
    for branch in branches.0.iter() {
        println!("({for_index}) : {branch}");
        for_index += 1;
    }

    let selection: String = utils::inputs::input("Branch? (number) ");
    let index: usize = selection
        .parse::<usize>()
        .unwrap_or(0);

    //NOTE: i can avoid recursion by switching immediately
    if index == branches.0.len(){
        let _ = new_branch();
        // switch(); //forgive me father for i have sinned
    } 

    let _ = utils::run_cmd::run("git", 
        &vec![ "checkout", &branches.0[index - 1] ], true
    );

}

//TODO: implemment this
pub fn delete_branch(){
    let branches: (Vec<String>, String) = get_branches(true);
    
    let mut for_index = 1;
    for branch in branches.0.iter() {
        println!("({for_index}) : {branch}");
        for_index += 1;
    }

    let selection: String = utils::inputs::input("Delete: (number) ");
    let index: usize = selection
        .parse::<usize>()
        .unwrap_or(0);

    let _ = utils::run_cmd::run("git", 
        &vec![ "branch", "-D", &branches.0[index - 1] ], true
    );

}
