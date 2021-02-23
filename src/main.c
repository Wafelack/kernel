#include "drivers/vga.h"
#include "tables/gdt.h"
#include "utils/utils.h"

extern WRITER writer;

GDT_ENTRY entries[3] = {};

static void
tests(void);

void
kernel_main(void) 
{
    clear_screen(&writer);

    OK("Initialized VGA buffer.");

    init_gdt(entries);

    tests();
    

    while (1);
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
