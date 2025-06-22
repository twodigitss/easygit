mod common;
mod utils;
mod usecases;
mod features;

fn main() {
    println!("Hello, world!");

    // usecases::setup::github_upl();
    features::branch::switch();
}
