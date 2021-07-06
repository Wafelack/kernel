use super::{MemEType, MemEntry, ENTRIES_COUNT, PAGE_SIZE};
use crate::{info, ok, err};
use core::ptr::{self, null_mut};

#[derive(Debug)]
pub struct Bitmap {
    size: u64,
    start: *mut u8,
}
static mut LAST_FREE: u64 = 0;
impl Bitmap {
    pub const fn new() -> Self {
        Self {
            size: 0,
            start: null_mut(),
        }
    }
    fn get_byte(&self, idx: usize) -> *mut u8 {
        assert!(idx < self.size as usize);
        (self.start as usize + idx) as *mut u8
    }
    pub fn set_bit(&mut self, addr: u64) {
        let bit = get_bm_bit_idx(addr);
        let byte = get_bm_array_idx(addr);
        unsafe {
            *self.get_byte(byte) |= 1 << bit;
        }
    }
    pub fn clear_bit(&mut self, addr: u64) {
        let bit = get_bm_bit_idx(addr);
        let byte = get_bm_array_idx(addr);
        unsafe {
            *self.get_byte(byte) &= !(1 << bit);
        }
    }
    pub fn is_set(&self, addr: u64) -> bool {
        let bit = get_bm_bit_idx(addr);
        let byte = get_bm_array_idx(addr);
        (unsafe { *self.get_byte(byte)  } & (1 << bit)) != 0
    }
    pub fn find_free_pages(&self, count: u64) -> Option<u64> {
        let (mut free_count, mut i) = (unsafe { LAST_FREE }, 0);
        let page_count = unsafe { MEM_SIZE } / PAGE_SIZE;

        while i < page_count {
            while unsafe { *self.get_byte(i as usize / 8) == 0xFF } && i < page_count-8 {
                free_count = 0;
                i += 8 - (i % 8);
            }

            if !self.is_set(i) {
                free_count += 1;
                if free_count == count {
                    return Some(i);
                }
            } else {
                free_count = 0;
            }

            if i == page_count - 1 {
                unsafe { LAST_FREE = 0};
                return self.find_free_pages(count);
            }

            i += 1;
        }
        err!("Kernel is out of memory.");
        return None;
    }
}

fn get_bm_array_idx(addr: u64) -> usize {
    addr as usize / 8
}
fn get_bm_bit_idx(addr: u64) -> usize {
    addr as usize % 8
}

static mut BITMAP: Bitmap = Bitmap::new();
static mut MEM_SIZE: u64 = 0;

pub fn init_pmm(memory_map: [MemEntry; ENTRIES_COUNT]) {
    let mut bm_start: *mut u8 = null_mut();
    let mut available = 0;
    unsafe { MEM_SIZE = memory_map.iter().fold(0, |size, entry| size + entry.size) };
    let mem_size = unsafe { MEM_SIZE };
    let free = memory_map.iter().filter(|e| e.etype == MemEType::Usable);

    // Find a place for the bitmap pointer
    free.clone().for_each(|e| {
        if bm_start.is_null() {
            bm_start = e.start as *mut u8;
        }
        info!(
            "Usable entry: {{ start: {:#08x}, size: {} }}",
            e.start, e.size
        );
    });
    let bm_size = mem_size / (PAGE_SIZE * 8);
    unsafe {
        BITMAP = Bitmap {
            size: bm_size,
            start: bm_start,
        };
    }

    // Set all entries to used
    unsafe {
        ptr::write_bytes(bm_start, 0xFF, bm_size as usize);
    }

    // Set free entries to free
    free.for_each(|e| {
        (e.start..(e.start + e.size))
            .step_by(PAGE_SIZE as usize)
            .for_each(|i| {
                unsafe { BITMAP.clear_bit(i / PAGE_SIZE) };
                available += PAGE_SIZE;
            })
    });

    // Mark the bitmap zone as used
    (bm_start as u64..bm_start as u64 + bm_size)
        .step_by(PAGE_SIZE as usize)
        .for_each(|i| unsafe { BITMAP.set_bit(i / PAGE_SIZE) });

    info!(
        "{}/{} bytes available ({} pages).",
        available,
        mem_size,
        available / PAGE_SIZE
    );
    
    ok!(
        "Bitmap stored at {:#08x} ({} bytes long).",
        BITMAP.start as usize,
        BITMAP.size
    );
}

pub fn alloc_page(count: u64) -> Option<u64> {
    if let Some(page) = unsafe { BITMAP.find_free_pages(count)  }{
        for i in page..count+page {
            unsafe { BITMAP.set_bit(i) };
        }
        Some (page * PAGE_SIZE)
    } else {
        None
    }
}
pub fn free_page(addr: u64, count: u64) {
    let target = addr / PAGE_SIZE;
    
    for i in target..target+count {
        unsafe { BITMAP.clear_bit(i) };
    }
    unsafe { LAST_FREE = target };
}
