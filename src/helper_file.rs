pub mod helper_mod {
    use crate::UserInput;

    pub enum AppCommand {
        Uppercase(String),
        Lowercase(String),
        Reverse(String),
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
