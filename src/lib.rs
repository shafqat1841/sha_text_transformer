pub struct UserInput {
    pub text: String,
    pub command: String,
}

impl UserInput {
    pub fn build(args: Vec<String>) -> UserInput {
        UserInput {
            text: args[1].clone(),
            command: args[2].clone().to_lowercase(),
        }
    }
}

pub enum AppCommand {
    Uppercase(String),
    Lowercase(String),
    Reverse(String),
}

pub fn get_command_to_use(user_input: &UserInput) -> Option<AppCommand> {
    let mut command_to_use: Option<AppCommand> = None;
    match user_input.command.as_str() {
        "uppercase" => command_to_use = Some(AppCommand::Uppercase(user_input.text.clone())),
        "lowercase" => command_to_use = Some(AppCommand::Lowercase(user_input.text.clone())),
        "reverse" => command_to_use = Some(AppCommand::Reverse(user_input.text.clone())),
        _ => {}
    }
    command_to_use
}

pub fn validate_args(args: &Vec<String>) -> bool {
    let args_length: usize = args.len();
    if args_length != 3 {
        false;
        // process::exit(1);
    }
    true
}
