use std::env;
use std::process;

struct UserInput {
    text: String,
    command: String,
}

impl UserInput {
    fn build(args: Vec<String>) -> UserInput {
        UserInput {
            text: args[1].clone(),
            command: args[2].clone().to_lowercase(),
        }
    }
}

enum AppCommand {
    Uppercase(String),
    Lowercase(String),
    Reverse(String),
}

fn get_command_to_use(user_input: &UserInput) -> Option<AppCommand> {
    let mut command_to_use: Option<AppCommand> = None;
    match user_input.command.as_str() {
        "uppercase" => command_to_use = Some(AppCommand::Uppercase(user_input.text.clone())),
        "lowercase" => command_to_use = Some(AppCommand::Lowercase(user_input.text.clone())),
        "reverse" => command_to_use = Some(AppCommand::Reverse(user_input.text.clone())),
        _ => {}
    }
    command_to_use
}

fn validate_args(args: &Vec<String>) {
    let args_length: usize = args.len();
    if args_length != 3 {
        eprintln!("Wrong number of arguments passed");
        eprintln!("Usage: cargo run <arg1: Text> <arg2: Command>");
        process::exit(1);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    validate_args(&args);

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
        },
         AppCommand::Lowercase(text) => {
            println!("lowercase: {}", text.to_lowercase());
        },
         AppCommand::Reverse(text) => {
            let chars = text.chars();
            let reversed_chars = chars.rev();
            let reversed_text: String = reversed_chars.collect();
            println!("reverse: {}", reversed_text);
        },
    }

}
