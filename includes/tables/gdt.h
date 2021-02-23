#ifndef _GDT_H_
#define _GDT_H_

#include "../utils/includes.h"

typedef struct __attribute__((__packed__)) {
    uint16_t limit0;
    uint16_t base0;
    uint8_t base1;
    uint8_t access;
    uint8_t limit_flags;
    uint8_t base2;
} GDT_ENTRY;

void
gdt_entry(GDT_ENTRY* entry, uint32_t limit, uint32_t base, uint8_t flags, uint8_t access);

void
init_gdt(GDT_ENTRY* entries);

#endif