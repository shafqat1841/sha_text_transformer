use std::env;
use std::process;

use text_transformer::{AppCommand, UserInput, get_command_to_use, validate_args};

fn main() {
    let args: Vec<String> = env::args().collect();

    let is_valid = validate_args(&args);

    if (!is_valid) {
        eprintln!("Wrong number of arguments passed");
        eprintln!("Usage: cargo run <arg1: Text> <arg2: Command>");
        process::exit(1);
    }

    let user_input: UserInput = UserInput::build(args);
    let command_to_use: Option<AppCommand> = get_command_to_use(&user_input);

    println!("user_command: {}", user_input.command);

    if command_to_use.is_none() {
        eprintln!("Not valid command {}", user_input.command);
        process::exit(1);
    }

    println!("user_text: {}", user_input.text);

    match command_to_use.unwrap() {
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
