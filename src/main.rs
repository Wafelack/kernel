#![no_std]
#![no_main]
#![feature(asm, panic_info_message)]

mod serial;
mod log;
mod gdt;

use core::{panic::PanicInfo, fmt::Write};
use serial::Serial;
use stivale::StivaleHeader;
use gdt::gdt;

static STACK: [u8; 4096] = [0; 4096];

#[link_section = ".stivale2hdr"]
#[no_mangle]
#[used]
static STIVALE_HDR: StivaleHeader = StivaleHeader::new(&STACK[4095] as *const u8);
pub static mut SERIAL: Serial = Serial::new();

#[no_mangle]
extern "C" fn k_main(_hdr_addr: usize) -> ! {
    info!("Booting from limine...");
    info!("Installing GDT...");
    unsafe { gdt() };
    ok!("Loaded GDT.");
    todo!();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // As info.location() and info.message() should not return None yet, they are not handled, and
    // the message is printed directly.
    if let Some(l) = info.location() {
        if let Some(m) = info.message() {
            serial!("\x1b[1;31merror\x1b[0m: {}:{}: Kernel panicked at: `", l.file(), l.line());
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
