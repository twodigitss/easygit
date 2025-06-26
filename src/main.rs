mod common;
mod utils;
mod usecases;
mod features;

fn main() {
    println!("Hello, world!");
    //TODO: make a test for sudo privileges on this project
    //if asks for superuser permissions or gui authentication

    // let comm: String = utils::inputs::input("Comand: ");
    // let param: Vec<String> = utils::inputs::arg_input("Parameters: ");
    // let _ = utils::run_cmd::run(comm.trim(), None, Some(&param));

    usecases::setup::quick();
    // features::branch::switch();
}
