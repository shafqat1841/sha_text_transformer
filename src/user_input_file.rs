
use std::env::Args;

pub struct UserInput {
    pub text: String,
    pub command: String,
}

impl UserInput {
    pub fn build(mut args: Args) -> Result<UserInput, &'static str> {
        args.next();
        let text = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get the text query string"),
        };

        let command = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get the command query string"),
        };

        Ok(UserInput {
            text,
            command: command.to_lowercase(),
        })
    }
}
