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

#[macro_use]
mod console;
use console::CONSOLE;
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

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    let COM1 = SerialDevice::new(serial::COM1);
    device_write!(COM1, "Hello, world!");
    loop {}
}