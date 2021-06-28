fn main() {
    nasm_rs::Build::new()
        .target("x86_64-none-none")
        .file("src/gdt.s")
        .compile("x86_64_arch")
        .expect("Failed to compile x86_64 ASM.");
}
