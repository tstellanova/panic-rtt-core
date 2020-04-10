//! Set the panicking behavior to log to a JLINK debugger and break.
//!
//! Currently, this crate only supports the ARM Cortex-M architecture.
//!
//! # Usage
//!
//! ``` ignore
//! #![no_std]
//!
//! use panic_rtt_core::{self, rtt_init_print, rprintln};
//!
//! fn main() {
//!     // you must create a print channel if you wish to see print output in RTT
//!     rtt_init_print!(NoBlockTrim);
//!     let value = 5;
//!     rprintln!("Hello world! {}", value);
//!     panic!("message logged to jlink debugger");
//! }
//! ```
#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]


use core::panic::PanicInfo;
use cortex_m::{asm, interrupt};
pub use rtt_target::{rprintln, rtt_init_print};

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    interrupt::disable();
    rprintln!("{}", info);

    loop {
        asm::bkpt();
    }
}




