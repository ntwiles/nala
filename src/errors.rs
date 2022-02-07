use std::process;

use crate::io_context::IoContext;

pub trait NalaRuntimeError {
    fn message(&self) -> String;
}

pub fn runtime_error(context: &mut dyn IoContext, error: impl NalaRuntimeError) -> ! {
    let message = error.message();
    context.print(&format!("Nala Runtime Error: {}", message));
    process::exit(0x0100);
}
