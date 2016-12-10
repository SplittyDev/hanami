//! Hanami kernel.

#![allow(non_upper_case_globals)]
// #![feature(core_intrinsics)]
#![feature(static_in_const)]
#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique)]
#![no_std]

extern crate rlibc;
extern crate spin;
extern crate cpuio;
extern crate multiboot2;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rcstring;

macro_rules! klog {
    ($f:expr $(,$arg:expr)*) => {
        device_write!($crate::serial0, concat!($f, "\r\n") $(,$arg)*);
    };
}

macro_rules! print {
    ($f:expr $(,$arg:expr)*) => {
        device_write!($crate::ktty0, $f $(,$arg)*);
    };
}

macro_rules! println {
    ($f:expr $(,$arg:expr)*) => {
        print!(concat!($f, "\r\n") $(,$arg)*);
    };
}

#[macro_use]
mod device;
mod heap;
mod serial;
mod terminal;

/// Macro for constructing thread-safe devices.
macro_rules! device {
    ($name:ident, $kind:ident, $t:path, $val:expr) => {
        lazy_static! {
            pub static ref $name
                : $crate::device::ThreadSafeDevice<$t>
                = spin::Mutex::new(device::Device::new(
                    $val, device::DeviceKind::$kind, stringify!($name)));
        }
    };
}

// /dev/serial0
device!(serial0,
        CharsDevice,
        serial::SerialDevice,
        serial::SerialDevice::new(serial::SERIAL0));

// /dev/ktty0
device!(ktty0,
        CharsDevice,
        terminal::TerminalDevice,
        terminal::TerminalDevice::new(terminal::VGA_PTR));

#[no_mangle]
pub extern "C" fn kmain(mb_addr: usize) -> ! {
    let boot_info = unsafe { multiboot2::load(mb_addr) };
    let mut heap: heap::Heap = heap::Heap::new(boot_info.end_address());
    println!("Hello from Hanami!");
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn rust_eh_personality() {}

#[lang = "panic_fmt"]
extern "C" fn rust_begin_panic() -> ! {
    klog!("*** PANIC!");
    loop {}
}