#ifndef _UTILS_H_
#define _UTILS_H_

#define ASM_I(...) asm volatile(#__VA_ARGS__)
#define ENABLE_INTERRUPTS() asm volatile ("sti");
#define DISABLE_INTERRUPTS() asm volatile ("cli");

#define SET_COLOR(color) writer_setcolor(&writer, make_color((color), BLACK));

#define PANIC(comment) { \
    DISABLE_INTERRUPTS()\
    kprint("\n");\
    SET_COLOR(LIGHT_GRAY);\
    kprint("Kernel panicked at " __FILE__ ":");\
    char buffer[7] = {0};\
    itoa(buffer, __LINE__);\
    kprint(buffer);\
    kprint(": ");\
    kprint((comment));\
    kprint("\n");\
    while (1);\
}

#define assert(expr) {\
    if (!(expr)) {\
        PANIC("Assertion failed: `" #expr "`");\
    }\
}


#define OK(message) {\
  kprint("[");\
  SET_COLOR(GREEN);\
  kprint(" OK ");\
  SET_COLOR(LIGHT_GRAY);\
  kprint("] ");\
  kprint((message));\
  kprint("\n");\
}


#define SUCCESS_TEST(test_name) {\
  kprint("test::" test_name "... ");\
  SET_COLOR(GREEN);\
  kprint("ok\n");\
  SET_COLOR(LIGHT_GRAY);\
}

#include "includes.h"

#if 0
#define NULL ((voic*)0)
#endif

void
itoa(char* buffer, int32_t number);

void*
memset(void* dest, uint16_t ch, size_t count);
void*
memcpy(void* dest, const void* src, size_t count);
void*
memmove(void* dest, const void* src, size_t count);

#endif
