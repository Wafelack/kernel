mod gdt;
mod idt;

use crate::{info, ok};
use core::fmt::Write;

pub fn init_arch() {
    info!("Installing GDT...");
    unsafe {
        gdt::gdt();
    }
    ok!("Installed GDT.");
    info!("Loading IDT...");
    unsafe {
        idt::idt();
    }
    ok!("Loaded IDT.");
    ok!(
        "Arch {} initialized.",
        module_path!().split("::").last().unwrap()
    );
}
