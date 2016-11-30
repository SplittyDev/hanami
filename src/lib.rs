#![feature(lang_items)]
#![no_std]

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[lang = "panic_fmt"]
extern "C" fn rust_begin_panic() -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    loop {}
}