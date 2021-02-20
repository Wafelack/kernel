#ifndef _UTILS_H_
#define _UTILS_H_

#define ASM_I(...) asm volatile(#__VA_ARGS__)
#define ENABLE_INTERRUPTS() asm volatile ("sti");
#define DISABLE_INTERRUPTS() asm volatile ("cli");

#define SET_COLOR(color) writer_setcolor(&writer, make_color(color, BLACK));

#define PANIC(comment) { \
    DISABLE_INTERRUPTS()\
    PRINT("\n");\
    SET_COLOR(LIGHT_GRAY);\
    PRINT("Please keep kalm, don't ");\
    SET_COLOR(RED);\
    PRINT("PANIK !\n");\
    SET_COLOR(LIGHT_GRAY);\
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

#define assert(expr, message) {\
    if (!(expr))\
        PANIC("Assertion failed: `" #expr "`: " message);\
}


#define OK(message) {\
  PRINT("[");\
  SET_COLOR(GREEN);\
  PRINT(" OK ");\
  SET_COLOR(LIGHT_GRAY);\
  PRINT("] ");\
  PRINT(message);\
  PRINT("\n");\
}


#define SUCCESS_TEST(test_name) {\
  PRINT("test::" test_name "... ");\
  SET_COLOR(GREEN);\
  PRINT("ok\n");\
  SET_COLOR(LIGHT_GRAY);\
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
