use std::{
    io::{self, Write}
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

    let splitted: Vec<&str> = args.trim().split(' ').collect();

    //shadowed original var
    let mut args: Vec<String> = Vec::new();
    for s in splitted {
        args.push(s.to_string());
    }
    args
}

pub fn vec_str_string(vector: Vec<&str>) -> Vec<String>{
    let mut args: Vec<String> = Vec::new();
    for s in vector { args.push(s.to_string()); }
    args
}
