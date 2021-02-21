#include "tables/gdt.h"
#include "drivers/vga.h"
#include "utils/utils.h"

extern WRITER writer;
extern int set_gdt(void);

void
gdt_entry(GDT_ENTRY* entry, uint32_t limit, uint32_t base, uint8_t type)
{

    entry->limit = limit;
    entry->base = base;
    entry->type = type;
}

void
init_gdt(void)
{
    GDT_ENTRY GDT[3] = {};
    gdt_entry(&GDT[0], 0, 0, 0);
    gdt_entry(&GDT[1], 0xffffffff, 0, 0x9A);
    gdt_entry(&GDT[2], 0xffffffff, 0, 0x92);

    DISABLE_INTERRUPTS();

    set_gdt();

    ENABLE_INTERRUPTS();
}