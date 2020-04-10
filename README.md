# panic-rtt-core

Set the panicking behavior to log to a JLINK debugger and break.
This leverages the [`rtt-target`](https://crates.io/crates/rtt-target) crate.

Currently, this crate only supports the ARM Cortex-M architecture.

# Example
``` 
#![no_std]
use panic_rtt_core::{self, rtt_init_print, rprintln};

fn main() {
  // you must create a print channel if you wish to see print output in RTT
  rtt_init_print!(NoBlockTrim);
  let value = 5;
  rprintln!("Hello world! {}", value);
  panic!("message logged to jlink debugger");
}
```