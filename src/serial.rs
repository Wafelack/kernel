use core::fmt;

#[repr(u16)]
#[derive(Copy, Clone)]
pub enum Serial {
    COM1 = 0x3F8,
    COM2 = 0x2F8,
    COM3 = 0x3E8,
    COM4 = 0x2E8,
}
impl Serial {
    pub unsafe fn write_byte(&self, byte: u8) {
        asm!("out dx, al", in("dx") *self as u16, in("al") byte);
    }
}

impl fmt::Write for Serial {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.as_bytes() {
            unsafe { self.write_byte(*b) };
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
        $crate::serial!($($arg)*);
        $crate::serial!("\n");
    }
}
