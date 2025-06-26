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

    args.trim().split(' ')
        .map(|w| w.to_string()).collect()

}
