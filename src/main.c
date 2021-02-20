#include "include/vga.h"
#include "include/gdt.h"

void
kernel_main(void) 
{
    uint8_t null_descriptor[8] = {};
    uint8_t code_descriptor[8] = {};
    uint8_t data_descriptor[8] = {};
    encodeEntry(null_descriptor, 0, 0, 0);
    encodeEntry(code_descriptor, 0xffffffff, 0, 0x9A);
    encodeEntry(data_descriptor, 0xffffffff, 0, 0x92);
    
    uint8_t GDT[3][8] = {
        null_descriptor,
        code_descriptor,
        data_descriptor
    };

    WRITER writer = writer_init();
    vga_write(&writer, "Hello, World !\n");
}