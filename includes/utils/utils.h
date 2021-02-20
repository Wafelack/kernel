#ifndef _UTILS_H_
#define _UTILS_H_

#define ASM_I(...) asm volatile(#__VA_ARGS__)
#define ENABLE_INTERRUPTS() asm volatile ("sti");
#define DISABLE_INTERRUPTS() asm volatile ("cli");

#define PANIC(comment) { \
    DISABLE_INTERRUPTS()\
    PRINT("\n");\
    writer_setcolor(&writer, make_color(LIGHT_GRAY, BLACK));\
    PRINT(__FILE__);\
    PRINT(":");\
    char buffer[7] = {0};\
    itoa(buffer, __LINE__);\
    PRINT(buffer);\
    PRINT(": ");\
    PRINT(comment);\
    PRINT("\n");\
    while (1);\
}

#include "includes.h"

#if 0
#define NULL ((voic*)0)
#endif

void
itoa(char* buffer, int32_t number);

void*
memset(void* dest, int32_t ch, size_t count);
void*
memcpy(void* dest, const void* src, size_t count);
void*
memmove(void* dest, const void* src, size_t count);

#endif