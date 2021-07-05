use super::{MemEType, MemEntry, ENTRIES_COUNT, PAGE_SIZE};
use crate::{info, ok};
use core::ptr::{self, null_mut};

#[derive(Debug)]
pub struct Bitmap {
    size: u64,
    start: *mut u8,
}
impl Bitmap {
    pub const fn new() -> Self {
        Self {
            size: 0,
            start: null_mut(),
        }
    }
    fn get_byte(&self, idx: usize) -> *mut u8 {
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
}

fn get_bm_array_idx(addr: u64) -> usize {
    addr as usize / 8
}
fn get_bm_bit_idx(addr: u64) -> usize {
    addr as usize % 8
}

static mut BITMAP: Bitmap = Bitmap::new();

pub fn init_pmm(memory_map: [MemEntry; ENTRIES_COUNT]) {
    let mut bm_start: *mut u8 = null_mut();
    let mut available = 0;
    let mem_size = memory_map.iter().fold(0, |size, entry| size + entry.size);
    let free = memory_map.iter().filter(|e| e.etype == MemEType::Usable);

    // Find a place for the bitmap pointer
    free.clone().for_each(|e| {
        if bm_start.is_null() {
            bm_start = e.start as *mut u8;
        }
        info!(
            "Usable entry: {{ start: {:#016x}, size: {} }}",
            e.start, e.size
        );
    });
    let bm_size = mem_size / (PAGE_SIZE * 8);

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
    unsafe {
        BITMAP = Bitmap {
            size: bm_size,
            start: bm_start,
        };
    }
    ok!(
        "Bitmap stored at {:#08x} ({} bytes long).",
        BITMAP.start as usize,
        BITMAP.size
    );
}
