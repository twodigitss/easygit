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
    else if args.contains(&String::from("-bd")) {
        //Branch deletion
        features::branch::delete_branch();
    }
    else {
        println!("easygit");
    }

}
