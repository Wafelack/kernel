use crate::{serial, serialn, SERIAL};
use core::{fmt::Write, panic::PanicInfo};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let (Some(l), Some(m)) = (info.location(), info.message()) {
        serial!("Kernel panic - {}:{} - ", l.file(), l.line());
        unsafe {
            SERIAL.write_fmt(*m);
        }
        serialn!("");
    } else {
        serialn!("Kernel {}", info);
    }
    unsafe {
        asm!("cli"); // Clear interrupts
        asm!("hlt"); // Halt gracefully
    }
    loop {}
}
