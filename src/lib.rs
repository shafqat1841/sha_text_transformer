mod helper_file;
mod user_input_file;

pub use crate::helper_file::AppCommand;
pub use crate::user_input_file::UserInput;

pub fn run(args: std::env::Args) -> Result<(), &'static str> {
    if args.len() != 3 {
        eprintln!("Usage: cargo run <arg1: Text> <arg2: Command>");
        return Err("Wrong number of arguments passed");
    }

    let user_input: UserInput = UserInput::build(args)?;

    let command_to_use: AppCommand = AppCommand::get_command_to_use(user_input)?;

    println!("{}", command_to_use);

    Ok(())
}
