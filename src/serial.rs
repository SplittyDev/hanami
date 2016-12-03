use device::*;
use rcstring::CString;
use cpuio::{inb, outb};

pub const COM1: u16 = 0x03F8;
pub const COM2: u16 = 0x02F8;
pub const COM3: u16 = 0x03E8;
pub const COM4: u16 = 0x02E8;

macro_rules! serial_ier { ($port:expr) => ($port + 0x01); }
macro_rules! serial_data { ($port:expr) => ($port + 0x00); }
macro_rules! serial_fifo { ($port:expr) => ($port + 0x02); }
macro_rules! serial_line { ($port:expr) => ($port + 0x03); }
macro_rules! serial_modem { ($port:expr) => ($port + 0x04); }
macro_rules! serial_line_status { ($port:expr) => ($port + 0x05); }

/// Serial device.
pub struct SerialDevice {
    port: u16,
}

/// Implements `SerialDevice`.
impl SerialDevice {
    /// Constructs a new serial device.
    pub fn new<'a>(port: u16) -> Device<'a, SerialDevice> {
        fn create_device<'a>(dev: SerialDevice) -> Device<'a, SerialDevice> {
            let name = match dev.port {
                COM1 => cstr!("ttyS0"),
                COM2 => cstr!("ttyS1"),
                COM3 => cstr!("ttyS2"),
                COM4 => cstr!("ttyS3"),
                _ => unreachable!(),
            };
            Device::new(dev, DeviceType::CharsDevice, name)
        }
        let dev = SerialDevice { port: port };
        dev.initialize();
        create_device(dev)
    }
    /// Initializes the serial port.
    fn initialize(&self) {
        unsafe {
            outb(0x00, serial_ier!(self.port));
            outb(0x80, serial_line!(self.port));
            outb(0x03, serial_data!(self.port));
            outb(0x00, serial_ier!(self.port));
            outb(0x03, serial_line!(self.port));
            outb(0xc7, serial_fifo!(self.port));
            outb(0x03, serial_modem!(self.port));
        }
    }
    /// Waits till the serial port is ready.
    fn await_ready_state(&self) {
        loop {
            if unsafe { inb(self.port + 0x05) } & 0x20 != 0 {
                break;
            }
        }
    }
    /// Writes a byte to the serial port.
    fn write_byte(&self, b: u8) {
        self.await_ready_state();
        unsafe {
            outb(b, self.port);
        }
    }
}

/// Implements `SerialWrite<&str>` for `SerialDevice`.
impl<'a> DeviceWrite<&'a str> for SerialDevice {
    fn write(&self, dev: &DeviceInfo, buf: &'a str) {
        for b in buf.bytes() {
            self.write_byte(b);
        }
    }
}

/// Implements `SerialWrite<CString>` for `SerialDevice`.
impl<'a> DeviceWrite<CString<'a>> for SerialDevice {
    fn write(&self, dev: &DeviceInfo, buf: CString<'a>) {
        let ptr = unsafe { buf.into_raw() };
        for i in 0..buf.len() {
            let b = unsafe { ptr.offset(i as isize) };
            self.write_byte(unsafe { *b } as u8);
        }
    }
}