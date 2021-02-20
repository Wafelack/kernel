         bits 32
MBALIGN  equ 1 << 0            ; align loaded modules
MEMINFO  equ 1 << 1            ; memory map
FLAGS    equ MBALIGN | MEMINFO ; Multiboot flags
MAGIC    equ 0x1BADB002        ; Magic number (for the bootloader to find the header)
CHECKSUM equ -(MAGIC + FLAGS)  ; Checksum of flags and magic number (multiboot proof)

section .multiboot ; Multiboot header
align 4
    dd MAGIC
    dd FLAGS
    dd CHECKSUM

section .bss
align 16
stack_bottom:
resb 16384 ; 16 KiB
stack_top:

section .text
extern kernel_main
global _start:function (_start.end - _start)
_start:
       cli
       mov esp, stack_top ; initialize esp to top of stack
       
       call kernel_main

       
       
.hang: hlt
       jmp .hang
.end: