use std::process;

mod helper_file;
mod user_input_file;

pub use crate::helper_file::helper_mod::AppCommand;
pub use crate::user_input_file::user_input_mod::UserInput;

pub fn run(args: std::env::Args) {
    if args.len() != 3 {
        eprintln!("Wrong number of arguments passed");
        eprintln!("Usage: cargo run <arg1: Text> <arg2: Command>");
        process::exit(1);
    }

    let user_input: UserInput = UserInput::build(args).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    let command_to_use: AppCommand =
        AppCommand::get_command_to_use(user_input).unwrap_or_else(|err| {
            eprintln!("Error: {}", err);
            process::exit(1);
        });

    println!("{}", command_to_use);
}
