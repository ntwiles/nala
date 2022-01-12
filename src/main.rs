use std::env;

use library;

fn main() -> () {
    let args: Vec<String> = env::args().collect();
    library::main(&args[1]);
}
