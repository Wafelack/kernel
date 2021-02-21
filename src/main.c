#include "drivers/vga.h"
#include "tables/gdt.h"
#include "utils/utils.h"

extern WRITER writer;

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

    assert(1 != 1, "This assertion will fail !");

    #if 0
    for (uint8_t i = 0; i < 28; i++) {
      char buffer[7] = {};
      itoa(buffer, (int32_t)i);
      PRINT(buffer);
      PRINT("\n");
    }
    #endif
    
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

static void
tests(void)
{
    PRINT("\nRunning tests ...\n\n");

    // Memmove
    char dest[2] = {2, 3};
    char src[2]  = {5, 7};
    memmove(dest, src, 2);
    assert(dest[1] == 7 && dest[0] == 5, "Failed to move data.");

    SUCCESS_TEST("memmove");

    PRINT("\n"); 
    OK("Sucessfully passed all tests.");
}
