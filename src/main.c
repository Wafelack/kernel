#include "drivers/vga.h"
#include "tables/gdt.h"
#include "utils/utils.h"

extern WRITER writer;

void
kernel_main(void) 
{
    uint8_t GDT[3][8] = {};

    gdt_entry(GDT[0], 0, 0, 0);
    gdt_entry(GDT[1], 0xffffffff, 0, 0x9A);
    gdt_entry(GDT[2], 0xffffffff, 0, 0x92);

    clear_screen(&writer);
    PRINT("Hello, World !\n");

    writer_setcolor(&writer, make_color(GREEN, BLACK));
    PRINT("Hello from the Matrix !");

    PANIC("This is a panic test.");

    while (1);
}