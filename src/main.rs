#![no_std]
#![no_main]
#![feature(asm, panic_info_message, abi_x86_interrupt)]

mod log;
mod memory;
mod panic;
mod serial;
mod x86_64;
mod tests;

use memory::pmm::{init_memory_map, init_pmm, MemEntry, ENTRIES_COUNT};
use serial::Serial;
use stivale_boot::v2::{StivaleHeader, StivaleStruct};

const STACK_SIZE: usize = 8192;
static STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

#[link_section = ".stivale2hdr"]
#[no_mangle]
#[used]
static STIVALE_HDR: StivaleHeader = StivaleHeader::new()
    .stack(&STACK[STACK_SIZE - 1] as *const u8)
    .entry_point(_start);
pub static mut SERIAL: Serial = Serial::COM1;
pub static mut MEMORY_MAP: [MemEntry; ENTRIES_COUNT] = [MemEntry::default(); ENTRIES_COUNT];

#[no_mangle]
extern "C" fn _start(stivale_struct: &'static StivaleStruct) -> ! {
    info!("Booting from limine...");
    x86_64::init_arch();
    
    init_memory_map(stivale_struct);
    init_pmm(unsafe { MEMORY_MAP });
    
    tests::run_tests();

    todo!();
}
