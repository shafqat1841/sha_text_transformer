use std::env;

fn main() {
    let args2: Vec<String> = env::args().collect();
    println!("args2: {:?}", args2);
}
