#![no_std]
#![no_main]
#![feature(asm, panic_info_message, abi_x86_interrupt)]

mod log;
mod memory;
mod panic;
mod serial;
mod x86_64;

use memory::{pmm::{init_pmm, alloc_page, free_page}, MemEntry, ENTRIES_COUNT};
use serial::Serial;
use stivale_boot::v2::{StivaleHeader, StivaleStruct};

const STACK_SIZE: usize = 8192;
static STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

#[link_section = ".stivale2hdr"]
#[no_mangle]
#[used]
static STIVALE_HDR: StivaleHeader = StivaleHeader::new()
    .stack(&STACK[STACK_SIZE - 1] as *const u8)
    .entry_point(k_main);
pub static mut SERIAL: Serial = Serial::COM1;
pub static mut MEMORY_MAP: [MemEntry; ENTRIES_COUNT] = [MemEntry::default(); ENTRIES_COUNT];

#[no_mangle]
#[allow(unconditional_panic)]
extern "C" fn k_main(stivale_struct: &'static StivaleStruct) -> ! {
    info!("Booting from limine...");
    x86_64::init_arch();

    unsafe { asm!("int 0x0") };

    stivale_struct.memory_map()
                  .expect("Memory map is unavailable")
                  .iter()
                  .enumerate()
                  .for_each(|(idx, e)| unsafe { MEMORY_MAP[idx] = MemEntry::from_stivale_mem(*e) });
    init_pmm(unsafe { MEMORY_MAP });

    let page = alloc_page(1);
    if let Some(idx) = page {
        ok!("Successfully allocated a page at {:#08x}.", idx);
        free_page (idx, 1);
        assert_eq!(Some(idx), alloc_page(1));
        ok!("Successfully freed page at {:#08x}", idx);
        free_page (idx, 1);
    }
    todo!();
}
