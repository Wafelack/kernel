#include "gdt.h"

void
encodeEntry(uint8_t* target, uint32_t limit, uint32_t base, uint8_t type)
{
    if ((limit > 65536) && ((limit & 0XFFF) != 0xFFF))
    {
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