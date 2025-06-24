mod common;
mod utils;
mod usecases;
mod features;

fn main() {
    println!("Hello, world!");
    // let comm: String = utils::inputs::input("Comand: ");
    // let param: Vec<String> = utils::inputs::arg_input("Parameters: ");
    // let _ = utils::run_cmd::run(comm.trim(), None, Some(&param));

    // usecases::setup::github_upl();
    features::branch::switch();
}
