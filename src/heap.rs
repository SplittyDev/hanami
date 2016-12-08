use core;
use rlibc;

const ALIGN: usize = 8;
const HEADER: usize = 0x5F597A35;
const GUARD1: usize = 0x51EBFCD7;
const GUARD2: usize = 0x52FBEDAC;

/// Block.
struct Block {
    size: usize,
    next: *mut Block,
    chunk: *const usize,
}

/// Heap.
pub struct Heap {
    used_top: *mut Block,
    free_top: *mut Block,
    free_addr: *mut usize,
}

impl Heap {
    /// Constructs a new `Heap`.
    pub fn new(end_of_kernel: usize) -> Self {
        Heap {
            used_top: core::ptr::null_mut(),
            free_top: core::ptr::null_mut(),
            free_addr: Self::align(end_of_kernel) as *mut _,
        }
    }
    /// Allocates a chunk of memory.
    pub fn kalloc(&mut self, size: usize) -> Option<*const usize> {
        let new_block = {
            let block = self.get_block(size);
            if block.is_null() {
                let block = self.alloc(core::mem::size_of::<Block>()) as *mut Block;
                {
                    let mut block_ref = match unsafe { block.as_mut() } {
                        Some(val) => val,
                        None => return None,
                    };
                    block_ref.size = size;
                    block_ref.chunk = self.alloc(size);
                }
                block
            } else {
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
        Some(block_ref.chunk)
    }
    fn alloc(&mut self, size: usize) -> *mut usize {
        let aligned = Self::align(size) + 8;
        unsafe {
            *self.free_addr = GUARD1;
            *self.free_addr.offset((aligned + 4) as isize) = GUARD2;
            self.free_addr = self.free_addr.offset(aligned as isize);
            ((*self.free_addr - aligned) + 4) as *mut _
        }
    }
    fn get_block(&mut self, size: usize) -> *mut Block {
        let mut i: *mut Block = self.free_top;
        let mut p: *mut Block = i;
        while !i.is_null() {
            let ival = match unsafe { i.as_ref() } {
                Some(val) => val,
                None => return core::ptr::null_mut(),
            };
            let mut pval = match unsafe { p.as_mut() } {
                Some(val) => val,
                None => return core::ptr::null_mut(),
            };
            if ival.size > size {
                if i == p {
                    self.free_top = ival.next;
                } else {
                    pval.next = ival.next;
                }
                return i;
            }
            p = i;
            i = ival.next;
        }
        core::ptr::null_mut()
    }
    /// Aligns an address.
    fn align(addr: usize) -> usize {
        (addr % ALIGN) + addr
    }
}