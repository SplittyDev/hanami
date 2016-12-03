#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique)]
#![no_std]

extern crate rlibc;
extern crate spin;
extern crate cpuio;

#[macro_use]
mod console;
use console::CONSOLE;

#[lang = "eh_personality"]
extern "C" fn rust_eh_personality() {}

#[lang = "panic_fmt"]
extern "C" fn rust_begin_panic() -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    CONSOLE.lock().clear();
    println!("Hello, world!");
    loop {}
}