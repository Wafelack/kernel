use crate::{serial, serialn, SERIAL};
use core::{fmt::Write, panic::PanicInfo};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let (Some(l), Some(m)) = (info.location(), info.message()) {
        serial!("\x1b[0;31merror: \x1b[0;34m{}.{}:\x1b[0m Kernel panicked: ", l.file(), l.line());
        unsafe {
            SERIAL.write_fmt(*m).unwrap();
        }
        serialn!("");
    } else {
        serialn!("Kernel {}", info);
    }
    unsafe {
        asm!("cli"); // Clear interrupts
    }
    loop {
        unsafe { asm!("hlt") };
    }
}
