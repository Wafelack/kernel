#include "drivers/vga.h"
#include "tables/gdt.h"
#include "utils/utils.h"

extern WRITER writer;
extern int set_gdt(void);

static void
init_gdt(void);

static void
tests(void);

void
kernel_main(void) 
{
    clear_screen(&writer);

    OK("Initialized VGA buffer.");

    init_gdt();

    OK("Initialized Global Descriptor Table.");

    tests();

    assert(1 != 1);

    while (1);
}


static void
init_gdt(void)
{
    uint8_t GDT[3][8] = {};
    gdt_entry(GDT[0], 0, 0, 0);
    gdt_entry(GDT[1], 0xffffffff, 0, 0x9A);
    gdt_entry(GDT[2], 0xffffffff, 0, 0x92);

    DISABLE_INTERRUPTS();

    set_gdt();

    ENABLE_INTERRUPTS();
}


static void
tests(void)
{
    kprint("\nRunning tests ...\n\n");

    // Memmove
    char dest[2] = {2, 3};
    char src[2]  = {5, 7};
    memmove(dest, src, 2);
    assert((dest[1] == 7 && dest[0] == 5));

    SUCCESS_TEST("memmove");

    kprint("\n"); 
    OK("Sucessfully passed all tests.");
}
