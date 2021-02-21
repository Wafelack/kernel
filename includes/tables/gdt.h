#ifndef _GDT_H_
#define _GDT_H_

#include "../utils/includes.h"

void
gdt_entry(uint8_t* target, uint32_t limit, uint32_t base, uint8_t type);

void
init_gdt(void);

#endif