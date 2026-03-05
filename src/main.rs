use std::env;
use std::env::Args;

use text_transformer::run;

fn main() {
    let mut args: Args = env::args();

    if let Err(err) = run(&mut args) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
