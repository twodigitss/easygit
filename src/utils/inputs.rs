use crate::common;
use common::variables::SIZE;

use std::{
    io::{self, Write}, vec
};

/// USED FOR COMMON STRING TYPE INPUTS
/// RETURNS -> String
pub fn input(label: &str) -> String{
    print!("> {}", label);
        let _ = io::stdout().flush();

    let mut save = String::new();
    let _ = io::stdin()
        .read_line(&mut save)
        .expect("Error reading line.");

    save.trim().to_string()
}

/// USED TO RETURN MULTIPLE ARGUMENTS.
/// ARGUMENTS ARE SEPARATED BY 'SPACE'.
/// EXAMPLE:
///   >> git global config name twodigitss
pub fn arg_input(label: &str) -> Vec<String>{
    print!("> {}", label);
        let _ = io::stdout().flush();

    let mut args = String::new();
        let _ = io::stdin()
            .read_line(&mut args)
            .expect("Error reading line.");

    //FIX: handle this case better in the future
    // if args.trim().is_empty() {
    //    println!("parameters: {}", args);
    //    let mut args: Vec<String> = Vec::new();
    //    args.push("".to_string());
    // }

    let splitted: Vec<&str> = args.trim().split(' ').collect();

    //shadowed original var
    let mut args: Vec<String> = Vec::new();
    for s in splitted {
        args.push(s.to_string());
    }

    // println!("{:?}",args);
    args

    // let mut safe_obj: [Option<String>; SIZE] = [
    //     const { None }; SIZE
    // ];
    //
    // for (index, value) in args.iter().enumerate(){
    //     safe_obj[index] = Some(value.to_string());
    // }
    //
    // safe_obj
    
}
