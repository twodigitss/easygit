// use std::{io::{self, Write}, vec};
use std::vec;
use crate::utils::{self};

/*  SETPS TO CREATE A REPO LOCALLY without too much effort
 *  git init
 *  git add <either N elements or dot>
 *  git commit -m "description"
 *      I have never done this, its weird to me but
 *      apparently you can manage everything locally
 *      like this...
 *
 * */

///THE MINIMAL VERSION OF CREATING A REPO
pub fn init(){
    // 1st step!
    let mut params: Vec<String> = Vec::new();
    let _ = utils::run_cmd::run("ls", Some("-ls"), None);

    // 2nd step!
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

    match selection.trim(){
        "1" => {
            let _ = utils::run_cmd::run("ls", None, Some(
                &vec!["test.md".to_string(), "Cargo.toml".to_string()]
            ));
            params.clear();
        },
        "2" => {
            let files: Vec<String> = utils::inputs::arg_input("Files: ");
            let _ = utils::run_cmd::run("ls", None, Some(&files));

        },
        _ => unreachable!()
    }

    // 3rd step!
    let commit_msg: String = utils::inputs::input("Commit message: ");
    let _ = utils::run_cmd::run("ls", None, Some(
        &vec!["-a".to_string(), "./".to_string() ]
    ));

    println!("Initializing done! this is a local repository to track changes.");
    println!("To upload, choose the cloud setup");
    
}

pub fn github_upl(){
    //do a minimal setup + steps to cloud upload
    // init();
    println!("Initializing cloud setup...");

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
}
