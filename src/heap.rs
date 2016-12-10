#![allow(dead_code)]

use core;
use rlibc;

const ALIGN: usize = 8;
const GUARD1: u32 = 0x5EABFCD7;
const GUARD2: u32 = 0x52FCEDAB;

/// Block.
struct Block {
    size: usize,
    next: *mut Block,
    chunk: *const u8,
}

/// Heap.
pub struct Heap {
    used_top: *mut Block,
    free_top: *mut Block,
    free_addr: *mut u8,
}

impl Heap {
    pub fn new(end_of_kernel: usize) -> Self {
        let addr = Self::align(end_of_kernel) as *mut u8;
        klog!("Heap pointer: {:p}", addr);
        Heap {
            used_top: core::ptr::null_mut(),
            free_top: core::ptr::null_mut(),
            free_addr: addr,
        }
    }
    pub fn alloc<T>(&mut self) -> Option<&T> {
        let ptr = match self.kalloc(core::mem::size_of::<T>()) {
            Some(val) => val as *mut T,
            None => return None,
        };
        match unsafe { ptr.as_ref() } {
            Some(ref val) => Some(*val),
            None => None,
        }
    }
    fn kalloc(&mut self, size: usize) -> Option<*mut u8> {
        let new_block = {
            if let Some(block) = self.internal_get_block(size) {
                block
            } else {
                let block = self.internal_alloc(core::mem::size_of::<Block>()) as *mut Block;
                {
                    let mut block_ref = match unsafe { block.as_mut() } {
                        Some(val) => val,
                        None => return None,
                    };
                    block_ref.size = size;
                    block_ref.chunk = self.internal_alloc(size);
                }
                block
            }
        };
        let mut block_ref = match unsafe { new_block.as_mut() } {
            Some(val) => val,
            None => return None,
        };
        block_ref.next = self.used_top;
        self.used_top = new_block;
        unsafe {
            rlibc::memset(block_ref.chunk as *mut _, 0, size);
        }
        Some(block_ref.chunk as *mut _)
    }
    fn internal_alloc(&mut self, size: usize) -> *mut u8 {
        let guard_size = core::mem::size_of::<u32>() * 2;
        let aligned = Self::align(size) + guard_size;
        {
            let sys = aligned;
            let real = aligned - guard_size;
            let sys_loss = ((sys - size) * 100) / sys;
            let real_loss = ((real - size) * 100) / real;
            klog!("[kalloc] req={} alloc=[sys={} real={}] loss=[sys={} real={}]",
                  size,
                  sys,
                  real,
                  sys_loss,
                  real_loss);
        }
        unsafe {
            *(self.free_addr as *mut u32) = GUARD1;
            *(self.free_addr.offset((aligned + 4) as isize) as *mut u32) = GUARD2;
            self.free_addr = self.free_addr.offset(aligned as isize);
            self.free_addr.offset((4 - aligned) as isize)
        }
    }
    fn internal_get_block(&mut self, size: usize) -> Option<*mut Block> {
        let mut i: *mut Block = self.free_top;
        let mut p: *mut Block = i;
        while !i.is_null() {
            let ival = match unsafe { i.as_ref() } {
                Some(val) => val,
                None => return None,
            };
            let mut pval = match unsafe { p.as_mut() } {
                Some(val) => val,
                None => return None,
            };
            if ival.size > size {
                if i == p {
                    self.free_top = ival.next;
                } else {
                    pval.next = ival.next;
                }
                return Some(i);
            }
            p = i;
            i = ival.next;
        }
        None
    }
    fn align(addr: usize) -> usize {
        (addr % ALIGN) + addr
    }
}