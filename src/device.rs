use core::ops::Deref;
use spin::Mutex;
use rcstring::{c_char, CString};

/// Next device id.
static mut next_device_id: usize = 0_usize;

macro_rules! device_write {
    ($dev:expr, $buf:expr) => ($crate::device::DeviceWrite::write(&$dev.dev, &$dev.info, $buf));
}

/// Device type.
pub enum DeviceType {
    BlockDevice = 0,
    CharsDevice = 1,
}

/// Device information.
pub struct DeviceInfo<'a> {
    device_id: usize,
    device_name: CString<'a>,
    device_type: DeviceType,
}

/// Device.
pub struct Device<'a, T> {
    pub dev: T,
    pub info: DeviceInfo<'a>,
}

/// Implements `Deref` for `Device`.
impl<'a, T> Deref for Device<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.dev
    }
}

/// Provides read functionality for devices.
pub trait DeviceRead<T: Sized> {
    fn read(&self, dev: &DeviceInfo, buf: T);
}

/// Provides write functionality for devices.
pub trait DeviceWrite<T: Sized> {
    fn write(&self, dev: &DeviceInfo, buf: T);
}

/// Provides ioctl functionality for devices.
pub trait DeviceIoctl {
    fn ioctl(&self, dev: &DeviceInfo, arg1: i32, arg2: i32, arg3: i32);
}

/// Implements `Device`.
impl<'a> DeviceInfo<'a> {
    /// Constructs a new `DeviceInfo`.
    pub fn new(device_type: DeviceType, name: CString) -> DeviceInfo {
        let id: usize;
        unsafe {
            id = next_device_id;
            next_device_id += 1;
        }
        DeviceInfo {
            device_id: id,
            device_name: name,
            device_type: device_type,
        }
    }
}

/// Implements `Device`.
impl<'a, T> Device<'a, T> {
    /// Constructs a new `Device`.
    pub fn new(dev: T, dev_type: DeviceType, dev_name: CString) -> Device<T> {
        Device {
            dev: dev,
            info: DeviceInfo::new(dev_type, dev_name),
        }
    }
}