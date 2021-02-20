#ifndef _GDT_H_
#define _GDT_H_

#include <stdint.h>

void
encodeEntry(uint8_t* target, uint32_t limit, uint32_t base, uint8_t type);

#endif