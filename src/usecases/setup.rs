// use std::{io::{self, Write}, vec};
use std::vec;
use crate::{
    features::{self},
    utils::{self}
};

///THE MINIMAL VERSION OF CREATING A REPO
//TODO: CREATE MAIN BRANCH, ADD GIT BRANCH, ADD GIT ORIGIN,
pub fn init(){
    let link: String = loop{
        let temp: String = utils::inputs::input("Repository link: ");
        match temp.is_empty() {
            true => { },
            false => { break temp; }
            
        };
    };

    //TODO: manage a direct connection to upload 
    //    direct example
    /*  curl --request GET \
             --url "https://api.github.com/repos/octocat/Spoon-Knife/issues" \
             --header "Accept: application/vnd.github+json" \
             --header "Authorization: Bearer YOUR-TOKEN"
     * */

    //Show branches and getting the actual (NONE BY DEFAULT)
    let branches: (Vec<String>, String) = features::branch::get_branches();
}

// NOTE: THIS SETUP ASSUMES YOU ALREADY HAVE A ORIGIN REPO
// NOTE: i can skip the first step (30 lines) this if i assume the user 
// wants always to upload the whole thing, everybody knows how to use a gitignore...
pub fn quick(){
    // 1st step!
    let selection: &str = loop {
        let selection: String = utils::inputs::input(
            "(1) add everything or (2) select files?: "
        );
        match selection.trim(){
            "1" => { break "1" },
            "2" => { break "2" },
            _ => { eprint!("Invalid option."); }
        }
    };

    match selection {
        // git, add . 
        "1" => {
            println!("Choosen add everyting...");
            let _ = utils::run_cmd::run("ls", None, Some(
                &vec!["test.md", "Cargo.toml"]
            ));
        },
        // git, add <files> 
        "2" => {
            println!("Choosen add files...");
            let files: Vec<String> = utils::inputs::arg_input("Files: ");
            let vec_str: Vec<&str> = files.iter().map(|s| s.as_str()).collect();
            let _ = utils::run_cmd::run("ls", None, Some(&vec_str));

        },
        _ => unreachable!()
    }

    // 2nd step!
    // git, commit, -m, <commit_msg>
    let commit_msg: String = utils::inputs::input("Commit message: ");
    let _ = utils::run_cmd::run("ls", None, Some(
        &vec!["-a", "./" ]
    ));


    // 3rd step!
    let default: (Vec<String>, String) = features::branch::get_branches();
    //git, push, origin, <default_branch>
    let _ = utils::run_cmd::run("ls", None, Some(
        &vec!["-a", "./" ]
    ));

    println!("Done! (:");
    
}

