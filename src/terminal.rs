use core::ptr::Unique;
use device::*;
use cpuio::outb;

/// The address of the framebuffer in memory.
pub const VGA_PTR: usize = 0xB8000;

const VGA_SIZE: usize = VGA_WIDTH * VGA_HEIGHT;
const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;

macro_rules! color {
    ($fc:expr, $bc:expr) => (bc << 4 | fc)
}

macro_rules! chattr {
    ($b:expr, $c:expr) => (($c as u16) << 8 | ($b as u16));
}

macro_rules! offset {
    ($x:expr, $y:expr) => ($y * VGA_WIDTH + $x)
}

type TerminalBuffer = Unique<[u16; VGA_SIZE]>;

pub struct TerminalDevice {
    x: usize,
    y: usize,
    color: u8,
    buf: TerminalBuffer,
}

impl TerminalDevice {
    pub fn new(ptr: usize) -> Self {
        let mut term = TerminalDevice {
            x: 0,
            y: 0,
            color: 0x80,
            buf: unsafe { Unique::new(ptr as *mut _) },
        };
        term.clear();
        term
    }
    pub fn clear(&mut self) {
        let chr = chattr!(b' ', self.color);
        let buf = unsafe { self.buf.get_mut() };
        for i in 0..VGA_SIZE {
            buf[i] = chr;
        }
    }
    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\r' => self.x = 0,
            b'\n' => self.new_line(),
            b'\t' => {
                const tab_size: usize = 4;
                for _ in 0..(tab_size - (self.x % tab_size)) {
                    self.write_byte(b' ');
                }
            }
            0x08 => {
                let chr = chattr!(b' ', self.color);
                let off = offset!(self.x, self.y);
                let buf = unsafe { self.buf.get_mut() };
                if self.y != 0 {
                    buf[off] = chr;
                    match self.x {
                        0 => {
                            self.y -= 1;
                            self.x = VGA_WIDTH - 1;
                        }
                        _ => self.x -= 1,
                    }
                }
            }
            _ => {
                if self.x >= VGA_WIDTH {
                    self.new_line();
                }
                let chr = chattr!(byte, self.color);
                let off = offset!(self.x, self.y);
                self.x += 1;
                unsafe {
                    self.buf.get_mut()[off] = chr;
                }
            }
        }
        self.update_physical_cursor();
    }
    fn new_line(&mut self) {
        self.x = 0;
        if self.y < VGA_HEIGHT - 1 {
            self.y += 1;
        } else {
            self.scroll();
        }
    }
    fn scroll(&mut self) {
        let chr = chattr!(b' ', self.color);
        let buf = unsafe { self.buf.get_mut() };
        for y in 1..VGA_HEIGHT {
            for x in 0..VGA_WIDTH {
                let off = offset!(x, y);
                buf[off - VGA_HEIGHT] = buf[off];
            }
        }
        for x in 0..VGA_WIDTH {
            buf[VGA_SIZE - VGA_WIDTH + x] = chr;
        }
    }
    fn update_physical_cursor(&mut self) {
        let off = offset!(self.x, self.y);
        unsafe {
            outb(0x0E, 0x03D4);
            outb((off >> 0x08) as u8, 0x03D5);
            outb(0x0F, 0x03D4);
            outb((off & 0xFF) as u8, 0x03D5);
        }
    }
}

impl DeviceWrite for TerminalDevice {
    fn write_byte(&mut self, _: &DeviceInfo, b: u8) {
        self.write_byte(b);
    }
}