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

fn get_branches() -> Vec<String>{
    let mut branches: Vec<String> = Vec::new();
    
    let output = utils::run_cmd::run(
        "git", Some("branch"), None
    );

    for mut line in output.lines() {
        line = line.trim();
        if line.starts_with("*"){ line = &line[2..line.len()]; }
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

    //map through the branches
    let mut for_index = 1;
    for branch in branches.iter() {
        println!("({for_index}) : {branch}");
        for_index += 1;
    }
    let selection: String = utils::inputs::input("Branch? (number) ");
    let index: usize = selection
        .parse::<usize>()
        .unwrap_or(0);

    let choice = &branches[index - 1];
    // println!("YOUR CHOICE: {choice:?}");
    let _ = utils::run_cmd::run("git", None, Some(
        &vec![ "checkout".to_string(), choice.to_owned() ]
    ));

}
