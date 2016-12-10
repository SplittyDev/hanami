#![allow(dead_code)]
use cpuio;

// General
const PIC_EOI: u8 = 0x20;

// Masking
const PIC_UNMASK: u8 = 0x00;
const PIC_MASK: u8 = 0xFF;

// PIC 1 (Master)
const PIC_MASTER_COMMAND: u16 = 0x20;
const PIC_MASTER_DATA: u16 = 0x21;

// PIC 2 (Slave)
const PIC_SLAVE_COMMAND: u16 = 0xA0;
const PIC_SLAVE_DATA: u16 = 0xA1;

// ICW 1
const PIC_ICW1_ICW4: u8 = 0x01;
const PIC_ICW1_SINGLE: u8 = 0x02;
const PIC_ICW1_INTERVAL4: u8 = 0x04;
const PIC_ICW1_LEVEL: u8 = 0x08;
const PIC_ICW1_INIT: u8 = 0x10;

// ICW 2
const PIC_ICW2_MASTER_OFF: u8 = 0x20;
const PIC_ICW2_SLAVE_OFF: u8 = 0x28;

// ICW 3
const PIC_ICW3_CASCADE: u8 = 0x02;
const PIC_ICW3_IRQ2_SLAVE: u8 = 0x04;

// ICW 4
const PIC_ICW4_8086: u8 = 0x01;
const PIC_ICW4_AUTO: u8 = 0x02;
const PIC_ICW4_BUF_SLAVE: u8 = 0x08;
const PIC_ICW4_BUF_MASTER: u8 = 0x0C;
const PIC_ICW4_SFNM: u8 = 0x10;

pub struct PIC;
impl PIC {
    pub fn remap() {
        unsafe {
            PIC::remap_master();
            PIC::remap_slave();
            PIC::enable();
        }
    }
    #[inline]
    unsafe fn remap_master() {
        PIC::outb_wait(PIC_MASTER_COMMAND, PIC_ICW1_INIT + PIC_ICW1_ICW4);
        PIC::outb_wait(PIC_MASTER_DATA, PIC_ICW2_MASTER_OFF);
        PIC::outb_wait(PIC_MASTER_DATA, PIC_ICW3_IRQ2_SLAVE);
        PIC::outb_wait(PIC_MASTER_DATA, PIC_ICW4_8086);
    }
    #[inline]
    unsafe fn remap_slave() {
        PIC::outb_wait(PIC_SLAVE_COMMAND, PIC_ICW1_INIT + PIC_ICW1_ICW4);
        PIC::outb_wait(PIC_SLAVE_DATA, PIC_ICW2_SLAVE_OFF);
        PIC::outb_wait(PIC_SLAVE_DATA, PIC_ICW3_CASCADE);
        PIC::outb_wait(PIC_SLAVE_DATA, PIC_ICW4_8086);
    }
    #[inline]
    unsafe fn enable() {
        PIC::outb_wait(PIC_MASTER_DATA, PIC_UNMASK);
        PIC::outb_wait(PIC_SLAVE_DATA, PIC_UNMASK);
    }
    #[inline(always)]
    unsafe fn outb_wait(addr: u16, val: u8) {
        cpuio::outb(val, addr);
        cpuio::outb(0x00, 0x80);
    }
}