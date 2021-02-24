#include "tables/gdt.h"
#include "drivers/vga.h"
#include "utils/utils.h"

extern WRITER writer;
extern int set_gdt(void);

void
gdt_entry(GDT_ENTRY* entry, uint32_t limit, uint32_t base, uint8_t gran, uint8_t access)
{
    entry->base0 = (uint16_t)(base & 0xFFFF);
    entry->base1 = (uint8_t)((base >> 16) & 0xFF);
    entry->base2 = (uint8_t)((base >> 24) & 0xFF);
    entry->limit0 = (uint16_t)(limit & 0xFFFF);
    entry->gran = (uint8_t)((limit & 0xFFFF) | (gran & 0xF0));
    entry->access = access;
}

void
init_gdt(GDT_ENTRY* entries)
{
    gdt_entry(&entries[0], 0, 0, 0, 0);
    gdt_entry(&entries[1], 0xffffffff, 0, 0xCF, 0x9A); // Code
    gdt_entry(&entries[2], 0xffffffff, 0, 0xCF, 0x92); // Data
    OK("Initialized Global Descriptor Table.");
}