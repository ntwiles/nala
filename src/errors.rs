use std::process;

pub trait NalaRuntimeError {
    fn message(&self) -> String;
}

pub fn runtime_error(error: impl NalaRuntimeError) -> ! {
    let message = error.message();
    println!("Nala Runtime Error: {}", message);
    process::exit(0x0100);
}
