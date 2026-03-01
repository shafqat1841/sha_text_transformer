use std::env;

fn main() {
    let app_commands = ["uppercase", "lowercase", "reverse"];

    let args: Vec<String> = env::args().collect();

    let args_length: usize = args.len();
    println!("{} arguments passed", args_length.to_string());

    if args_length != 3 {
        println!("Wrong number of arguments passed");
        println!("Usage: cargo run <arg1: Text> <arg2: Command>");
    }

    let mut command_to_use: Option<String> = None;

    let user_text: &String = &args[1];
    let user_command: &String = &args[2].to_lowercase();
    println!("user_command: {}", user_command);

    for app_command in app_commands {
        if app_command == user_command {
            command_to_use = Some(app_command.to_string());
            break;
        }
    }

    if command_to_use.is_none() {
        println!("Not valid command {}", user_command);
        return;
    }

    println!("user_text: {}", user_text);

    if command_to_use.as_ref().unwrap() == app_commands[0] {
        println!("uppercase: {}", user_text.to_uppercase());
    } else if command_to_use.as_ref().unwrap() == app_commands[1] {
        print!("lowercase: {}", user_text.to_lowercase());
    } else if command_to_use.as_ref().unwrap() == app_commands[2] {
        let chars = user_text.chars();
        let reversed_chars = chars.rev();
        let reversed_text: String = reversed_chars.collect();
        print!("reverse: {}", reversed_text);
    }
}
