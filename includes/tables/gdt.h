#ifndef _GDT_H_
#define _GDT_H_

#include "../utils/includes.h"

typedef struct __attribute__((__packed__)) {
    uint32_t limit;
    uint32_t base;
    uint8_t type;
} GDT_ENTRY;

void
gdt_entry(GDT_ENTRY* entry, uint32_t limit, uint32_t base, uint8_t type);

void
init_gdt(void);

#endif