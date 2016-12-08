#![allow(dead_code)]

use core::{self, fmt};
use spin::Mutex;

macro_rules! device_write {
    ($dev:expr $(,$arg:expr)*) => ({
        use core::fmt::Write;
        let mut writer = $dev.lock();
        writer.write_fmt(format_args!($($arg,)*)).unwrap();
    });
}

/// Next device id.
static mut NEXT_DEVICE_ID: Mutex<usize> = Mutex::new(0_usize);

/// Device kind.
pub enum DeviceKind {
    BlockDevice = 0,
    CharsDevice = 1,
}

/// Device information.
pub struct DeviceInfo<'a> {
    id: usize,
    name: &'a str,
    kind: DeviceKind,
}

/// Device.
pub struct Device<'a, P> {
    pub proto: P,
    pub info: DeviceInfo<'a>,
}

/// Device manager.
pub struct DeviceManager;

/// Provides read functionality for devices.
pub trait DeviceRead {
    fn read_byte(&self, dev: &DeviceInfo) -> u8;
    fn read_chunk(&self, dev: &DeviceInfo, buf: &[u8], size: usize);
}

/// Provides write functionality for devices.
pub trait DeviceWrite {
    fn write_byte(&self, dev: &DeviceInfo, b: u8);
}

/// Provides ioctl functionality for devices.
pub trait DeviceIoctl {
    // TODO: Rethink this.
    fn ioctl(&self, dev: &DeviceInfo, arg1: i32, arg2: i32, arg3: i32);
}

impl<'a> DeviceInfo<'a> {
    /// Constructs a new `DeviceInfo`.
    pub fn new(kind: DeviceKind, name: &str) -> DeviceInfo {
        DeviceInfo {
            id: DeviceInfo::get_next_id_safe(),
            name: name,
            kind: kind,
        }
    }
    /// Gets the next device id in a thread-safe manner.
    fn get_next_id_safe() -> usize {
        unsafe {
            let mut data = NEXT_DEVICE_ID.lock();
            let id = *data;
            *data += 1;
            id
        }
    }
}

impl<'a, P> Device<'a, P> {
    /// Constructs a new `Device`.
    pub fn new(proto: P, kind: DeviceKind, name: &'a str) -> Device<P> {
        Device {
            proto: proto,
            info: DeviceInfo::new(kind, name),
        }
    }
}

impl<'a, P> fmt::Write for Device<'a, P>
    where P: DeviceWrite
{
    fn write_str(&mut self, string: &str) -> fmt::Result {
        for b in string.bytes() {
            self.write_byte(&self.info, b);
        }
        Ok(())
    }
}

impl<'a, P> core::ops::Deref for Device<'a, P> {
    type Target = P;
    fn deref(&self) -> &P {
        &self.proto
    }
}