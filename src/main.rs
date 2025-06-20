mod common;
mod utils;
use utils::{
    run_cmd,
    inputs::{input, arg_input},
};

fn main() {
    println!("Hello, world!");
    let comm: String = input("Comand: ");
    let param: Vec<String> = arg_input("Parameters: ");

    run_cmd::run(comm.trim(), param);
}
