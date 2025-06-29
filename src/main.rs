mod utils;
mod usecases;
mod features;

use std::env;

fn main() {
    // println!("Hello, world!"); //original hello world line cargo creates... respect
    let args: Vec<String> = env::args().collect();

    if args.contains(&String::from("-i")) {
        usecases::setup::init();
    } 
    else if args.contains(&String::from("-q")) {
        usecases::setup::quick();
    }
    else if args.contains(&String::from("-b")) {
        features::branch::switch();
    }
    else {
        println!("TODO: finish");
    }

}
