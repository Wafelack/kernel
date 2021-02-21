#include "tables/gdt.h"
#include "drivers/vga.h"
#include "utils/utils.h"

extern WRITER writer;
extern int set_gdt(void);

void
gdt_entry(uint8_t* target, uint32_t limit, uint32_t base, uint8_t type)
{

    if ((limit > 65536) && ((limit & 0XFFF) != 0xFFF))
    {
        PANIC("Limit is too hight or invalid !");
        return;
    }

    if (limit > 65536) {
        // Adjust granularity
        limit >>= 12;
        target[6] = 0xC0;
    } else {
        target[6] = 0x40;
    }

    target[0] = limit & 0xFF;
    target[1] = (limit >> 8) & 0xFF;
    target[6] = (limit >> 16) & 0xF;

    target[2] = base & 0xFF;
    target[3] = (base >> 8) & 0xFF;
    target[4] = (base >> 16) & 0xFF;
    target[7] = (base >> 24) & 0xFF;

    target[5] = type;
}

void
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