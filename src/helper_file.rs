pub mod helper_mod {
    use core::fmt;

    use crate::UserInput;

    pub enum AppCommand {
        Uppercase(String),
        Lowercase(String),
        Reverse(String),
    }

    impl fmt::Display for AppCommand {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                AppCommand::Uppercase(text) => write!(f, "uppercase: {}", text.to_uppercase()),
                AppCommand::Lowercase(text) => write!(f, "lowercase: {}", text.to_lowercase()),
                AppCommand::Reverse(text) => {
                    let chars = text.chars();
                    let reversed_chars = chars.rev();
                    let reversed_text: String = reversed_chars.collect();
                    write!(f, "reverse: {}", reversed_text)
                }
            }
        }
    }

    // pub fn get_command_to_use(user_input: &UserInput) -> Option<AppCommand> {
    pub fn get_command_to_use(user_input: UserInput) -> Result<AppCommand, &'static str> {
        match user_input.command.as_str() {
            "uppercase" => Ok(AppCommand::Uppercase(user_input.text)),
            "lowercase" => Ok(AppCommand::Lowercase(user_input.text)),
            "reverse" => Ok(AppCommand::Reverse(user_input.text)),
            _ => Err("Not valid command"),
        }
    }
}
