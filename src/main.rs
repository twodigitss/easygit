mod utils;
mod usecases;
mod features;

use std::env;

/*  IDEAS:
 *  recommit -- rewind commit and recommit changes
 *      git reset --soft HEAD^, git add --all, git commit -am message, 	
 *
 * */
fn main() {
    // println!("Hello, world!"); //original hello world line cargo creates... respect
    let args: Vec<String> = env::args().collect();

    if args.contains(&String::from("-i")) {
        usecases::setup::init();
    } 
    //TODO: add a second parameter to specify destionation branch
    //should accept a Option<> or figure it out a default value and?
    //its all about parameters
    else if args.contains(&String::from("-q")) {
        usecases::setup::quick();
    }
    else if args.contains(&String::from("-b")) {
        features::branch::switch();
    }
    else if args.contains(&String::from("-bd")) {
        features::branch::delete_branch();
    }
    else {
        println!("easygit");
    }

}
