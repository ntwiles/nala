use std::env;
use std::error::Error;

use library;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    library::main(&args[1])
}
