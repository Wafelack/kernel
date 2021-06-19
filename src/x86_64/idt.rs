pub use crate::{err, info};
use core::{fmt::Write, mem::size_of};

#[repr(packed)]
pub struct IDTTable {
    size: u16,
    addr: u64,
}
#[repr(packed)]
#[derive(Copy, Clone)]
pub struct Entry {
    offset0: u16,
    code: u16,
    idx: u8,
    attr: u8,
    offset1: u16,
    offset2: u32,
    zero: u32,
}
impl Entry {
    pub const fn new() -> Self {
        Self {
            offset0: 0,
            code: 0,
            idx: 0,
            attr: 0,
            offset1: 0,
            offset2: 0,
            zero: 0,
        }
    }
    pub const fn from(off: u64, attr: u8) -> Self {
        Self {
            offset0: off as u16,
            code: KCODE,
            idx: IST_IDX,
            attr,
            offset1: (off >> 16) as u16,
            offset2: (off >> 32) as u32,
            zero: 0,
        }
    }
}
#[repr(u8)]
pub enum IType {
    TRAP16 = 0b0111,
    GATE16 = 0b0110,
    GATE32 = 0b1110,
    TRAP32 = 0b1111,
}

fn make_attr(itype: IType) -> u8 {
    (PRESENT as u8) << 7 | DPL << 5 | (itype as u8)
}

const ENTRIES: usize = 0x15;
const PRESENT: bool = true;
const IST_IDX: u8 = 0;
const KCODE: u16 = 0x8; // Kernel code selector in GDT
const DPL: u8 = 0;

static mut TABLE: IDTTable = IDTTable { size: 0, addr: 0 };
static mut ENT: [Entry; ENTRIES] = [Entry::new(); ENTRIES];

pub unsafe fn idt() {
    asm!("cli");

    TABLE = IDTTable {
        size: (size_of::<[Entry; ENTRIES]>() - 1) as u16,
        addr: (&ENT as *const _) as u64,
    };

    use interrupts::*;

    ENT[0] = Entry::from(dbz as u64, IType::TRAP32 as u8);
    ENT[1] = Entry::from(ssi as u64, IType::TRAP32 as u8);
    ENT[2] = Entry::from(nmi as u64, IType::TRAP32 as u8);
    ENT[3] = Entry::from(brk as u64, IType::TRAP32 as u8);
    ENT[4] = Entry::from(overflow as u64, IType::TRAP32 as u8);
    ENT[5] = Entry::from(bre as u64, IType::TRAP32 as u8);
    ENT[6] = Entry::from(invalid as u64, IType::TRAP32 as u8);
    ENT[7] = Entry::from(coprocessor_not_available as u64, IType::TRAP32 as u8);
    ENT[8] = Entry::from(double_fault as u64, IType::TRAP32 as u8);
    ENT[9] = Entry::from(coproc as u64, IType::TRAP32 as u8);
    ENT[10] = Entry::from(tss as u64, IType::TRAP32 as u8);
    ENT[11] = Entry::from(not_present as u64, IType::TRAP32 as u8);
    ENT[12] = Entry::from(stack_seg_fault as u64, IType::TRAP32 as u8);
    ENT[13] = Entry::from(gpf as u64, IType::TRAP32 as u8);
    ENT[14] = Entry::from(page_fault as u64, IType::TRAP32 as u8);
    ENT[15] = Entry::from(reserved as u64, IType::TRAP32 as u8);
    ENT[16] = Entry::from(fpe as u64, IType::TRAP32 as u8);
    ENT[17] = Entry::from(align_check as u64, IType::TRAP32 as u8);
    ENT[18] = Entry::from(machine_check as u64, IType::TRAP32 as u8);
    ENT[19] = Entry::from(simd as u64, IType::TRAP32 as u8);
    ENT[20] = Entry::from(virtu as u64, IType::TRAP32 as u8);

    asm!("lidt [rdi]", in("rdi")(&ENT as *const _));
    asm!("sti");
}

mod interrupts {
    use super::{err, info, Write};

    pub extern "x86-interrupt" fn dbz() {
        err!("Attempted to divide by zero.");
    }
    pub extern "x86-interrupt" fn ssi() {
        info!("Single step interrupt");
    }
    pub extern "x86-interrupt" fn nmi() {
        panic!("Non maskable interrupt");
    }
    pub extern "x86-interrupt" fn brk() {
        info!("Hit breakpoint");
    }
    pub extern "x86-interrupt" fn overflow() {
        panic!("Overflow");
    }
    pub extern "x86-interrupt" fn bre() {
        panic!("Bound range exceded");
    }
    pub extern "x86-interrupt" fn invalid() {
        panic!("Invalid instruction");
    }
    pub extern "x86-interrupt" fn coprocessor_not_available() {
        panic!("Coprocessor not available");
    }
    pub extern "x86-interrupt" fn double_fault() {
        panic!("Double fault");
    }
    pub extern "x86-interrupt" fn coproc() {
        panic!("Coprocessor segment overrun");
    }
    pub extern "x86-interrupt" fn tss() {
        panic!("Invalid task state segment");
    }
    pub extern "x86-interrupt" fn not_present() {
        panic!("Segment not present");
    }
    pub extern "x86-interrupt" fn stack_seg_fault() {
        panic!("Stack segment fault")
    }
    pub extern "x86-interrupt" fn gpf() {
        panic!("General Protection Fault");
    }
    pub extern "x86-interrupt" fn page_fault() {
        panic!("Page fault");
    }
    pub extern "x86-interrupt" fn reserved() {
        info!("Reserved interrupt");
    }
    pub extern "x86-interrupt" fn fpe() {
        panic!("Floating point exception");
    }
    pub extern "x86-interrupt" fn align_check() {
        panic!("Alignment check");
    }
    pub extern "x86-interrupt" fn machine_check() {
        panic!("Machine check");
    }
    pub extern "x86-interrupt" fn simd() {
        panic!("Simd floating point exception");
    }
    pub extern "x86-interrupt" fn virtu() {
        panic!("Virtualization exception");
    }
}
