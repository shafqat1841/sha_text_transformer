use std::env;
use std::env::Args;
use std::process;

use text_transformer::{AppCommand, UserInput, get_command_to_use};

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

    let command_to_use: AppCommand = get_command_to_use(&user_input).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    println!("user_command: {}", user_input.command);

    println!("user_text: {}", user_input.text);

    match command_to_use {
        AppCommand::Uppercase(text) => {
            println!("uppercase: {}", text.to_uppercase());
        }
        AppCommand::Lowercase(text) => {
            println!("lowercase: {}", text.to_lowercase());
        }
        AppCommand::Reverse(text) => {
            let chars = text.chars();
            let reversed_chars = chars.rev();
            let reversed_text: String = reversed_chars.collect();
            println!("reverse: {}", reversed_text);
        }
    }
}
