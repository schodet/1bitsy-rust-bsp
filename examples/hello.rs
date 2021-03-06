//! Prints "Hello, world!" on the debugger console using semihosting.

#![feature(used)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;

use cortex_m::asm;
use core::fmt::Write;

fn main() {
    // Print a warm greeting.
    let mut stdout = cortex_m_semihosting::hio::hstdout().unwrap();
    writeln!(stdout, "Hello, world!").ok();
    // Break to debugger.
    asm::bkpt();
}

// As we are not using interrupts, we just register a dummy catch all handler.
#[link_section = ".vector_table.interrupts"]
#[used]
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C" fn default_handler() {
    asm::bkpt();
}
