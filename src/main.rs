use std::env;
use std::env::Args;

use text_transformer::run;

fn main() {
    let args: Args = env::args();

    if let Err(err) = run(args) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
