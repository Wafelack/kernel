use core::mem::size_of;

#[repr(packed)]
pub struct Desc {
    size: u16,
    addr: u64,
}

#[repr(packed)]
#[derive(Copy, Clone)]
pub struct Seg {
    lim_low: u16,
    bas_low: u16,
    bas_mid: u16,
    flag: u8,
    lim_high_gran: u8, // gran << 4 | lim_high
    bas_high: u8,
}
impl Seg {
    pub const fn new() -> Self {
        Self {
            lim_low: 0,
            bas_low: 0,
            bas_mid: 0,
            flag: 0,
            lim_high_gran: 0,
            bas_high: 0,
        }
    }
    pub const fn with_flag(flag: u8, gran: u8) -> Self {
        let mut seg = Self::new();
        seg.flag = flag;
        seg.lim_high_gran = gran;
        return seg;
    }
}

#[link(name = "x86_64_arch")]
extern "C" {
       fn install_gdt(desc: *const Desc);
}

pub const ENTRIES: usize = 5;
const CODE_GRAN: u8 = 0b100000;
static mut GDT: [Seg; ENTRIES] = [Seg::new(); ENTRIES];
static mut PTR: Desc = Desc { size: 0, addr: 0 };

pub unsafe fn gdt() {
    GDT[1] = Seg::with_flag(0b10011010, CODE_GRAN); // Kernel code
    GDT[2] = Seg::with_flag(0b10010010, 0); // Kernel data
    GDT[3] = Seg::with_flag(0b11111010, CODE_GRAN); // User code
    GDT[4] = Seg::with_flag(0b11110010, 0); // User data
    PTR = Desc {
        size: (size_of::<[Seg; ENTRIES]>() - 1) as u16,
        addr: (&GDT as *const _) as u64,
    };
    install_gdt(&PTR as *const _);
}
