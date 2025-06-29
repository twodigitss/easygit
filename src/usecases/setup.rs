// use std::{io::{self, Write}, vec};
use std::vec;
use crate::{
    features::{self},
    utils::{self}
};

///THE MINIMAL VERSION OF CREATING A REPO
pub fn init(){

    utils::run_cmd::run("git", &vec!["init"]);
    utils::run_cmd::run("touch", &vec!["README.md"]);
    utils::run_cmd::run("git", &vec!["add", "."]);
    utils::run_cmd::run("git", &vec!["commit", "-m", "initial commit"]);

    // THIS ONLY DOES WORK AFTER FIRST COMMIT.
    // i had no other option than initializing the whole thing above
    utils::run_cmd::run("git", &vec!["branch", "-m", "main"]);

    let mut validated = false;
    let mut repository: String = String::new(); 
    while !validated {
        repository = loop{
            let temp: String = utils::inputs::input("Repository link: ");
            match temp.is_empty() {
                true => { },
                false => { break temp; }

            };
        };
        let it_is: bool = utils::valid_repo::is_valid_repo(&repository); 
        if it_is {validated = true}
    }

    utils::run_cmd::run("git", &vec!["remote", "add", "origin", &repository]);
    utils::run_cmd::run("git", &vec!["push", "-u", "origin", "main"]);

}

//did save this procedure in case i need it (i doubt it)
#[warn(dead_code)]
fn add_selection(){
    // 1st step!
    let selection: &str = loop {
        let selection: String = utils::inputs::input(
            "(1) add everything \n(2) select files?: "
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
            // println!("Choosen add everyting...");
            let _ = utils::run_cmd::run("ls", &vec!["test.md", "Cargo.toml"]);
        },
        // git, add <files> 
        "2" => {
            // println!("Choosen add files...");
            let files: Vec<String> = utils::inputs::arg_input("Files: ");
            let vec_str: Vec<&str> = files.iter().map(|s| s.as_str()).collect();
            let _ = utils::run_cmd::run("ls", &vec_str);

        },
        _ => unreachable!()
    }

}

// NOTE: THIS SETUP ASSUMES YOU ALREADY HAVE A ORIGIN REPO
// AND YOU WANT TO UPLOAD EVERYTHING EXCEPT FOR WHATEVER YOU
// HAVE ON YOUR GITIGNORE
pub fn quick(){
    // 1st step!
    let _ = utils::run_cmd::run("git", &vec!["add", "."]);

    // 2nd step!
    let commit_msg: String = utils::inputs::input("Commit message: ");
    let _ = utils::run_cmd::run("git", &vec!["commit", "-m", &commit_msg ]);

    // 3rd step!
    let default: (Vec<String>, String) = features::branch::get_branches();
    let _ = utils::run_cmd::run("git", &vec!["push", "origin", &default.1 ]);

    println!("Done!");
    
}

