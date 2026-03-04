use std::env;
use std::env::Args;

use text_transformer::run;

fn main() {
    let args: Args = env::args();

    run(args);
}
