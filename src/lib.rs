//! Hanami kernel.

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

#[macro_use]
mod klog;
#[macro_use]
mod device;
mod heap;
mod console;
mod serial;

/// Thread-safe `device::Device<T>` wrapped in a `spin::Mutex`.
pub type ThreadSafeDevice<T> = spin::Mutex<device::Device<'static, T>>;

/// Macro for constructing thread-safe devices.
macro_rules! device {
    ($name:ident, $t:path, $val:expr) => {
        lazy_static! {
            #[allow(missing_docs)]
            pub static ref $name
                : ThreadSafeDevice<$t>
                = spin::Mutex::new($val);
        }
    };
}

// /dev/serial0
device!(DEV_SERIAL0,
        serial::SerialDevice,
        serial::SerialDevice::new(serial::SERIAL0));

#[no_mangle]
pub extern "C" fn kmain(mb_addr: usize) -> ! {
    let boot_info = unsafe { multiboot2::load(mb_addr) };
    let mut heap: heap::Heap = heap::Heap::new(boot_info.end_address());
    klog!("Hello from Hanami!");
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn rust_eh_personality() {}

#[lang = "panic_fmt"]
extern "C" fn rust_begin_panic() -> ! {
    klog!("*** PANIC!");
    loop {}
}