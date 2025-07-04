mod utils;
mod usecases;
mod features;

use std::env;

fn main() {
    // println!("Hello, world!"); //original hello world line cargo creates...

    //TODO: i gotta find a better way to handle parameters...
    let args: Vec<String> = env::args().collect();

    if args.contains(&String::from("-i")) {
        //Local setup
        usecases::setup::init(false);
    } 
    if args.contains(&String::from("-I")) {
        //With github initialization
        usecases::setup::init(true);
    } 
    else if args.contains(&String::from("-rc")) {
        //Quick upload
        usecases::undo_comm::recommit();
    }
    
    //TODO: add a second parameter to specify destionation branch
    //should accept a Option<> or figure it out a default value and?
    //its all about parameters
    
    else if args.contains(&String::from("-q")) {
        //Quick upload
        usecases::setup::quick();
    }
    else if args.contains(&String::from("-b")) {
        //Branch switching
        features::branch::switch();
    }
    else {
        let text = "
EASYGIT: GIT WORKFLOW MADE LESS REPETITIVE

| commands      | description                              |
|---------------|------------------------------------------|
| easygit -i    | initialize a project locally with git    | 
| easygit -I    | initialize a project on github with git  | 
| easygit -q    | quick upload, just type a message        |
| easygit -b    | branch switching, creation and deletion  |
| easygit -rc   | soft reset the prev commit and re commit |
";

        println!("{text}");
    }

}
