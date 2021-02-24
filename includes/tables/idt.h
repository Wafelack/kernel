#ifndef _IDT_H_
#define _IDT_H_

#include "utils/utils.h"

typedef struct {
    uint16_t offset0;
    uint16_t selector;
    uint8_t zero;
    uint8_t type_attr;
    uint16_t offset1;
} IDT_ENTRY;

#endif