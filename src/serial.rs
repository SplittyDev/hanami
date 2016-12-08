use device::*;
use cpuio::{inb, outb};

pub const SERIAL0: u16 = 0x03F8;
pub const SERIAL1: u16 = 0x02F8;
pub const SERIAL2: u16 = 0x03E8;
pub const SERIAL3: u16 = 0x02E8;

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

impl SerialDevice {
    /// Constructs a new serial device.
    pub fn new<'a>(port: u16) -> Device<'a, SerialDevice> {
        let name = match port {
            SERIAL0 => "serial0",
            SERIAL1 => "serial1",
            SERIAL2 => "serial2",
            SERIAL3 => "serial3",
            _ => unreachable!(),
        };
        let dev = SerialDevice { port: port };
        dev.initialize();
        Device::new(dev, DeviceKind::CharsDevice, name)
    }
    /// Initializes the serial port.
    #[inline]
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
    #[inline]
    fn await_ready_state(&self) {
        loop {
            if unsafe { inb(self.port + 0x05) } & 0x20 != 0 {
                break;
            }
        }
    }
    /// Writes a byte to the serial port.
    #[inline]
    fn write_byte(&self, b: u8) {
        self.await_ready_state();
        unsafe {
            outb(b, self.port);
        }
    }
}

impl DeviceWrite for SerialDevice {
    fn write_byte(&self, _: &DeviceInfo, b: u8) {
        self.write_byte(b);
    }
}