#![allow(dead_code)]

use core::ops::Deref;
use spin::Mutex;
use rcstring::CString;

macro_rules! device_read {
    ($dev:expr, $buf:expr) => ($crate::device::DeviceRead::read(&$dev.proto, &$dev.info, $buf));
}

macro_rules! device_write {
    ($dev:expr, $buf:expr) => ($crate::device::DeviceWrite::write(&$dev.proto, &$dev.info, $buf));
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
    name: CString<'a>,
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
pub trait DeviceRead<B: Sized> {
    fn read(&self, dev: &DeviceInfo, buf: B);
}

/// Provides write functionality for devices.
pub trait DeviceWrite<B: Sized> {
    fn write(&self, dev: &DeviceInfo, buf: B);
}

/// Provides ioctl functionality for devices.
pub trait DeviceIoctl {
    fn ioctl(&self, dev: &DeviceInfo, arg1: i32, arg2: i32, arg3: i32);
}

impl<'a> DeviceInfo<'a> {
    /// Constructs a new `DeviceInfo`.
    pub fn new(kind: DeviceKind, name: CString) -> DeviceInfo {
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
    pub fn new(proto: P, kind: DeviceKind, name: CString) -> Device<P> {
        Device {
            proto: proto,
            info: DeviceInfo::new(kind, name),
        }
    }
}

impl<'a, P> Deref for Device<'a, P> {
    type Target = P;
    fn deref(&self) -> &P {
        &self.proto
    }
}