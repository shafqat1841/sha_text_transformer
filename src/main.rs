use std::env;
use std::env::Args;
use std::process;

use text_transformer::{AppCommand, UserInput};

fn main() {
    let args: Args = env::args();

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
