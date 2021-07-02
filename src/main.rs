#![no_std]
#![no_main]
#![feature(asm, panic_info_message, abi_x86_interrupt)]

mod gdt;
mod idt;
mod log;
mod serial;

use core::{fmt::Write, panic::PanicInfo};
use gdt::gdt;
use idt::idt;
use serial::Serial;
use stivale::StivaleHeader;

static STACK: [u8; 4096] = [0; 4096];

#[link_section = ".stivale2hdr"]
#[no_mangle]
#[used]
static STIVALE_HDR: StivaleHeader = StivaleHeader::new(&STACK[4095] as *const u8);
pub static mut SERIAL: Serial = Serial::COM1;

#[no_mangle]
#[allow(unconditional_panic)]
extern "C" fn k_main(_hdr_addr: usize) -> ! {
    info!("Booting from limine...");
    unsafe { gdt() };
    unsafe { idt() };
    serialn!("Division by zero result: {}.", 7 / 0);
    todo!();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // As info.location() and info.message() should not return None yet, they are not handled, and
    // the message is printed directly.
    if let Some(l) = info.location() {
        if let Some(m) = info.message() {
            serial!("Kernel panic - {}:{} - ", l.file(), l.line());
            unsafe {
                SERIAL.write_fmt(*m);
            }
            serialn!("");
            loop {}
        }
    }
    serialn!("Kernel {}", info);
    loop {}
}
