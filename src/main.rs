use std::env;

use nala_interpreter;

fn main() -> () {
    let args: Vec<String> = env::args().collect();
    nala_interpreter::main(&args[1]);
}
