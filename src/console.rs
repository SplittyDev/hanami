use core::ptr::Unique;
use core::fmt;
use cpuio::outb;
use spin::Mutex;

/// The buffer width.
const BUFFER_WIDTH: usize = 80;

/// The buffer height.
const BUFFER_HEIGHT: usize = 25;

/// The buffer size.
const BUFFER_SIZE: usize = BUFFER_WIDTH * BUFFER_HEIGHT;

/// A static textmode writer.
pub static CONSOLE: Mutex<TextWriter> = Mutex::new(TextWriter {
    x: 0,
    y: 0,
    color: CompositeColor::new(Color::DarkGray, Color::Black),
    buffer: unsafe { Unique::new(0xB8000 as *mut _) },
});

macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let mut writer = $crate::console::CONSOLE.lock();
        writer.write_fmt(format_args!($($arg)*)).unwrap();
    });
}

macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

/// A color.
#[repr(u8)]
#[allow(dead_code)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    LightMagenta = 13,
    Yellow = 14,
    White = 15,
}

/// A composite color.
#[derive(Copy, Clone)]
pub struct CompositeColor(u8);

/// Implements `CompositeColor`.
impl CompositeColor {
    /// Constructs a new `CompositeColor` from two colors.
    #[inline(always)]
    pub const fn new(fc: Color, bc: Color) -> CompositeColor {
        CompositeColor((fc as u8) << 4 | (bc as u8))
    }
}

/// A CharacterCell.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CharacterCell {
    chr: u8,
    color: CompositeColor,
}

/// Implements `CharacterCell`.
impl CharacterCell {
    /// Constructs a new `CharacterCell`.
    #[inline(always)]
    pub fn new(chr: u8, color: CompositeColor) -> CharacterCell {
        CharacterCell {
            chr: chr,
            color: color,
        }
    }
}

/// A text buffer.
struct TextBuffer {
    chars: [CharacterCell; BUFFER_SIZE],
}

/// A text writer.
pub struct TextWriter {
    x: usize,
    y: usize,
    color: CompositeColor,
    buffer: Unique<TextBuffer>,
}

/// Implements `TextWriter`.
impl TextWriter {
    /// Writes an ASCII character to the screen.
    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\r' => self.x = 0,
            b'\n' => self.newline(),
            b'\t' => {
                for _ in 0..(4 - (self.x % 4)) {
                    self.write_byte(b' ');
                }
            }
            0x08 => {
                let chr = CharacterCell::new(b' ', self.color);
                let off = TextWriter::offset(self.x, self.y);
                if self.y != 0 {
                    self.buffer(|buf| buf.chars[off] = chr);
                    match self.x {
                        0 => {
                            self.y -= 1;
                            self.x = BUFFER_WIDTH - 1;
                        }
                        _ => self.x -= 1,
                    }
                }
            }
            _ => {
                if self.x >= BUFFER_WIDTH {
                    self.newline()
                }
                let chr = CharacterCell::new(byte, self.color);
                let off = TextWriter::offset(self.x, self.y);
                self.buffer(|buf| buf.chars[off] = chr);
                self.x += 1;
            }
        }
        self.relocate_cursor();
    }
    /// Begins a new line.
    #[inline]
    fn newline(&mut self) {
        self.x = 0;
        match self.y {
            v if v < BUFFER_HEIGHT - 1 => self.y += 1,
            _ => self.scroll(),
        }
    }
    /// Scrolls the buffer.
    fn scroll(&mut self) {
        let chr = CharacterCell::new(b' ', self.color);
        self.buffer(|buf| {
            for y in 1..BUFFER_HEIGHT {
                for x in 0..BUFFER_WIDTH {
                    let off_a = TextWriter::offset(x, y - 1);
                    let off_b = TextWriter::offset(x, y);
                    buf.chars[off_a] = buf.chars[off_b];
                }
            }
            for x in 0..BUFFER_WIDTH {
                buf.chars[TextWriter::offset(x, BUFFER_HEIGHT - 1)] = chr;
            }
        });
    }
    /// Clears the buffer.
    pub fn clear(&mut self) {
        self.x = 0;
        self.y = 0;
        {
            let chr = CharacterCell::new(b' ', self.color);
            self.buffer(|buf| {
                for off in 0..BUFFER_SIZE {
                    buf.chars[off] = chr;
                }
            });
        }
        self.relocate_cursor();
    }
    /// Relocates the hardware cursor.
    fn relocate_cursor(&mut self) {
        let off = TextWriter::offset(self.x, self.y);
        let hi = (off >> 0x08) as u8;
        let lo = (off & 0xFF) as u8;
        unsafe {
            outb(0x0E, 0x03D4);
            outb(hi, 0x03D5);
            outb(0x0F, 0x03D4);
            outb(lo, 0x03D5);
        }
    }
    /// Retrieves a mutable reference to the buffer.
    fn buffer<F>(&mut self, mut callback: F)
        where F: FnMut(&mut TextBuffer)
    {
        callback(unsafe { self.buffer.get_mut() });
    }
    /// Calculates an offset into the buffer.
    #[inline(always)]
    fn offset(x: usize, y: usize) -> usize {
        y * BUFFER_WIDTH + x
    }
}

/// Implements `core::fmt::Write` for `TextWriter`.
impl fmt::Write for TextWriter {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        for byte in string.bytes() {
            self.write_byte(byte)
        }
        Ok(())
    }
}