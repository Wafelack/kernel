#![no_std]
#![no_main]
#![feature(asm, panic_info_message)]

mod serial;
mod log;

use core::{panic::PanicInfo, fmt::Write};
use serial::Serial;
use stivale::StivaleHeader;

static STACK: [u8; 4096] = [0; 4096];

#[link_section = ".stivale2hdr"]
#[no_mangle]
#[used]
static STIVALE_HDR: StivaleHeader = StivaleHeader::new(&STACK[4095] as *const u8);
pub static mut SERIAL: Serial = Serial::new();

#[no_mangle]
extern "C" fn k_main(_hdr_addr: usize) -> ! {
    log!("Booting from limine...");
    todo!();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // As info.location() and info.message() should not return None yet, they are not handled, and
    // the message is printed directly.
    if let Some(l) = info.location() {
        if let Some(m) = info.message() {
            serial!("\x1b[1;31m{}:{}\x1b[0m Kernel panicked at: `", l.file(), l.line());
            unsafe {
                SERIAL.write_fmt(*m).unwrap();
            }
            serialn!("`.");
            loop {  }
        }
    }
    serialn!("Kernel {}", info);
    loop {  }
}
