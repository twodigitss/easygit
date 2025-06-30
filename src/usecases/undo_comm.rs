use crate::{
    utils::{self}
};

pub fn recommit(){
    utils::run_cmd::run("git", &vec!["reset", "--soft", "HEAD^"], true);
    utils::run_cmd::run("git", &vec!["add", "--all"], true);
    let message: String = utils::inputs::input("Commit message: ");
    utils::run_cmd::run("git", &vec!["commit", "-am", &message], true);
    println!("Done!")
}
