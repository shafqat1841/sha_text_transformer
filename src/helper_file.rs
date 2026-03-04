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
            AppCommand::Uppercase(text) => write!(f, "uppercase: {}", text),
            AppCommand::Lowercase(text) => write!(f, "lowercase: {}", text),
            AppCommand::Reverse(text) => write!(f, "reverse: {}", text),
        }
    }
}

impl AppCommand {
    // pub fn get_command_to_use(user_input: &UserInput) -> Option<AppCommand> {
    pub fn get_command_to_use(user_input: UserInput) -> Result<AppCommand, &'static str> {
        match user_input.command.as_str() {
            "uppercase" => Ok(AppCommand::Uppercase(user_input.text.to_uppercase())),
            "lowercase" => Ok(AppCommand::Lowercase(user_input.text.to_lowercase())),
            "reverse" => Ok(AppCommand::Reverse(user_input.text.chars().rev().collect())),
            _ => Err("Not valid command"),
        }
    }
}
