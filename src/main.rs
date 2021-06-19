#![no_std]
#![no_main]
#![feature(asm, panic_info_message, abi_x86_interrupt)]

mod log;
mod panic;
mod serial;
mod x86_64;

use core::{fmt::Write, panic::PanicInfo};
use serial::Serial;
use stivale::StivaleHeader;
use x86_64::init_arch;

const STACK_SIZE: usize = 4096;
static STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

#[link_section = ".stivale2hdr"]
#[no_mangle]
#[used]
static STIVALE_HDR: StivaleHeader = StivaleHeader::new(&STACK[STACK_SIZE - 1] as *const u8);
pub static mut SERIAL: Serial = Serial::COM1;

#[no_mangle]
extern "C" fn k_main(_hdr_addr: usize) -> ! {
    info!("Booting from limine...");
    x86_64::init_arch();
    todo!();
}
