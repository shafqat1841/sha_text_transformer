pub mod helper_mod {
    use crate::UserInput;


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
    
    pub fn validate_args(args: &[String]) -> bool {
        args.len() == 3
    }
}

