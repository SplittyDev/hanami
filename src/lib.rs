//! Hanami kernel.

#![deny(missing_docs)]
#![feature(static_in_const)]
#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique)]
#![no_std]

extern crate rlibc;
extern crate spin;
extern crate cpuio;
#[macro_use]
extern crate rcstring;
#[macro_use]
extern crate lazy_static;

mod heap;
#[macro_use]
mod console;
#[macro_use]
mod device;
mod serial;

use serial::SerialDevice;

#[lang = "eh_personality"]
extern "C" fn rust_eh_personality() {}

#[lang = "panic_fmt"]
extern "C" fn rust_begin_panic() -> ! {
    loop {}
}

/// Main entry point.
#[no_mangle]
pub extern "C" fn kmain() -> ! {
    let serial0 = SerialDevice::new(serial::SERIAL0);
    device_write!(serial0, "Hello, world!");
    loop {}
}