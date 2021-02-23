#include "tables/gdt.h"
#include "drivers/vga.h"
#include "utils/utils.h"

extern WRITER writer;
extern int set_gdt(void);

void
gdt_entry(GDT_ENTRY* entry, uint32_t limit, uint32_t base, uint8_t flags, uint8_t access)
{
    uint16_t base0 = (uint16_t)(limit >> 16);
    uint16_t base_tmp = (uint16_t)(base & 0xFFFF);
    entry->base0 = base0;
    entry->base1 = (uint8_t)(base_tmp >> 8);
    entry->base2 = (uint8_t)(base_tmp & 0xFF);

    entry->limit0 = (uint16_t)(limit >> 16);
    uint16_t limit_tmp = (uint16_t)(limit & 0xFFFF);
    uint8_t limit1 = (uint8_t)((limit_tmp & 0xFF) >> 4);
    entry->limit_flags = limit1 << 4 | flags >> 4;
    entry->access = access;
}

void
init_gdt(GDT_ENTRY* entries)
{
    gdt_entry(&entries[0], 0, 0, 0, 0);
    gdt_entry(&entries[1], 0xffffffff, 0, 0, 0x9A); // Code
    gdt_entry(&entries[2], 0xffffffff, 0, 0, 0x92); // Data
    OK("Initialized Global Descriptor Table.");
}