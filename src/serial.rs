use core::fmt;

pub struct Serial {
    port: u16,
}
impl Serial {
    pub const fn new() -> Self {
        Self {
            port: 0x3F8, /* COM1 */
        }
    }
    pub unsafe fn write_byte(&self, byte: u8) {
            asm!("out dx, al", in("dx") self.port, in("al") byte);
    }
}

impl fmt::Write for Serial {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.as_bytes() {
            unsafe {
                self.write_byte(*b)
            };
        }
        Ok(())
    }
}
#[macro_export]
macro_rules! serial {
    ($($arg:tt)*) => {
        unsafe {
            // Errors are not possible
            write!($crate::SERIAL, $($arg)*).unwrap();
        }
    }
}
#[macro_export]
macro_rules! serialn {
    ($($arg:tt)*) => {
        serial!($($arg)*);
        serial!("\n");
    }
}
