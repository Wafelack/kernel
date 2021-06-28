#![no_std]
#![no_main]
#![feature(llvm_asm)]

use core::panic::PanicInfo;

const COM1: u16 = 0x3F8;

#[no_mangle]
extern "C" fn k_main(_hdr_addr: usize) -> ! {
    loop {  }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop { }
}
