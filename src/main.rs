use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let args_length: usize = args.len();
    println!("{} arguments passed",args_length.to_string());

    if args_length > 3 {
        println!("The limit of args is 2");
        println!("Usage: cargo run <arg1> <arg2>");
        return;
    }

    if args_length < 3 {
        println!("Not enough arguments passed");
        println!("Usage: cargo run <arg1> <arg2>");
        return;
    }

    for args_items in &args {
        println!("args: {}", args_items);
    }
    println!("args: {:?}", args);
}
