
pub mod user_input_mod {
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
}
