#include "drivers/vga.h"
#include "tables/gdt.h"
#include "utils/utils.h"

extern WRITER writer;

static void
init_gdt(void);

void
kernel_main(void) 
{
    clear_screen(&writer);

    OK("Initialized VGA buffer.");

    init_gdt();

    OK("Initialized Global Descriptor Table.");

    assert(1 != 1, "This is an assertion that will fail.");

    while (1);
}

static void
init_gdt(void)
{
    uint8_t GDT[3][8] = {};
    gdt_entry(GDT[0], 0, 0, 0);
    gdt_entry(GDT[1], 0xffffffff, 0, 0x9A);
    gdt_entry(GDT[2], 0xffffffff, 0, 0x92);
}
